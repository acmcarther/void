extern crate protobuf;
extern crate state_api;
extern crate state_proto;

use protobuf::Message;
use state_api::ComponentAck;
use state_api::ComponentTypeId;
use state_api::KeyFrameId;
use state_api::NodeId;
use state_api::StateBlob;
use state_proto::state::ComponentUpdates;
use state_proto::state::StateUpdate;
use state_proto::state::StateUpdateResponse;
use std::collections::HashMap;

pub trait StateAcceptor {
  fn produce_update_response(&mut self) -> StateUpdateResponse;

  fn integrate_update(
    &mut self,
    state_blob: &mut StateBlob,
    update: StateUpdate,
    current_time_s: f64,
  );
}
