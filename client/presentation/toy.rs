extern crate chrono;
extern crate fern;
extern crate log;
extern crate sdl2;
extern crate sdl2_vulkan_interop;
extern crate vk_lite as vkl;
extern crate vk_new_triangle as vkt;

fn main() {
  fern::Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message
        ))
    })
    // Add blanket level filter -
    .level(log::LogLevelFilter::Debug)
    // - and per-module overrides
    .chain(std::io::stdout())
    // Apply globally
    .apply()
    .unwrap();


  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let mut window = video_subsystem
    .window("rust-sdl2 demo", 800, 600)
    .position_centered()
    .vulkan()
    .build()
    .unwrap();
  // TODO(acmcarther): This seems unpleasant and brittle-ish.
  let mut sdl_window_system_plugin = sdl2_vulkan_interop::SdlWindowSystemPlugin::new(&mut window);
  let vulkan = vkl::Vulkan::new("libvulkan.so.1");
  let vulkan_triangle = vkt::vulkan_triangle(&vulkan, &mut sdl_window_system_plugin);

  let mut event_pump = sdl_context.event_pump().unwrap();

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. }
        | sdl2::event::Event::KeyDown {
          keycode: Some(sdl2::keyboard::Keycode::Escape),
          ..
        } => break 'running,
        _ => {},
      }
    }
    vulkan_triangle.draw_demo_frame();
  }
}
