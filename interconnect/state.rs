extern crate protobuf;
extern crate state_proto;

use protobuf::Message;
use state_proto::state::ComponentType;
use state_proto::state::ComponentUpdates;
use std::collections::HashMap;

pub type NodeId = u64;
pub type KeyFrameId = u32;
pub type EntityId = u32;

// TODO(acmcarther): Move this elsewhere
pub trait StateBlob {
  fn get_global_components_for_type(
    &self,
    component_type: &ComponentType,
  ) -> Vec<(EntityId, &Message)>;
}
