extern crate vk_lite as vkl;
extern crate vk_triangle as vkt;
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
  // TODO(acmcarther): This seems unpleasant and brittle-ish.
  let vert_shader_bytes = include_bytes!("../../bazel-genfiles/client/presentation/toy_vert_shader.spv");
  let frag_shader_bytes = include_bytes!("../../bazel-genfiles/client/presentation/toy_frag_shader.spv");
  let mut sdl_window_system_plugin = sdl2_vulkan_interop::SdlWindowSystemPlugin::new(&mut window);
  let vulkan = vkl::Vulkan::new("libvulkan.so.1");
  let vulkan_triangle = vkt::vulkan(&vulkan, &mut sdl_window_system_plugin, vert_shader_bytes, frag_shader_bytes);

  let mut event_pump = sdl_context.event_pump().unwrap();

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
          break 'running
        },
        _ => {},
      }
    }
    vkt::draw_demo_frame(&vulkan_triangle)
  }
}
