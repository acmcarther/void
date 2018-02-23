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
use planet_renderer::MeshToRender;
use planet_renderer::PlanetRenderer;
use renderer::BaseRenderer;
use renderer::BaseRendererConfig;
use sdl2::Sdl;
use sdl2::video::Window;
use sdl2_vulkan_interop::SdlWindowSystemPlugin;
use std::marker::PhantomData;
use std::time::Instant;

fn main() {
  init::init();

  let mut game_window = GameWindow::new();
  let mut planet_renderer = {
    let vulkan = vkl::Vulkan::new("libvulkan.so.1");
    let base_renderer = BaseRenderer::new(
      vulkan,
      &mut SdlWindowSystemPlugin::new(&mut game_window.window),
      BaseRendererConfig {
        extension_spec: x11_extension_spec(),
        layer_spec: x11_layer_spec(),
      },
    );
    PlanetRenderer::new(base_renderer)
  };

  let start_time = Instant::now();
  let mut event_pump = game_window.sdl.event_pump().unwrap();

  let mut meshes_to_render = Vec::new();
  let base_mesh_ids = vec![
    planet_renderer::ICO_1_MESH_ID,
    planet_renderer::ICO_2_MESH_ID,
    planet_renderer::ICO_3_MESH_ID,
    planet_renderer::ICO_4_MESH_ID,
    planet_renderer::ICO_5_MESH_ID,
    planet_renderer::ICO_6_MESH_ID,
  ];

  for (idx, base_mesh_id) in base_mesh_ids.into_iter().enumerate() {
    let x = -50f32 + ((idx as f32) * 20f32);
    for y_idx in 0..6 {
      let y = -50f32 + ((y_idx as f32) * 20f32);
      for z_idx in 0..6 {
        let z = -50f32 + ((z_idx as f32) * 20f32);
        meshes_to_render.push(MeshToRender {
          mesh_id: base_mesh_id,
          pos: [x, y, z],
        });
      }
    }
  }

  let mut pause = false;
  let mut cam_x = 200f32;
  let mut cam_y = 200f32;
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. }
        | sdl2::event::Event::KeyDown {
          keycode: Some(sdl2::keyboard::Keycode::Escape),
          ..
        } => break 'running,
        sdl2::event::Event::KeyDown {
          keycode: Some(sdl2::keyboard::Keycode::Space),
          ..
        } => pause = !pause,
        _ => {},
      }
    }

    let now = Instant::now();
    let duration = now.duration_since(start_time);
    let time_since_start_s =
      (duration.as_secs() as f32) + ((duration.subsec_nanos()) as f32 / 1_000_000_000.0);

    if !pause {
      cam_x = time_since_start_s.sin() * 200f32;
      cam_y = time_since_start_s.cos() * 200f32;
    }

    planet_renderer.set_camera_pos(cam_x, cam_y, 150f32);

    planet_renderer.draw_demo_frame(&meshes_to_render);
  }
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
