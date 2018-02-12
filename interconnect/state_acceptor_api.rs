extern crate protobuf;
extern crate state;
extern crate state_proto;

use protobuf::Message;
use state::KeyFrameId;
use state::NodeId;
use state::StateBlob;
use state_proto::state::ComponentType;
use state_proto::state::ComponentUpdates;
use std::collections::HashMap;

pub trait StateAcceptor {
  fn integrate_update<T: StateBlob>(
    &mut self,
    state_blob: &mut T,
    update: Vec<(ComponentType, ComponentUpdates)>;
    current_time_s: f64,
  );
}
