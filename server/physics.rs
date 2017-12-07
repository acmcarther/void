extern crate chrono;
#[macro_use] extern crate log;
extern crate service_specification;

use service_specification::NodeServiceMetadata;
use service_specification::NodeService;
use service_specification::TickContext;
use service_specification::RunError;

pub struct PhysicsService { }

impl NodeService for PhysicsService {
  fn metadata(&self) -> NodeServiceMetadata {
    NodeServiceMetadata
  }

  fn run_tick(&mut self, tick: &Box<TickContext>) -> Result<(), RunError> {
    Ok(())
  }
}
