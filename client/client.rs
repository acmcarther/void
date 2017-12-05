#[macro_use]
extern crate log;
#[macro_use]
extern crate winit;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_win;

use std::default::Default;
use std::ptr;
use std::ffi::{CStr, CString};
use std::ops::Drop;

struct Sys {
  pub events_loop: winit::EventsLoop,
  pub window: winit::Window,
}

pub fn run() {
  let sys = init();
  start_main_loop(sys);
}

/** Initializes globals for main loop. */
fn init() -> Sys {
  let events_loop = winit::EventsLoop::new();
  let window = winit::WindowBuilder::new()
              .with_title("Void")
              .with_dimensions(1920, 1080)
              .build(&events_loop)
              .unwrap();
  Sys {
    events_loop: events_loop,
    window: window
  }
}

/** Runs the game application in an infinite loop. */
fn start_main_loop(sys: Sys) {
  let b = true;

  while b {
    std::thread::sleep(std::time::Duration::new(1, 0))
  }
}
