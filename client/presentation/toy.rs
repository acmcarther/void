extern crate vulkan;
extern crate sdl2;
extern crate sdl2_vulkan_interop;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let mut window = video_subsystem.window("rust-sdl2 demo", 800, 600)
      .position_centered()
      .vulkan()
      .build()
      .unwrap();
  let mut vk_ctx = {
    let mut sdl_window_system_plugin = sdl2_vulkan_interop::SdlWindowSystemPlugin::new(&mut window);
    vulkan::vulkan(&mut sdl_window_system_plugin)
  };

  loop {
  }
}
