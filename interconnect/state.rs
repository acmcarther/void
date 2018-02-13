extern crate protobuf;
extern crate state_proto;

use protobuf::Message;
use state_proto::state::ComponentType;
use state_proto::state::ComponentUpdates;
use std::collections::HashMap;

pub type NodeId = u64;
pub type KeyFrameId = u32;
pub type EntityId = u32;

pub struct ComponentAck {
  pub component_type: ComponentType,
  pub keyframe_id: KeyFrameId,
}

pub enum UpdateErr {
  UnknownEntity,
  UnknownComponentType,
  IncompatiblePayload,
}

// TODO(acmcarther): Move this elsewhere
pub trait StateBlob {
  fn add_entity(&mut self, entity_id: EntityId);
  fn remove_entity(&mut self, entity_id: EntityId);
  fn list_entities(&self) -> Vec<EntityId>;

  fn get_global_components_for_type(
    &self,
    component_type: &ComponentType,
  ) -> Vec<(EntityId, &Message)>;

  fn update_component(
    &mut self,
    entity_id: &EntityId,
    component_type: &ComponentType,
    data: Vec<u8>,
  ) -> Result<(), UpdateErr>;
}
