extern crate protobuf;
extern crate state_api;
extern crate state_proto;

use protobuf::Message;
use state_api::ComponentTypeId;
use state_api::KeyFrameId;
use state_api::NodeId;
use state_api::StateBlob;
use state_proto::state::StateUpdate;
use std::collections::HashMap;

pub trait StateTransmitter {
  fn add_node(&mut self, node_id: NodeId);
  fn remove_node(&mut self, node_id: &NodeId);

  fn add_global_interest(&mut self, node_id: &NodeId, component_type_id: ComponentTypeId);
  fn remove_global_interest(&mut self, node_id: &NodeId, component_type_id: &ComponentTypeId);

  fn add_keyframe_id_ack(
    &mut self,
    node_id: &NodeId,
    component_type_id: &ComponentTypeId,
    keyframe_id: KeyFrameId,
  );

  fn produce_update(
    &mut self,
    node_id: &NodeId,
    state_blob: &StateBlob,
    current_time_s: f64,
  ) -> StateUpdate;
}
