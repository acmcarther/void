extern crate sdl2;
extern crate sdl2_sys;
extern crate vk;
extern crate vk_sys;

use vk::lite;
use std::ffi::CStr;
use sdl2::video::Window;
use sdl2_sys::SDL_bool::SDL_TRUE;
use sdl2_sys::SDL_version;
use sdl2_sys::SDL_SysWMinfo;
use sdl2_sys::SDL_bool::SDL_FALSE;

pub struct SdlWindowSystemPlugin<'window> {
  window: &'window mut Window,
}

impl<'window> SdlWindowSystemPlugin<'window> {
  pub fn new(window: &mut Window) -> SdlWindowSystemPlugin {
    SdlWindowSystemPlugin { window: window }
  }
}

/**
 * Yields the static SDL_version to build against
 *
 * TODO: this needs to live in a sdl2 specific helper lib 
 */
fn sdl2_version() -> SDL_version {
  sdl2_sys::SDL_version {
    major: 2,
    minor: 0,
    patch: 7,
  }
}

/** Makes the surface_khr corresponding to this X11 window */
pub fn make_sdl2_x11_surface_khr(
    instance: &lite::LInstance,
    window: &mut Window) -> lite::RawResult<vk_sys::SurfaceKHR> {
  let wm_info = make_sdl2_x11_wminfo(window);

  make_x11_surface_khr(instance, &wm_info)
}

/** Makes the SDL2_SysWMInfo corresponding to this X11 window */
fn make_sdl2_x11_wminfo(window: &mut Window) -> SDL_SysWMinfo {
  let mut sys_wm_info = unsafe {
    let mut sys_wm_info = SDL_SysWMinfo {
      version: sdl2_version(),
      subsystem: sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_UNKNOWN,
      info: std::mem::uninitialized(),
    };

    let did_get_info =
      SDL_TRUE == sdl2_sys::SDL_GetWindowWMInfo(window.raw(), &mut sys_wm_info);

    if !did_get_info {
      panic!(
        "SDL2 Failed to get WM Info: {}",
        CStr::from_ptr(sdl2_sys::SDL_GetError())
            .to_str().unwrap());
    }

    sys_wm_info
  };

  assert_eq!(sys_wm_info.subsystem, sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_X11);

  sys_wm_info
}

fn make_x11_surface_khr(instance: &lite::LInstance,
                        sys_wm_info: &SDL_SysWMinfo)
    -> lite::RawResult<vk_sys::SurfaceKHR> {
  // If this is not an X11 SDL sys_wm_info, then another function needs to be used
  assert_eq!(sys_wm_info.subsystem, sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_X11);

  // UNSAFE: Access to known union fields (from above assert)
  let xlib_surface_create_info_khr = unsafe {
    vk_sys::XlibSurfaceCreateInfoKHR {
      sType: vk_sys::STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
      pNext: std::ptr::null(),
      flags: 0,
      dpy: sys_wm_info.info.x11.display as *mut std::os::raw::c_void,
      window: sys_wm_info.info.x11.window,
    }
  };

  instance.create_xlib_surface_khr(&xlib_surface_create_info_khr)
}

impl<'window> lite::WindowSystemPlugin<'window> for SdlWindowSystemPlugin<'window> {
  /**
   * Creates a crossplat vulkan surface object by extracting the underlying X11 display.
   *
   * This method will fail if the underlying window is not X11-backed.
   * This method disregards the lifetime of the SDL2 window.
   * TODO(acmcarther): fix ^
   */
  unsafe fn create_surface(&mut self, instance: &lite::LInstance) -> lite::RawResult<vk_sys::SurfaceKHR> {
    let mut sys_wm_info = sdl2_sys::SDL_SysWMinfo {
      version: sdl2_sys::SDL_version {
        major: 2,
        minor: 0,
        patch: 7,
      },
      subsystem: sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_UNKNOWN,
      info: std::mem::uninitialized(),
    };
    if sdl2_sys::SDL_GetWindowWMInfo(self.window.raw(), &mut sys_wm_info)
      != sdl2_sys::SDL_bool::SDL_TRUE
    {
      panic!(
        "SDL2: {}",
        CStr::from_ptr(sdl2_sys::SDL_GetError()).to_str().unwrap()
      );
    }
    assert_eq!(
      sys_wm_info.subsystem,
      sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_X11
    );

    let xlib_surface_create_info_khr = vk_sys::XlibSurfaceCreateInfoKHR {
      sType: vk_sys::STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
      pNext: std::ptr::null(),
      flags: 0,
      dpy: sys_wm_info.info.x11.display as *mut std::os::raw::c_void,
      window: sys_wm_info.info.x11.window,
    };

    instance.create_xlib_surface_khr(&xlib_surface_create_info_khr)
  }
}
