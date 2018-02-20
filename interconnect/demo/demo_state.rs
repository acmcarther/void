extern crate demo_proto;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate state_api;

use demo_proto::demo::ColorComponent;
use demo_proto::demo::PositionComponent;
use protobuf::Message;
use state_api::ComponentTypeId;
use state_api::EntityId;
use state_api::StateBlob;
use state_api::UpdateErr;
use std::collections::HashMap;
use std::collections::HashSet;

pub const POSITION_COMPONENT_ID: u32 = 1;
pub const COLOR_COMPONENT_ID: u32 = 2;

#[derive(Debug)]
pub struct State {
  entities: HashSet<EntityId>,
  position_components: HashMap<EntityId, PositionComponent>,
  color_components: HashMap<EntityId, ColorComponent>,
}

impl State {
  pub fn new() -> State {
    State {
      entities: HashSet::new(),
      position_components: HashMap::new(),
      color_components: HashMap::new(),
    }
  }

  pub fn update_color(&mut self, entity_id: EntityId, data: ColorComponent) {
    self.color_components.insert(entity_id, data);
  }

  pub fn update_position(&mut self, entity_id: EntityId, data: PositionComponent) {
    self.position_components.insert(entity_id, data);
  }

  fn update_color_raw(&mut self, entity_id: &EntityId, data: &[u8]) -> Result<(), UpdateErr> {
    let color_data = try!(
      protobuf::parse_from_bytes::<ColorComponent>(data)
        .map_err(|_| UpdateErr::IncompatiblePayload)
    );

    self.update_color(*entity_id, color_data);
    Ok(())
  }

  fn update_position_raw(&mut self, entity_id: &EntityId, data: &[u8]) -> Result<(), UpdateErr> {
    let position_data = try!(
      protobuf::parse_from_bytes::<PositionComponent>(data)
        .map_err(|_| UpdateErr::IncompatiblePayload)
    );

    self.update_position(*entity_id, position_data);
    Ok(())
  }
}

impl StateBlob for State {
  fn add_entity(&mut self, entity_id: EntityId) {
    self.entities.insert(entity_id);
  }

  fn remove_entity(&mut self, entity_id: EntityId) {
    self.entities.remove(&entity_id);
  }

  fn list_entities(&self) -> Vec<EntityId> {
    self.entities.iter().cloned().collect()
  }

  fn get_entities_with_component(&self, component_type_id: &ComponentTypeId) -> HashSet<EntityId> {
    match *component_type_id {
      POSITION_COMPONENT_ID => self.position_components.keys().cloned().collect(),
      COLOR_COMPONENT_ID => self.color_components.keys().cloned().collect(),
      _ => {
        warn!("Unknown component_type_id requested: {}", component_type_id);
        HashSet::new()
      },
    }
  }

  fn get_global_components_for_type(
    &self,
    component_type_id: &ComponentTypeId,
  ) -> Vec<(EntityId, &Message)> {
    match *component_type_id {
      POSITION_COMPONENT_ID => self
        .position_components
        .iter()
        .map(|(k, v)| (k.clone(), v as &protobuf::Message))
        .collect(),
      COLOR_COMPONENT_ID => self
        .color_components
        .iter()
        .map(|(k, v)| (k.clone(), v as &protobuf::Message))
        .collect(),
      _ => {
        warn!("Unknown component_type_id requested: {}", component_type_id);
        Vec::new()
      },
    }
  }

  fn update_component(
    &mut self,
    entity_id: &EntityId,
    component_type_id: &ComponentTypeId,
    data: &[u8],
  ) -> Result<(), UpdateErr> {
    match *component_type_id {
      POSITION_COMPONENT_ID => self.update_position_raw(entity_id, data),
      COLOR_COMPONENT_ID => self.update_color_raw(entity_id, data),
      _ => {
        warn!("Unknown component_type_id update: {}", component_type_id);
        Err(UpdateErr::UnknownComponentTypeId)
      },
    }
  }

  fn remove_component(&mut self, entity_id: &EntityId, component_type_id: &ComponentTypeId) {
    match *component_type_id {
      POSITION_COMPONENT_ID => {
        self.position_components.remove(entity_id);
      },
      COLOR_COMPONENT_ID => {
        self.color_components.remove(entity_id);
      },
      _ => {
        warn!("Unknown component_type_id remove: {}", component_type_id);
      },
    }
  }
}
