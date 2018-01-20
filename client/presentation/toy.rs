extern crate vulkan;
extern crate sdl2_window;

fn main() {
  let mut window_ctx = sdl2_window::make_context();
  let mut vk_ctx = vulkan::vulkan(|i, p| window_ctx.create_surface(i, p));

  loop {
  }
}
