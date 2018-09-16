extern crate init;
#[macro_use]
extern crate log;
extern crate sdl2;

mod implementation {
  use sdl2;
  use sdl2::audio::AudioCallback;
  use sdl2::audio::AudioSpecDesired;
  use std::time::Duration;
  use std::thread;

  const MONO_AUDIO_SPEC: AudioSpecDesired = AudioSpecDesired {
    freq: Some(44100),
    channels: Some(1),
    samples: None,
  };

  struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
  }

  impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
      // Generate a square wave
      for x in out.iter_mut() {
        *x = if self.phase <= 0.5 {
          self.volume
        } else {
          -self.volume
        };
        self.phase = (self.phase + self.phase_inc) % 1.0;
      }
    }
  }

  fn debug_drivers() {
    for driver in sdl2::audio::drivers() {
      info!("Audio Driver: {}", driver);
    }
  }

  pub fn real_main() {
    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    debug_drivers();

    let device = audio
      .open_playback(None, &MONO_AUDIO_SPEC, |spec| SquareWave {
        phase_inc: 440.0 / spec.freq as f32,
        phase: 0.0,
        volume: 0.25,
      })
      .unwrap();

    device.resume();

    thread::sleep(Duration::from_millis(2000));
  }
}

fn main() {
  init::init();

  implementation::real_main();

  preexit();
}

fn preexit() {
  info!("Exiting");
}
