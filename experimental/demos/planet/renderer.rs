extern crate sdl2;
extern crate sdl2_vulkan_interop;
extern crate vk_lite as vkl;
extern crate vk_renderer;

use vk_renderer::BaseRenderer;
use vk_renderer::BaseRendererConfig;
use sdl2::video::Window;
use sdl2_vulkan_interop::SdlWindowSystemPlugin;
use vkl::Vulkan;

pub struct Renderer<'window> {
  base_renderer: BaseRenderer<'window>,
}

impl<'window> Renderer<'window> {
  pub fn new(base_renderer: BaseRenderer) -> Renderer {
    Renderer {
      base_renderer: base_renderer,
    }
  }

  pub fn from_sdl_window(sdl_window: &mut Window) -> Renderer {
    let base_renderer = BaseRenderer::new(
      Vulkan::new("libvulkan.so.1"),
      &mut SdlWindowSystemPlugin::new(sdl_window),
      BaseRendererConfig {
        extension_spec: x11_extension_spec(),
        layer_spec: x11_layer_spec(),
      },
    );
    Renderer::new(base_renderer)
  }
}

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
