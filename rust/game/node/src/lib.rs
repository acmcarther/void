extern crate chrono;
#[macro_use]
extern crate log;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::RwLock;

pub type EntityId = u32;
pub type ServiceBuilderFn<T> = fn(config: &NodeConfig) -> Box<NodeService<T>>;

/**
 * An error emitted by a service during execution.
 *
 * - Retriable errors should be retried by the server using the same service instance.
 * - Severe errors should be retried by creating a new service instance.
 * - Fatal errors should cause the node to fail.
 */
#[derive(Debug)]
pub enum RunError {
  ErrorRetriable,
  ErrorSevere,
  ErrorFatal,
}

/** The configuration for a single ServerNode. */
pub struct NodeConfig {
  pub services: Vec<String>,
}

/** An array of running services and associated state. */
pub struct NodeServiceManager<T> {
  running_services: Vec<Box<NodeService<T>>>,
}

/** A game backend installation. */
pub struct ServerNode<T> {
  config: NodeConfig,
  state_store: T,
  service_builders: HashMap<String, fn(config: &NodeConfig) -> Box<NodeService<T>>>,
  service_manager: Option<NodeServiceManager<T>>,
  tick_context: TickContext,
}

pub struct DenseComponentStore<T> {
  pub component_data: Vec<T>,
}

pub struct SparseComponentStore<T> {
  component_data: HashMap<EntityId, T>,
}

/** Details about the service (useful for planning, orchestration, etc). */
pub struct NodeServiceMetadata;

pub struct TickContext {
  pub current_tick: u64,
  pub last_creation_time: DateTime<Utc>,
  pub creation_time: DateTime<Utc>,
}

/**
 * A synchronized, individually threaded engine component.
 */
pub trait NodeService<T> {
  /** Emits common metadata for this node. */
  fn metadata(&self) -> NodeServiceMetadata;

  /**
   * A system callback called when the system is added to the node.
   */
  fn on_include(&mut self, _last_tick: &TickContext) {}

  /**
   * A system callback called at the beginning of the tick.
   *
   * This is a good time for systems to receive pubsub messages from last tick.
   * Pre-tick work happens strictly sequentially (i.e. single threaded), so
   * heavy-weight work should wait until `run_tick`.
   */
  fn run_pre_tick(&mut self, _state: &T, _tick: &TickContext) -> Result<(), RunError> {
    Ok(())
  }

  /**
   * A system callback called to perform the work of a given tick.
   *
   * Systems should focus on their core business logic at this time, including
   * processing received pubsub messages.
   */
  fn run_tick(&mut self, _state: &T, _tick: &TickContext) -> Result<(), RunError>;

  /**
   * A system callback called to perform post-tick cleanup.
   *
   * Systems should perform any necessary cleanup or late pubsub message
   * handling. Most systems do not need to implement this method, and should
   * prefer performing processing in `run_tick`, as this method is strictly
   * invoked sequentially according to system priority.
   */
  fn run_post_tick(&mut self, _state: &T, _tick: &TickContext) -> Result<(), RunError> {
    Ok(())
  }

  /**
   * A system callback called when the system is removed.
   *
   * In a single node installation, this may never get called.
   */
  fn on_remove(&mut self, _last_tick: &TickContext) {}
}

impl<T> ServerNode<T> {
  pub fn new(
    config: NodeConfig,
    service_builders: HashMap<String, ServiceBuilderFn<T>>,
    initial_state: T,
  ) -> ServerNode<T> {
    ServerNode {
      config: config,
      state_store: initial_state,
      service_builders: service_builders,
      service_manager: None,
      tick_context: TickContext {
        current_tick: 0u64,
        last_creation_time: Utc::now(),
        creation_time: Utc::now(),
      },
    }
  }
}

impl<T> ServerNode<T> {
  /** Configures the node for running by starting all services. */
  pub fn init(&mut self) {
    assert!(self.service_manager.is_none());

    let mut services = Vec::new();

    info!("Starting all services!");
    for service_name in self.config.services.iter() {
      debug!("Starting service: {}", service_name);
      let builder = self.service_builders.get(service_name).expect(&format!(
        "could not find service builder for {}",
        service_name
      ));
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
      service
        .run_pre_tick(&self.state_store, &self.tick_context)
        .unwrap();
    }

    trace!("Running service ticks");
    for service in manager.running_services.iter_mut() {
      service
        .run_tick(&self.state_store, &self.tick_context)
        .unwrap();
    }

    trace!("Running service post-ticks");
    for service in manager.running_services.iter_mut() {
      service
        .run_post_tick(&self.state_store, &self.tick_context)
        .unwrap();
    }

    Ok(())
  }
}

impl TickContext {
  pub fn next(&self) -> Self {
    TickContext {
      current_tick: self.current_tick + 1,
      last_creation_time: self.creation_time,
      creation_time: Utc::now(),
    }
  }

  // TODO(acmcarther): Cache this
  pub fn delta_t(&self) -> Duration {
    self
      .creation_time
      .signed_duration_since(self.last_creation_time)
  }
}

impl<T> DenseComponentStore<T> {
  pub fn new() -> DenseComponentStore<T> {
    DenseComponentStore {
      component_data: Vec::new(),
    }
  }
}

impl<T> SparseComponentStore<T> {
  pub fn new() -> SparseComponentStore<T> {
    SparseComponentStore {
      component_data: HashMap::new(),
    }
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
