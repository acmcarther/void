extern crate sdl2;

use sdl2::Sdl;
use sdl2::video::Window;

pub struct GameWindow {
  pub sdl: Sdl,
  pub window: Window,
}

impl GameWindow {
  pub fn new() -> GameWindow {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
      .window("planet demo", 1920, 1080)
      .position_centered()
      .vulkan()
      .build()
      .unwrap();

    sdl.mouse().show_cursor(false);
    sdl.mouse().set_relative_mouse_mode(true);

    GameWindow {
      sdl: sdl,
      window: window,
    }
  }
}
