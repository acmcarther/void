#![feature(used)]
extern crate cgmath;
extern crate cosmic_physics as cp;
extern crate galaxy_big_renderer;
extern crate geometry;
extern crate icosphere;
extern crate init;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rand;
extern crate renderer;
extern crate sdl2;
extern crate sdl2_vulkan_interop;
extern crate vk_lite as vkl;
#[macro_use]
extern crate zcfg;

use cgmath::Matrix4;
use geometry::Mesh;
use galaxy_big_renderer::MeshToRender;
use galaxy_big_renderer::GalaxyBigRenderer;
use rand::Rng;
use renderer::BaseRenderer;
use renderer::BaseRendererConfig;
use sdl2::Sdl;
use sdl2::video::Window;
use sdl2_vulkan_interop::SdlWindowSystemPlugin;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use std::time::Instant;

mod flags {
  define_pub_cfg!(
    sim_type,
    String,
    "sphere_grid",
    "type of sim to render (sphere_grid|galaxy)"
  );
}

lazy_static! {
  static ref SPHERE_MESHES_TO_RENDER: Vec<MeshToRender> = {
    let mut meshes_to_render = Vec::new();
    let base_mesh_ids = vec![
      galaxy_big_renderer::ICO_1_MESH_ID,
      galaxy_big_renderer::ICO_2_MESH_ID,
      galaxy_big_renderer::ICO_3_MESH_ID,
      galaxy_big_renderer::ICO_4_MESH_ID,
      galaxy_big_renderer::ICO_5_MESH_ID,
      galaxy_big_renderer::ICO_6_MESH_ID,
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
            scale: 8f32,
          });
        }
      }
    }
    meshes_to_render
  };

  static ref GALAXY_SIM: Arc<Mutex<GalaxySim>> = Arc::new(Mutex::new(GalaxySim::new()));
}

fn main() {
  init::init();

  let mut game_window = GameWindow::new();
  let mut galaxy_big_renderer = {
    let vulkan = vkl::Vulkan::new("libvulkan.so.1");
    let base_renderer = BaseRenderer::new(
      vulkan,
      &mut SdlWindowSystemPlugin::new(&mut game_window.window),
      BaseRendererConfig {
        extension_spec: x11_extension_spec(),
        layer_spec: x11_layer_spec(),
      },
    );
    GalaxyBigRenderer::new(base_renderer)
  };

  let start_time = Instant::now();
  let mut event_pump = game_window.sdl.event_pump().unwrap();

  let mut meshes_to_render = Vec::new();
  let base_mesh_ids = vec![
    galaxy_big_renderer::ICO_1_MESH_ID,
    galaxy_big_renderer::ICO_2_MESH_ID,
    galaxy_big_renderer::ICO_3_MESH_ID,
    galaxy_big_renderer::ICO_4_MESH_ID,
    galaxy_big_renderer::ICO_5_MESH_ID,
    galaxy_big_renderer::ICO_6_MESH_ID,
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
          scale: 8f32,
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
        _ => {}
      }
    }

    let now = Instant::now();
    let duration = now.duration_since(start_time);
    let time_since_start_s =
      (duration.as_secs() as f32) + ((duration.subsec_nanos()) as f32 / 1_000_000_000.0);

    if !pause {
      cam_x = (time_since_start_s / 2.0).sin() * 300f32;
      cam_y = (time_since_start_s / 2.0).cos() * 300f32;
    }

    let sim_type = ::flags::sim_type::CONFIG.get_value();

    let target = if sim_type == "galaxy" {
      let sim = GALAXY_SIM.lock().unwrap();
      sim.get_centroid()
    } else {
      [0f32, 0f32, 0f32]
    };

    let cam_pos = if sim_type == "galaxy" {
      [cam_x + target[0], cam_y + target[1], 225f32 + target[2]]
    } else {
      [cam_x, cam_y, 225f32]
    };
    galaxy_big_renderer.set_camera_pos(cam_pos, target);

    match sim_type.as_ref() {
      "sphere_grid" => galaxy_big_renderer.draw_demo_frame(&SPHERE_MESHES_TO_RENDER),
      "galaxy" => {
        let mut sim = GALAXY_SIM.lock().unwrap();
        sim.tick();
        galaxy_big_renderer.draw_demo_frame(sim.get_meshes_to_render());
      }
      something_else => {
        panic!("Unknown sim type {}", something_else);
      }
    }
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

struct GalaxySim {
  join_handle: JoinHandle<()>,
  meshes_to_render: Vec<MeshToRender>,
  state_recv: Receiver<Vec<MeshToRender>>,
}

impl GameWindow {
  pub fn new() -> GameWindow {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
      .window("galaxy big demo", 1920, 1080)
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

impl GalaxySim {
  pub fn new() -> GalaxySim {
    const POSITION_CEILING: f64 = 100.0;
    const VELOCITY_CEILING: f64 = 20.0;
    const STAR_MASS: f64 = 600.0f64;
    const PLANET_MASS: f64 = 1.0f64;
    let (state_send, state_recv) = std::sync::mpsc::channel();

    let join_handle = std::thread::spawn(move || {
      let mut grid = cp::CelestialGrid::new();
      let mut rng = rand::thread_rng();
      let cosmic_params = cp::CosmicParams {
        gravitational_constant: 10.0,
      };

      for _ in 0..2 {
        grid.insert_system(
          cp::CelestialVector {
            x: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
            y: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
            z: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
          },
          cp::CelestialVector {
            x: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
            y: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
            z: 0.0f64,
          },
          STAR_MASS,
        );
      }

      for _ in 0..40 {
        grid.insert_system(
          cp::CelestialVector {
            x: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
            y: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
            z: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
          },
          cp::CelestialVector {
            x: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
            y: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
            z: 0.0f64,
          },
          PLANET_MASS,
        );
      }

      for tick in 0..100000000 {
        grid.tick_celestial_grid(&cosmic_params, 900u64);

        let mut snap_ents = Vec::new();
        for id in grid.get_system_ids().iter() {
          if let Some(system_details) = grid.get_system_details(id.clone()) {
            snap_ents.push(MeshToRender {
              mesh_id: galaxy_big_renderer::ICO_2_MESH_ID,
              pos: [
                system_details.coords.x.clone() as f32,
                system_details.coords.y.clone() as f32,
                system_details.coords.z.clone() as f32,
              ],
              scale: (system_details.mass.clone().cbrt() / 3.0) as f32,
            });
          }
        }
        state_send.send(snap_ents).unwrap();

        if tick % 10 == 0 {
          for id in grid.get_system_ids().iter() {
            let mut remove_system = false;
            let mut system_mass = 0.0;
            if let Some(system_details) = grid.get_system_details(id.clone()) {
              if system_details.coords.x > 2_000.0 || system_details.coords.x < -2_000.0 {
                remove_system = true;
                system_mass = *system_details.mass;
              } else if system_details.coords.y > 2_000.0 || system_details.coords.y < -2_000.0 {
                remove_system = true;
                system_mass = *system_details.mass;
              }
            }
            if remove_system {
              println!("removing system: {}", id);
              grid.remove_system(id.clone());
              grid.insert_system(
                cp::CelestialVector {
                  x: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
                  y: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
                  z: 0.0f64,
                },
                cp::CelestialVector {
                  x: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
                  y: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
                  z: 0.0f64,
                },
                system_mass,
              );
            }
          }
        }
      }
    });
    GalaxySim {
      join_handle: join_handle,
      meshes_to_render: Vec::new(),
      state_recv: state_recv,
    }
  }

  pub fn tick(&mut self) {
    loop {
      let rcv = self.state_recv.try_recv();
      let exit_state_poll = match rcv {
        Ok(state) => {
          self.meshes_to_render = state;
          false
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => true,
        Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("main thread hung up!"),
      };
      if exit_state_poll {
        break;
      }
    }
  }

  pub fn get_meshes_to_render(&self) -> &Vec<MeshToRender> {
    &self.meshes_to_render
  }

  pub fn get_centroid(&self) -> [f32; 3] {
    let mut centroid_sum = [0.0, 0.0, 0.0];
    let mut total_mass = 0f32;

    for mesh_to_render in self.meshes_to_render.iter() {
      total_mass += mesh_to_render.scale;

      centroid_sum[0] += mesh_to_render.pos[0] * mesh_to_render.scale;
      centroid_sum[1] += mesh_to_render.pos[1] * mesh_to_render.scale;
      centroid_sum[2] += mesh_to_render.pos[2] * mesh_to_render.scale;
    }

    [
      centroid_sum[0] / total_mass,
      centroid_sum[1] / total_mass,
      centroid_sum[2] / total_mass,
    ]
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

fn get_meshes() {}
