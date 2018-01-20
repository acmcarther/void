extern crate sdl2;
extern crate sdl2_sys;
extern crate x11_dl;

pub struct Sdl2Ctx {
  inner_ctx: sdl2::Sdl,
  video_subsystem: sdl2::VideoSubsystem,
  window: sdl2::video::Window,
}

pub fn vulkan_window() -> Sdl2Ctx {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
      .position_centered()
      .vulkan()
      .build()
      .unwrap();

  let wm_info = unsafe {
    let mut inner_wm_info = std::mem::uninitialized();
    sdl2_sys::SDL_GetWindowWMInfo(window.raw(), &mut inner_wm_info);
    inner_wm_info
  };

  Sdl2Ctx {
    inner_ctx: sdl_context,
    video_subsystem: video_subsystem,
    window: window,
  }
}
