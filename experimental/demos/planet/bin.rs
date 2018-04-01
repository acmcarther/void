extern crate init;
#[macro_use]
extern crate log;
extern crate renderer;
extern crate sdl2;
extern crate sdl2_win;
extern crate sim;

use renderer::Renderer;
use sdl2_win::GameWindow;
use sim::PlanetGen;
use sim::PlanetGenConfig;
use sim::PlanetSim;
use sim::PlanetSimConfig;
use std::time::Instant;

fn main() {
  init::init();

  let mut game_window = GameWindow::new();

  let planet = PlanetGen::new(PlanetGenConfig::default()).smooth();
  let mut game_loop = GameLoop::new(GameLoopDeps {
    renderer: Renderer::from_sdl_window(&mut game_window.window),
    sim: PlanetSim::new(PlanetSimConfig::default(), planet),
  });

  let mut event_pump = game_window.sdl.event_pump().unwrap();
  while game_loop.running() {
    let now = Instant::now();

    game_loop.process_input(&mut event_pump);
    game_loop.tick(now);
  }

  info!("exited");
}

struct GameLoopDeps<'window> {
  pub renderer: Renderer<'window>,
  pub sim: PlanetSim,
}

struct GameLoopInit {
  pub start_time: Instant,
}

struct TickState {
  id: u64,
  time: Instant,
}

struct GameLoop<'window> {
  init: GameLoopInit,
  deps: GameLoopDeps<'window>,
  last_tick: TickState,
  running: bool,
}

impl<'window> GameLoop<'window> {
  pub fn new(deps: GameLoopDeps) -> GameLoop {
    let now = Instant::now();
    GameLoop {
      init: GameLoopInit {
        start_time: now.clone(),
      },
      deps: deps,
      last_tick: TickState { id: 1, time: now },
      running: true,
    }
  }

  pub fn running(&self) -> bool {
    self.running.clone()
  }


  pub fn process_input(&mut self, event_pump: &mut sdl2::EventPump) {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. }
        | sdl2::event::Event::KeyDown {
          keycode: Some(sdl2::keyboard::Keycode::Escape),
          ..
        } => self.running = false,
        _ => {},
      }
    }
  }

  pub fn tick(&mut self, now: Instant) {
    let id = self.last_tick.id + 1;
    let dt = now.duration_since(self.last_tick.time);




    {
      self.last_tick.id = 1;
      self.last_tick.time = now;
    }
  }
}
