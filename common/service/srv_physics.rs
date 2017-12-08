extern crate chrono;
#[macro_use] extern crate log;
extern crate service_spec;

use service_spec::NodeServiceMetadata;
use service_spec::NodeService;
use service_spec::TickContext;
use service_spec::RunError;

pub struct PhysicsService { }

impl NodeService for PhysicsService {
  fn metadata(&self) -> NodeServiceMetadata {
    NodeServiceMetadata
  }

  fn run_pre_tick(&mut self, tick: &Box<TickContext>) -> Result<(), RunError> {
    Ok(())
  }

  fn run_tick(&mut self, tick: &Box<TickContext>) -> Result<(), RunError> {
    Ok(())
  }

  fn run_post_tick(&mut self, tick: &Box<TickContext>) -> Result<(), RunError> {
    Ok(())
  }
}
