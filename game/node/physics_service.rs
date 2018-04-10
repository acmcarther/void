extern crate chrono;
extern crate log;
extern crate node;
extern crate physics_proto;

use std::mem;
use std::iter::Iterator;
use std::slice::Iter;
use std::slice::IterMut;
use node::EntityId;
use node::NodeService;
use node::RunError;
use node::TickContext;
use node::NodeServiceMetadata;
use physics_proto::physics::PhysicsInfo;

struct PhysicsServiceParams {}

struct PhysicsService {
  params: PhysicsServiceParams,
}

impl PhysicsService {
  fn new(params: PhysicsServiceParams) -> PhysicsService {
    PhysicsService { params: params }
  }
}

impl NodeService for PhysicsService {
  fn metadata(&self) -> NodeServiceMetadata {
    NodeServiceMetadata
  }

  fn run_tick(&mut self, _tick: &TickContext) -> Result<(), RunError> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create() {
    PhysicsService::new(PhysicsServiceParams {});
  }
}
