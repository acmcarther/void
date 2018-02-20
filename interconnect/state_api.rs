extern crate protobuf;
extern crate state_proto;

use protobuf::Message;
use state_proto::state::ComponentUpdates;
use std::collections::HashMap;
use std::collections::HashSet;

pub type ComponentTypeId = u32;
pub type NodeId = u64;
pub type KeyFrameId = u32;
pub type EntityId = u64;

pub struct ComponentAck {
  pub component_type_id: ComponentTypeId,
  pub keyframe_id: KeyFrameId,
}

pub enum UpdateErr {
  UnknownEntity,
  UnknownComponentTypeId,
  IncompatiblePayload,
}

// TODO(acmcarther): Move this elsewhere
pub trait StateBlob {
  fn list_entities(&self) -> Vec<EntityId>;

  fn get_entities_with_component(&self, component_type_id: &ComponentTypeId) -> HashSet<EntityId>;

  fn get_global_components_for_type(
    &self,
    component_type_id: &ComponentTypeId,
  ) -> Vec<(EntityId, &Message)>;

  fn update_component(
    &mut self,
    entity_id: &EntityId,
    component_type_id: &ComponentTypeId,
    data: &[u8],
  ) -> Result<(), UpdateErr>;

  fn remove_component(&mut self, entity_id: &EntityId, component_type_id: &ComponentTypeId);
}
