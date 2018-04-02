#![feature(used)]
extern crate init;
#[macro_use]
extern crate log;
extern crate netcode_io_sys as nio;
extern crate reliable_io_sys as rio;

use init::LogLevelParsable;

/**
 * Initializes global state for the network.
 * !!! REQUIRED !!! to be called before starting a low level client or server.
 */
pub unsafe fn init_network() {
  let LogLevelParsable(log_level) = init::log_level::CONFIG.get_value();
  let nio_log_level = util::find_nio_log_level(log_level);
  let rio_log_level = util::find_rio_log_level(log_level);

  if nio::netcode_init() != (nio::NETCODE_OK as i32) {
    error!("Could not initialize netcode_io");
    panic!("Could not initialize netcode_io");
  }

  if rio::reliable_init() != (rio::RELIABLE_OK as i32) {
    error!("Could not initialize reliable_io");
    panic!("Could not initialize reliable_io");
  }

  nio::netcode_log_level(nio_log_level as i32);
  rio::reliable_log_level(rio_log_level as i32);
}

/**
 * Destroys global state for the network.
 * !!! REQUIRED !!! to be called before ending the application.
 */
pub unsafe fn term_network() {
  rio::reliable_term();
  nio::netcode_term();
}

mod util {
  use log::LogLevelFilter;
  use nio;
  use rio;
  use std::os::raw::c_uint;

  pub fn find_nio_log_level(log_level_filter: LogLevelFilter) -> c_uint {
    match log_level_filter {
      LogLevelFilter::Off => nio::NETCODE_LOG_LEVEL_NONE,
      LogLevelFilter::Error | LogLevelFilter::Warn => nio::NETCODE_LOG_LEVEL_ERROR,
      LogLevelFilter::Info => nio::NETCODE_LOG_LEVEL_INFO,
      LogLevelFilter::Debug | LogLevelFilter::Trace => nio::NETCODE_LOG_LEVEL_DEBUG,
    }
  }

  pub fn find_rio_log_level(log_level_filter: LogLevelFilter) -> c_uint {
    match log_level_filter {
      LogLevelFilter::Off => rio::RELIABLE_LOG_LEVEL_NONE,
      LogLevelFilter::Error | LogLevelFilter::Warn => rio::RELIABLE_LOG_LEVEL_ERROR,
      LogLevelFilter::Info => rio::RELIABLE_LOG_LEVEL_INFO,
      LogLevelFilter::Debug | LogLevelFilter::Trace => rio::RELIABLE_LOG_LEVEL_DEBUG,
    }
  }
}
