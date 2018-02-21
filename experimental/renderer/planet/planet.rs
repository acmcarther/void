extern crate cgmath;
extern crate geometry;
extern crate icosphere;
extern crate init;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate planet_renderer;
extern crate renderer;
extern crate sdl2;
extern crate sdl2_vulkan_interop;
extern crate vk_lite as vkl;
#[macro_use]
extern crate zcfg;

use cgmath::Matrix4;
use geometry::Mesh;
use planet_renderer::PlanetRenderer;
use renderer::BaseRenderer;
use renderer::BaseRendererConfig;
use sdl2::Sdl;
use sdl2::video::Window;
use sdl2_vulkan_interop::SdlWindowSystemPlugin;
use std::marker::PhantomData;

fn main() {
  init::init();

  let planet = IcoPlanet::new(6 /* iterations */);
  let mut game_window = GameWindow::new();
  let planet_renderer = {
    let vulkan = vkl::Vulkan::new("libvulkan.so.1");
    let base_renderer = BaseRenderer::new(
      vulkan,
      &mut game_window.window_system_plugin(),
      BaseRendererConfig {
        extension_spec: x11_extension_spec(),
        layer_spec: x11_layer_spec(),
      },
    );
    PlanetRenderer::new(base_renderer)
  };

  loop {}
}

struct GameWindow {
  sdl: Sdl,
  window: Window,
}

struct IcoPlanet {
  pub mesh: Mesh,
  pub model: Matrix4<f32>,
}

impl GameWindow {
  pub fn new() -> GameWindow {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
      .window("planet demo", 1920, 1080)
      .position_centered()
      .vulkan()
      .build()
      .unwrap();

    GameWindow {
      sdl: sdl,
      window: window,
    }
  }

  pub fn window_system_plugin<'window>(&'window mut self) -> SdlWindowSystemPlugin<'window> {
    // TODO(acmcarther): This seems unpleasant and brittle-ish.
    SdlWindowSystemPlugin::new(&mut self.window)
  }
}

impl IcoPlanet {
  pub fn new(iterations: u32) -> IcoPlanet {
    IcoPlanet {
      mesh: icosphere::icosphere(iterations),
      model: Matrix4::<f32>::from_scale(1.0),
    }
  }
}

/** Dumps hardcoded x11-related extensions into a FeatureSpec */
fn x11_extension_spec() -> vkl::FeatureSpec {
  vkl::FeatureSpec {
    wanted: vec![
      "VK_EXT_acquire_xlib_display",
      //"VK_EXT_display_surface_counter",
      "VK_KHR_display",
      "VK_KHR_get_physical_device_properties2",
      "VK_KHR_get_surface_capabilities2",
      "VK_KHR_surface",
      //"VK_KHR_xcb_surface",
      "VK_KHR_xlib_surface",
      "VK_KHX_device_group_creation",
    ],
    required: vec!["VK_EXT_debug_report"],
  }
}

/** Dumps hardcoded x11-related layers into a FeatureSpec */
fn x11_layer_spec() -> vkl::FeatureSpec {
  vkl::FeatureSpec {
    wanted: vec![
      "VK_LAYER_LUNARG_core_validation",
      "VK_LAYER_LUNARG_parameter_validation",
    ],
    required: vec!["VK_LAYER_LUNARG_standard_validation"],
  }
}
