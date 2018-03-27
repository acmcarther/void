extern crate base_server;
#[macro_use]
extern crate log;
extern crate physics_proto;
extern crate stores;

use base_server::NodeConfig;
use base_server::NodeService;
use base_server::NodeServiceMetadata;
use base_server::RunError;
use base_server::TickContext;
// TODO(acmcarther): This is becoming a single point of recompilation
use stores::StateStores;

pub struct SimplePhysics;


impl SimplePhysics {
  pub fn new(_: &NodeConfig) -> Box<NodeService<StateStores>> {
    Box::new(SimplePhysics)
  }
}

impl NodeService<StateStores> for SimplePhysics {
  fn metadata(&self) -> NodeServiceMetadata {
    NodeServiceMetadata
  }

  fn run_tick(&mut self, state: &StateStores, tick: &TickContext) -> Result<(), RunError> {
    debug!("Running tick for SimplePhysics");
    let delta_t = tick.delta_t();
    // This unwrap is never going to fail
    let dt_seconds = delta_t.num_microseconds().unwrap() as f32 / 1_000_000f32;

    // TODO(acmcarther): This basically eats the error
    let mut physics_store = try!(
      state
        .physics_store
        .write()
        .map_err(|_| RunError::ErrorFatal)
    );

    // TODO(acmcarther): This basically eats the error
    let mut impulse_store = try!(
      state
        .impulse_store
        .write()
        .map_err(|_| RunError::ErrorFatal)
    );

    for entity in physics_store.component_data.iter_mut() {
      if entity.get_is_static() {
        continue;
      }

      let (dx, dy, dz) = {
        let v = entity.get_velocity();
        (
          v.get_x() * dt_seconds,
          v.get_y() * dt_seconds,
          v.get_z() * dt_seconds,
        )
      };
      {
        let mut pos = entity.mut_position();

        pos.x = pos.x + dx;
        pos.y = pos.y + dy;
        pos.z = pos.z + dz;
      }
    }

    Ok(())
  }
}
