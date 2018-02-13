extern crate protobuf;
extern crate state;
extern crate state_proto;

use protobuf::Message;
use state::KeyFrameId;
use state::NodeId;
use state::StateBlob;
use state_proto::state::ComponentType;
use state_proto::state::StateUpdate;
use std::collections::HashMap;

pub trait StateTransmitter {
  fn add_node(&mut self, node_id: NodeId);
  fn remove_node(&mut self, node_id: &NodeId);

  fn add_global_interest(&mut self, node_id: &NodeId, component_type: ComponentType);
  fn remove_global_interest(&mut self, node_id: &NodeId, component_type: &ComponentType);

  fn add_keyframe_id_ack(
    &mut self,
    node_id: &NodeId,
    component_type: &ComponentType,
    keyframe_id: KeyFrameId,
  );

  fn produce_update<T: StateBlob>(
    &mut self,
    node_id: &NodeId,
    state_blob: &T,
    current_time_s: f64,
  ) -> StateUpdate;
}
