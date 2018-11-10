#![feature(used)]
#[macro_use]
extern crate log;
extern crate base;
extern crate sdl2_vk;
extern crate vk;
extern crate sdl2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate zcfg;

use vk::lite;
use vk::lite::Vulkan as VulkanAPI;
use std::sync::Mutex;
use std::sync::Arc;
use sdl2::Sdl as RawSdl;
use sdl2::video::Window as RawWindow;

mod flags {
  define_pub_cfg!(
    window_h_size,
    u32,
    720u32,
    "The height of the window"
  );

  define_pub_cfg!(
    window_w_size,
    u32,
    1280u32,
    "The width of the window"
  );
}

#[derive(Clone)]
struct Sdl2 {
  pub sdl2_ctx: RawSdl,
}

struct Window {
  sdl2: Arc<Mutex<Sdl2>>,
  pub raw_window: RawWindow,
}

fn main() {
  base::init();

  real_main();
}

fn real_main() {
  let sdl2 = make_sdl2();
  let mut window = make_window(sdl2.clone());
  let vk_api = VulkanAPI::default();
  let vk_instance = make_vk_instance(
    &vk_api,
    make_x11_extension_spec(),
    make_x11_layer_spec()).unwrap();

  let debug_report_callback = lite::builtins::make_debug_report_callback(
    &vk_instance,
    lite::builtins::vk_debug_report_callback_ext,
  ).unwrap();

  let surface_khr = sdl2_vk::make_sdl2_x11_surface_khr(
    &vk_instance,
    &mut window.raw_window
  ).unwrap();


}

fn make_sdl2() -> Arc<Mutex<Sdl2>> {
  // Is this ok: ????
  // What happens if this fails??? When would it fail?
  let sdl2_ctx = sdl2::init().unwrap();
  Arc::new(Mutex::new(Sdl2 {
    sdl2_ctx: sdl2_ctx,
  }))
}

fn make_window(sdl2: Arc<Mutex<Sdl2>>) -> Window {
  let video_subsystem = sdl2
    .lock()
    .unwrap()
    .sdl2_ctx
    .video()
    .unwrap();
  let window = video_subsystem
    .window(
      "rust-sdl2 demo",
      flags::window_w_size::CONFIG.get_value(),
      flags::window_h_size::CONFIG.get_value())
    .position_centered()
    .vulkan()
    .build()
    .unwrap();

  Window {
    sdl2: sdl2,
    raw_window: window
  }
}

fn make_vk_instance(
  vk_api: &VulkanAPI,
  extension_spec: lite::FeatureSpec,
  layer_spec: lite::FeatureSpec) -> lite::RawResult<lite::LInstance> {
  let enabled_extensions = try!(vk_api.select_extensions(extension_spec.clone()));
  let enabled_layers = try!(vk_api.select_layers(layer_spec.clone()));
  vk::instance_support::make_instance(
    vk::instance_support::InstanceCfgBuilder::default().build().unwrap(),
    &enabled_extensions,
    &enabled_layers,
    &|a| vk_api.create_instance(a),
  )
}

/** Dumps hardcoded x11-related extensions into a FeatureSpec */
fn make_x11_extension_spec() -> lite::FeatureSpec {
  lite::FeatureSpec {
    wanted: vec![
      //"VK_EXT_display_surface_counter",
      "VK_KHR_display",
      "VK_KHR_get_physical_device_properties2",
      "VK_KHR_get_surface_capabilities2",
      "VK_KHR_surface",
      //"VK_KHR_xcb_surface",
      "VK_KHR_xlib_surface",
      "VK_KHX_device_group_creation",
    ],
    required: vec![
      "VK_EXT_direct_mode_display",
      "VK_EXT_acquire_xlib_display",
      "VK_EXT_debug_report"
    ],
  }
}

/** Dumps hardcoded x11-related layers into a FeatureSpec */
fn make_x11_layer_spec() -> lite::FeatureSpec {
  lite::FeatureSpec {
    wanted: vec![
      "VK_LAYER_LUNARG_core_validation",
      "VK_LAYER_LUNARG_parameter_validation",
    ],
    required: vec!["VK_LAYER_LUNARG_standard_validation"],
  }
}
