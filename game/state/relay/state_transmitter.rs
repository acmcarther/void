#[macro_use]
extern crate log;
extern crate protobuf;
extern crate state_api;
extern crate state_proto;
extern crate state_transmitter_api;

use state_api::ComponentTypeId;
use state_api::KeyFrameId;
use state_api::NodeId;
use state_api::StateBlob;
use state_proto::state::ComponentUpdates;
use state_proto::state::KeyComponentState;
use state_proto::state::KeyFrameContent;
use state_proto::state::StateUpdate;
use state_transmitter_api::StateTransmitter;
use std::collections::HashMap;

pub struct StateTransmitterConfig {
  pub component_configs: HashMap<ComponentTypeId, ComponentConfig>,
}

pub struct ComponentConfig {
  pub s_between_frames: f64,
  pub delta_frames_per_keyframe: u32,
}

pub struct Interests {
  pub global_component_interests: Vec<ComponentTypeId>,
}

pub struct StateTransmitterImpl {
  config: StateTransmitterConfig,
  node_details: HashMap<NodeId, NodeDetails>,
}

pub struct NodeDetails {
  node_id: NodeId,
  component_details: HashMap<ComponentTypeId, ComponentDetails>,
  interests: Interests,
}

pub struct ComponentDetails {
  last_keyframe_id: KeyFrameId,
  last_keyframe_time_s: f64,
  last_keyframe_id_acked: KeyFrameId,
}

impl Default for StateTransmitterConfig {
  fn default() -> StateTransmitterConfig {
    StateTransmitterConfig {
      component_configs: HashMap::new(),
    }
  }
}

impl StateTransmitterImpl {
  pub fn from_config(config: StateTransmitterConfig) -> StateTransmitterImpl {
    StateTransmitterImpl {
      config: config,
      node_details: HashMap::new(),
    }
  }
}

impl StateTransmitter for StateTransmitterImpl {
  fn add_node(&mut self, node_id: NodeId) {
    if self.node_details.contains_key(&node_id) {
      warn!("Tried to add node {} that already existed.", node_id);
      return;
    }

    let details = NodeDetails {
      node_id: node_id,
      component_details: HashMap::new(),
      interests: Interests {
        global_component_interests: Vec::new(),
      },
    };

    self.node_details.insert(node_id, details);
  }

  fn remove_node(&mut self, node_id: &NodeId) {
    if !self.node_details.contains_key(node_id) {
      warn!("Tried to remove node {} that didn't exist.", node_id);
      return;
    }

    self.node_details.remove(node_id);
  }

  fn add_global_interest(&mut self, node_id: &NodeId, component_type_id: ComponentTypeId) {
    if !self.node_details.contains_key(node_id) {
      warn!(
        "Tried to add global interest in {:?} for node {} but it didn't exist",
        component_type_id, node_id
      );
      return;
    }

    let node_details = self.node_details.get_mut(node_id).unwrap();
    let interest_already_exists = node_details
      .interests
      .global_component_interests
      .iter()
      .any(|c| *c == component_type_id);

    if interest_already_exists {
      warn!(
        "Tried to add global interest in {:?} for node {} but it was already present",
        component_type_id, node_id
      );
      return;
    }

    node_details
      .interests
      .global_component_interests
      .push(component_type_id);
    node_details.component_details.insert(
      component_type_id,
      ComponentDetails {
        last_keyframe_id: 0,
        // N.B: This is technically not correct, but whatever.
        // It'll trigger a keyframe on next update, which is probably the right thing to do.
        last_keyframe_time_s: 0.0,
        last_keyframe_id_acked: 0,
      },
    );
  }

  fn remove_global_interest(&mut self, node_id: &NodeId, component_type_id: &ComponentTypeId) {
    if !self.node_details.contains_key(node_id) {
      warn!(
        "Tried to remove global interest in {:?} for node {} but it didn't exist",
        component_type_id, node_id
      );
      return;
    }

    let node_details = self.node_details.get_mut(node_id).unwrap();
    let interest_position = node_details
      .interests
      .global_component_interests
      .iter()
      .position(|c| c == component_type_id);

    if interest_position.is_none() {
      warn!(
        "Tried to remove global interest in {:?} for node {} but it wasn't present",
        component_type_id, node_id
      );
      return;
    }

    node_details
      .interests
      .global_component_interests
      .remove(interest_position.unwrap());
    // N.B: component_details is retained.
    // This prevents potential weirdness with keyframe id that could occur if we remove and add a
    // global_interest quickly.
  }

  fn add_keyframe_id_ack(
    &mut self,
    node_id: &NodeId,
    component_type_id: &ComponentTypeId,
    keyframe_id: KeyFrameId,
  ) {
    let node_details_opt = self.node_details.get_mut(node_id);
    if node_details_opt.is_none() {
      warn!("Tried to add a keyframe id ack for an unknown node.");
      return;
    }
    let node_details = node_details_opt.unwrap();

    let component_details_opt = node_details.component_details.get_mut(component_type_id);
    if component_details_opt.is_none() {
      warn!("Tried to add a keyframe id ack for a known node without a component details entry.");
      return;
    }
    let component_details = component_details_opt.unwrap();

    component_details.last_keyframe_id_acked = keyframe_id;
  }

  fn produce_update(
    &mut self,
    node_id: &NodeId,
    state_blob: &StateBlob,
    current_time_s: f64,
  ) -> StateUpdate {
    if !self.node_details.contains_key(node_id) {
      warn!(
        "Tried to get update for node {} but it wasn't known.",
        node_id
      );
      return StateUpdate::new();
    }

    let node_details = self.node_details.get_mut(node_id).unwrap();

    let mut state_update = StateUpdate::new();

    // Update components
    {
      let component_updates_list = state_update.mut_component_updates();
      for interest in node_details.interests.global_component_interests.iter() {
        let mut component_details = node_details.component_details.get_mut(interest).unwrap();

        let s_between_frames = self
          .config
          .component_configs
          .get(interest)
          .map(|component_config| component_config.s_between_frames)
          .unwrap_or(0.1);

        // Bail this call if we haven't waited long enough for a new keyframe
        // TODO(acmcarther): Implement delta frames
        if current_time_s < component_details.last_keyframe_time_s + s_between_frames {
          continue;
        }

        let this_keyframe_id = component_details.last_keyframe_id + 1;
        let mut component_updates = ComponentUpdates::new();
        component_updates.set_keyframe_id(this_keyframe_id);
        component_updates.set_component_type_id(interest.clone());
        let mut keyframe_content = KeyFrameContent::new();
        {
          let mut key_component_states = keyframe_content.mut_key_component_states();
          for (entity_id, component_message) in state_blob.get_global_components_for_type(&interest)
          {
            let mut key_component_state = KeyComponentState::new();
            key_component_state.set_entity_id(entity_id);
            key_component_state.set_data(component_message.write_to_bytes().unwrap());

            key_component_states.push(key_component_state);
          }
        }
        component_updates.set_key_frame_content(keyframe_content);
        component_updates_list.push(component_updates);
        component_details.last_keyframe_id = this_keyframe_id;
        component_details.last_keyframe_time_s = current_time_s;
      }
    }

    state_update
  }
}
