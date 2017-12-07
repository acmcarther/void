extern crate chrono;
#[macro_use] extern crate log;
extern crate service_specification;

use chrono::Duration;
use chrono::DateTime;
use chrono::Utc;
use service_specification::TickContext;
use service_specification::NodeService;
use service_specification::RunError;

use std::collections::HashMap;

pub fn run() {
  let mut node = ServerNode::new();
  node.init();

  loop {
    node.tick().unwrap()
  }
}

struct BasicTickContext {
  current_tick: u64,
  last_creation_time: DateTime<Utc>,
  creation_time: DateTime<Utc>,
}

impl TickContext for BasicTickContext {
  fn tick_id(&self) -> u64 {
    self.current_tick
  }

  fn next(&self) -> Box<TickContext> {
    Box::new(BasicTickContext {
      current_tick: self.current_tick + 1,
      last_creation_time: self.creation_time,
      creation_time: Utc::now(),
    })
  }

  // TODO(acmcarther): Cache this
  fn delta_t(&self) -> Duration {
    self.creation_time.signed_duration_since(self.last_creation_time)
  }
}

/** The configuration for a single ServerNode. */
struct NodeConfig {
  services: Vec<String>,
}

/** An array of running services and associated state. */
struct NodeServiceManager {
  running_services: Vec<Box<NodeService>>
}

/** A game backend installation. */
struct ServerNode {
  config: NodeConfig,
  service_builders: HashMap<String, fn(config: &NodeConfig)->Box<NodeService>>,
  service_manager: Option<NodeServiceManager>,
  tick_context: Box<TickContext>,
}

impl ServerNode {
  pub fn new() -> ServerNode {
    ServerNode {
      config: NodeConfig {
        services: Vec::new(),
      },
      service_builders: HashMap::new(),
      service_manager: None,
      tick_context: Box::new(BasicTickContext {
        current_tick: 0u64,
        last_creation_time: Utc::now(),
        creation_time: Utc::now(),
      })
    }
  }

  /** Configures the node for running by starting all services. */
  pub fn init(&mut self) {
    assert!(self.service_manager.is_none());

    let mut services = Vec::new();

    info!("Starting all services!");
    for service_name in self.config.services.iter() {
      debug!("Starting service: {}", service_name);
      let builder = self.service_builders.get(service_name).unwrap();
      let mut service = builder(&self.config);
      service.on_include(&self.tick_context);
      services.push(service);
    }

    self.service_manager = Some(NodeServiceManager {
      running_services: services,
    });
  }

  /**
   * Advances the state of the simulation by one "tick".
   *
   * A tick proceeds through the service lifecycle for each service.
   */
  pub fn tick(&mut self) -> Result<(), RunError> {
    assert!(self.service_manager.is_some());

    self.tick_context = self.tick_context.next();

    let manager = self.service_manager.as_mut().unwrap();

    trace!("Running service pre-ticks");
    for service in manager.running_services.iter_mut() {
      service.run_pre_tick(&self.tick_context).unwrap();
    }

    trace!("Running service ticks");
    for service in manager.running_services.iter_mut() {
      // TODO(acmcarther): Use specs to parallelize this
      service.run_tick(&self.tick_context).unwrap();
    }

    trace!("Running service post-ticks");
    for service in manager.running_services.iter_mut() {
      service.run_post_tick(&self.tick_context).unwrap();
    }

    Ok(())
  }
}


#[cfg(test)]
mod tests {
  mod server_node {
    #[test]
    fn test_init_works() {
      let node = ServerNode::new();
      node::init();
    }

    #[test]
    #[should_panic]
    fn test_double_init_fails() {
      let node = ServerNode::new();
      node::init();
      node::init();
    }
  }
}
