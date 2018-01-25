extern crate sdl2;
extern crate sdl2_sys;
extern crate vk_sys as vk;
extern crate vk_lite as vkl;

use std::ffi::CStr;

pub struct SdlWindowSystemPlugin<'a> {
  window: &'a mut sdl2::video::Window,
}

impl <'a> SdlWindowSystemPlugin<'a> {
  pub fn new(window: &mut sdl2::video::Window) -> SdlWindowSystemPlugin {
    SdlWindowSystemPlugin {
      window: window
    }
  }
}

impl <'a> vkl::WindowSystemPlugin for SdlWindowSystemPlugin<'a> {
  /**
   * Creates a crossplat vulkan surface object by extracting the underlying X11 display.
   *
   * This method will fail if the underlying window is not X11-backed.
   * This method disregards the lifetime of the SDL2 window.
   * TODO(acmcarther): fix ^
   */
  fn create_surface(&mut self, instance: &vkl::LInstance) -> vkl::RawResult<vk::SurfaceKHR> {
    unsafe {
      let mut sys_wm_info = sdl2_sys::SDL_SysWMinfo {
        version: sdl2_sys::SDL_version {
          major: 2,
          minor: 0,
          patch: 7,
        },
        subsystem: sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_UNKNOWN,
        info: std::mem::uninitialized(),
      };
      if sdl2_sys::SDL_GetWindowWMInfo(self.window.raw(), &mut sys_wm_info) != sdl2_sys::SDL_bool::SDL_TRUE {
        panic!("SDL2: {}", CStr::from_ptr(sdl2_sys::SDL_GetError()).to_str().unwrap());
      }
      assert_eq!(sys_wm_info.subsystem, sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_X11);

      let xlib_surface_create_info_khr = vk::XlibSurfaceCreateInfoKHR {
        sType: vk::STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
        pNext: std::ptr::null(),
        flags: 0,
        dpy: sys_wm_info.info.x11.display as *mut std::os::raw::c_void,
        window: sys_wm_info.info.x11.window,
      };

      instance.create_xlib_surface_khr(&xlib_surface_create_info_khr)
    }
  }
}
