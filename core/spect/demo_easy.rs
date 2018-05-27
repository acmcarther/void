#![feature(used)]
extern crate easy_spect;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate zcfg;

use std::thread;
use std::time::Duration;

define_pub_cfg!(
  demo_sleep_time_ms,
  u64,
  20_000u64,
  "How long the main thread should sleep before initiating closing application."
);

fn main() {
  easy_spect::start(Vec::new() /* extra_modules */);

  thread::sleep(Duration::from_millis(
    demo_sleep_time_ms::CONFIG.get_value(),
  ))

  // It takes Spect some time to pick up on main thread closure...
}
