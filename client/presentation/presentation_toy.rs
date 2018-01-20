extern crate vulkan;
extern crate sdl2_window;

fn main() {
  let window_ctx = sdl2_window::vulkan_window();
  vulkan::vulkan();

  loop {
  }
}
