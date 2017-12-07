extern crate chrono;
#[macro_use] extern crate log;

use chrono::Duration;
use chrono::DateTime;
use chrono::Utc;

use std::collections::HashMap;

/** Details about the service (useful for planning, orchestration, etc). */
pub struct NodeServiceMetadata;

/** The state for a single tick. */
pub trait TickContext {
  /** The unique, monotonically increasing tick identifier. */
  fn tick_id(&self) -> u64;

  /** The time between this tick and the last one. */
  fn delta_t(&self) -> Duration;

  /** A new tick based on this tick. */
  fn next(&self) -> Box<TickContext>;
}

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

/**
 * A synchronized, individually threaded engine component.
 */
pub trait NodeService {
  /** Emits common metadata for this node. */
  fn metadata(&self) -> NodeServiceMetadata;

  /**
   * A system callback called when the system is added to the node.
   */
  fn on_include(&mut self, last_tick: &Box<TickContext>) {}

  /**
   * A system callback called at the beginning of the tick.
   *
   * This is a good time for systems to receive pubsub messages from last tick.
   * Pre-tick work happens strictly sequentially (i.e. single threaded), so
   * heavy-weight work should wait until `run_tick`.
   *
   * TODO(acmcarther): Yield result (which will let service be restarted)
   */
  fn run_pre_tick(&mut self, tick: &Box<TickContext>) -> Result<(), RunError> {
    Ok(())
  }

  /**
   * A system callback called to perform the work of a given tick.
   *
   * Systems should focus on their core business logic at this time, including
   * processing received pubsub messages.
   *
   * TODO(acmcarther): Yield result (which will let service be restarted)
   */
  fn run_tick(&mut self, tick: &Box<TickContext>) -> Result<(), RunError>;

  /**
   * A system callback called to perform post-tick cleanup.
   *
   * Systems should perform any necessary cleanup or late pubsub message
   * handling. Most systems do not need to implement this method, and should
   * prefer performing processing in `run_tick`, as this method is strictly
   * invoked sequentially according to system priority.
   *
   * TODO(acmcarther): Yield result (which will let service be restarted)
   */
  fn run_post_tick(&mut self, tick: &Box<TickContext>) -> Result<(), RunError> {
    Ok(())
  }

  /**
   * A system callback called when the system is removed.
   *
   * In a single node installation, this may never get called.
   */
  fn on_remove(&mut self, last_tick: &Box<TickContext>) {}
}
