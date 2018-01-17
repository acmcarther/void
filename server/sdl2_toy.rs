extern crate sdl2;
extern crate vk_sys as vk;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::ffi::CString;
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

  vulkan();

  let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
      .position_centered()
      .vulkan()
      .build()
      .unwrap();

  println!("3");

  let mut canvas = window.into_canvas().build().unwrap();

  canvas.set_draw_color(Color::RGB(0, 255, 255));
  canvas.clear();
  canvas.present();
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut i = 0;
  'running: loop {
      i = (i + 1) % 255;
      canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
      canvas.clear();
      for event in event_pump.poll_iter() {
          match event {
              Event::Quit {..} |
              Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                  break 'running
              },
              _ => {}
          }
      }
      // The rest of the game loop goes here...

      canvas.present();
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}

fn vulkan() {
  let vk_application_info = vk::ApplicationInfo {
    sType: vk::STRUCTURE_TYPE_APPLICATION_INFO,
    pNext: ptr::null(),
    pApplicationName: CString::new("sdl2_toy").unwrap().as_ptr(),
    applicationVersion: 1,
    pEngineName: CString::new("No Engine").unwrap().as_ptr(),
    engineVersion: 1,
    apiVersion: 0
  };

  let vk_instance_create_info = vk::InstanceCreateInfo {
    sType: vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
    pApplicationInfo: &vk_application_info as *const _,
    flags: 0,
    pNext: ptr::null(),
    enabledLayerCount: 0,
    ppEnabledLayerNames: ptr::null(),
    enabledExtensionCount: 0,
    ppEnabledExtensionNames: ptr::null(),
  };
}
