extern crate sdl2;
extern crate vulkan;
extern crate vk_sys as vk;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::ffi::CString;
use std::ffi::CStr;
use std::path::Path;
use std::path::PathBuf;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

fn main() {
  println!("1");
  let sdl_context = sdl2::init().unwrap();
  println!("{:?}", sdl2::video::drivers().collect::<Vec<_>>());
  println!("{:?}", sdl2::audio::drivers().collect::<Vec<_>>());
  println!("{:?}", sdl2::render::drivers().collect::<Vec<_>>());
  println!("2");
  let video_subsystem = sdl_context.video().unwrap();
  println!("3");

  vulkan::vulkan();

  let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
      .position_centered()
      .vulkan()
      .build()
      .unwrap();

}
