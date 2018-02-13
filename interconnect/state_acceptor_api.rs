extern crate protobuf;
extern crate state;
extern crate state_proto;

use protobuf::Message;
use state::ComponentAck;
use state::KeyFrameId;
use state::NodeId;
use state::StateBlob;
use state_proto::state::ComponentType;
use state_proto::state::ComponentUpdates;
use state_proto::state::StateUpdate;
use std::collections::HashMap;

pub trait StateAcceptor {
  fn take_keyframe_id_acks(&mut self) -> Vec<ComponentAck>;

  fn integrate_update<T: StateBlob>(
    &mut self,
    state_blob: &mut T,
    update: StateUpdate,
    current_time_s: f64,
  );
}
