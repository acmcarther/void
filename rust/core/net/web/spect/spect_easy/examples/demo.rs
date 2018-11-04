#![feature(used)]
extern crate spect_easy;
extern crate base;
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
  base::init();
  spect_easy::start(Vec::new() /* extra_modules */);

  thread::sleep(Duration::from_millis(
    demo_sleep_time_ms::CONFIG.get_value(),
  ))
}
