extern crate init;
#[macro_use]
extern crate lazy_static;
extern crate x11_dl;
#[macro_use]
extern crate zcfg;

use x11_dl::xlib;
use x11_dl::xlib::Xlib;
use std::ptr;

fn main() {
  init::init();

  let xlib = xlib::Xlib::open().unwrap();
  unsafe {
    let display = (xlib.XOpenDisplay)(ptr::null());

    if display == ptr::null_mut() {
      panic!("Could not open display");
    }
  }
}
