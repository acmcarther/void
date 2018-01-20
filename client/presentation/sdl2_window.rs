extern crate sdl2;
extern crate sdl2_sys;
extern crate vk_sys as vk;

use std::ffi::CStr;

pub struct Sdl2Ctx {
  inner_ctx: sdl2::Sdl,
  video_subsystem: sdl2::VideoSubsystem,
  window: sdl2::video::Window,
}

impl Sdl2Ctx {
  pub fn create_surface(&mut self, instance: vk::Instance, instance_ptrs: &vk::InstancePointers) -> vk::SurfaceKHR {
    // TODO(acmcarther): Revisit this method. It is incredibly unsafe and broken on any platform
    // not using XLib.
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

      let mut xlib_surface_create_info_khr = vk::XlibSurfaceCreateInfoKHR {
        sType: vk::STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
        pNext: std::ptr::null(),
        flags: 0,
        dpy: sys_wm_info.info.x11.display as *mut std::os::raw::c_void,
        window: sys_wm_info.info.x11.window,
      };

      let mut surface_khr = std::mem::uninitialized();
      let result = instance_ptrs.CreateXlibSurfaceKHR(
        instance,
        &xlib_surface_create_info_khr,
        std::ptr::null(),
        &mut surface_khr
      );

      if result != vk::SUCCESS {
        panic!("failed to create surface with raw {}", result as i32);
      }

      surface_khr
    }
  }
}

pub fn make_context() -> Sdl2Ctx {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
      .position_centered()
      .vulkan()
      .build()
      .unwrap();

  Sdl2Ctx {
    inner_ctx: sdl_context,
    video_subsystem: video_subsystem,
    window: window,
  }
}
