#[macro_use]
extern crate log;
extern crate protobuf;
extern crate state_acceptor_api;
extern crate state_api;
extern crate state_proto;

use state_acceptor_api::StateAcceptor;
use state_api::ComponentTypeId;
use state_api::KeyFrameId;
use state_api::StateBlob;
use state_proto::state::ComponentAck;
use state_proto::state::StateUpdate;
use state_proto::state::StateUpdateResponse;
use std::collections::HashMap;

pub struct StateAcceptorConfig {}

pub struct StateAcceptorImpl {
  config: StateAcceptorConfig,
  pending_component_acks: HashMap<ComponentTypeId, KeyFrameId>,
}

impl Default for StateAcceptorConfig {
  fn default() -> StateAcceptorConfig {
    StateAcceptorConfig {}
  }
}

impl StateAcceptorImpl {
  pub fn from_config(config: StateAcceptorConfig) -> StateAcceptorImpl {
    StateAcceptorImpl {
      config: config,
      pending_component_acks: HashMap::new(),
    }
  }
}

impl StateAcceptor for StateAcceptorImpl {
  fn produce_update_response(&mut self) -> StateUpdateResponse {
    let mut state_update_response = StateUpdateResponse::new();

    {
      let component_ack_list = state_update_response.mut_component_acks();
      for (component_type_id, keyframe_id) in self.pending_component_acks.iter() {
        let mut component_ack = ComponentAck::new();
        component_ack.set_keyframe_id(*keyframe_id);
        component_ack.set_component_type_id(*component_type_id);
        component_ack_list.push(component_ack);
      }
    }

    state_update_response
  }

  fn integrate_update(
    &mut self,
    state_blob: &mut StateBlob,
    mut update: StateUpdate,
    current_time_s: f64,
  ) {
    let component_updates_list = update.take_component_updates();
    for mut component_updates in component_updates_list.into_iter() {
      let keyframe_id = component_updates.get_keyframe_id();
      let component_type_id = component_updates.get_component_type_id();

      self
        .pending_component_acks
        .insert(component_type_id, keyframe_id);

      if component_updates.has_delta_frame_content() {
        warn!("Delta frames are not currently supported in state acceptor, dropping payload");
        continue;
      }

      if component_updates.has_key_frame_content() {
        let mut known_entities_with_component =
          state_blob.get_entities_with_component(&component_type_id);

        let mut key_frame_content = component_updates.take_key_frame_content();
        let mut key_component_states = key_frame_content.take_key_component_states();

        for key_component_state in key_component_states.into_iter() {
          let entity_id = key_component_state.get_entity_id();
          known_entities_with_component.remove(&entity_id);

          state_blob.update_component(
            &entity_id,
            &component_type_id,
            key_component_state.get_data(),
          );
        }

        for removed_component_entity in known_entities_with_component.into_iter() {
          state_blob.remove_component(&removed_component_entity, &component_type_id)
        }
      }
    }
  }
}
