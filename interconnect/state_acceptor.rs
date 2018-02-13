#[macro_use]
extern crate log;
extern crate protobuf;
extern crate state;
extern crate state_acceptor_api;
extern crate state_proto;

use protobuf::Message;
use state::ComponentAck;
use state::KeyFrameId;
use state::NodeId;
use state::StateBlob;
use state_acceptor_api::StateAcceptor;
use state_proto::state::ComponentType;
use state_proto::state::ComponentUpdates;
use state_proto::state::EntityUpdate;
use state_proto::state::StateUpdate;
use std::collections::HashMap;

pub struct StateAcceptorConfig {}

pub struct StateAcceptorImpl {
  pending_component_acks: HashMap<ComponentType, KeyFrameId>,
}

impl StateAcceptorImpl {
  fn update_entities(&mut self, entity_update_list: Vec<EntityUpdate>) {
    // TODO(acmcarther): Implement
  }

  fn update_components(&mut self, component_updates_list: Vec<ComponentUpdates>) {
    // TODO(acmcarther): Implement
  }
}

impl StateAcceptor for StateAcceptorImpl {
  fn take_keyframe_id_acks(&mut self) -> Vec<ComponentAck> {
    self
      .pending_component_acks
      .drain()
      .map(|(component_type, keyframe_id)| ComponentAck {
        component_type: component_type,
        keyframe_id: keyframe_id,
      })
      .collect()
  }

  fn integrate_update<T: StateBlob>(
    &mut self,
    state_blob: &mut T,
    update: StateUpdate,
    current_time_s: f64,
  ) {
    // TODO(acmcarther): Implement
  }
}
