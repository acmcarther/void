extern crate cosmic_physics;
extern crate rand;
extern crate gnuplot;
extern crate vk_sys;

use cosmic_physics::CelestialGrid;
use cosmic_physics::CelestialVector;
use cosmic_physics::CosmicParams;

use gnuplot::Figure;
use gnuplot::Caption;
use gnuplot::Color;

use rand::Rng;

fn main() {
  let cosmic_params = CosmicParams {
    gravitational_constant: 10.0,
  };

  const POSITION_CEILING: f64 = 500_000_000.0;
  const VELOCITY_CEILING: f64 = 100.0;
  const STAR_MASS: i64 = 2_000_000_000_000_000i64;
  const PLANET_MASS: i64 = 1_000_000_000i64;
  const ROCK_MASS: i64 = 1_000;

  let mut grid = CelestialGrid::new();
  let mut rng = rand::thread_rng();

  for _ in 0..100 {
    grid.insert_system(
      CelestialVector {
        x: (POSITION_CEILING * rng.gen::<f64>()) as i64,
        y: (POSITION_CEILING * rng.gen::<f64>()) as i64,
        z: 0i64,
      },
      CelestialVector {
        x: (VELOCITY_CEILING * rng.gen::<f64>()) as i64,
        y: (VELOCITY_CEILING * rng.gen::<f64>()) as i64,
        z: 0i64,
      },
      STAR_MASS
    );
  }

  for _ in 0..10000 {
    grid.insert_system(
      CelestialVector {
        x: (POSITION_CEILING * rng.gen::<f64>()) as i64,
        y: (POSITION_CEILING * rng.gen::<f64>()) as i64,
        z: 0i64,
      },
      CelestialVector {
        x: (VELOCITY_CEILING * rng.gen::<f64>()) as i64,
        y: (VELOCITY_CEILING * rng.gen::<f64>()) as i64,
        z: 0i64,
      },
      PLANET_MASS
    );
  }

  /*
  for _ in 0..10000 {
    grid.insert_system(
      CelestialVector {
        x: (POSITION_CEILING * rng.gen::<f64>() - HALF_POSITION_CEILING) as i64,
        y: (POSITION_CEILING * rng.gen::<f64>() - HALF_POSITION_CEILING) as i64,
        z: 0i64,
      },
      CelestialVector {
        x: (VELOCITY_CEILING * rng.gen::<f64>() - HALF_VELOCITY_CEILING) as i64,
        y: (VELOCITY_CEILING * rng.gen::<f64>() - HALF_VELOCITY_CEILING) as i64,
        z: 0i64,
      },
      ROCK_MASS
    );
  }*/

  let mut state_snaps = Vec::new();

  for tick in 0..100000 {
    if tick % 1 == 0 {
      println!("Tick {}", tick);
    }

    if tick % 1000 == 0 {
      let mut snap_ents = Vec::new();
      for id in grid.get_system_ids().iter() {
        if let Some(system_details) = grid.get_system_details(id.clone()) {
          snap_ents.push([system_details.coords.x.clone(), system_details.coords.y.clone(), system_details.coords.z.clone(), system_details.mass.clone()]);
        }

      }
      state_snaps.push(snap_ents);
    }

    if tick % 10 == 0 {
      for id in grid.get_system_ids().iter() {
        let mut remove_system = false;
        if let Some(system_details) = grid.get_system_details(id.clone()) {
          if system_details.coords.x > 5_000_000_000 || system_details.coords.x < -4_500_000_000 {
            remove_system = true;
          } else if system_details.coords.y > 5_000_000_000 || system_details.coords.y < -4_500_000_000 {
            remove_system = true;
          }

        }
        if remove_system {
          println!("removing system: {}", id);
          grid.remove_system(id.clone());
        }
      }
    }

    grid.tick_celestial_grid(&cosmic_params, 9_000_000u64);
  }

  for (idx, state_snap) in state_snaps.into_iter().enumerate() {
    let mut fg = Figure::new();

    let mut x_values_large = Vec::new();
    let mut x_values_medium = Vec::new();
    let mut x_values_small = Vec::new();
    let mut y_values_large = Vec::new();
    let mut y_values_medium = Vec::new();
    let mut y_values_small = Vec::new();

    for data in state_snap.into_iter() {
      match data[3] {
         STAR_MASS => {
           x_values_large.push(data[0]);
           y_values_large.push(data[1]);
         },
         PLANET_MASS => {
           x_values_medium.push(data[0]);
           y_values_medium.push(data[1]);
         },
         ROCK_MASS => {
           x_values_small.push(data[0]);
           y_values_small.push(data[1]);
         },
         _ => {}
      }
    }

    fg.axes2d()
      .points(&x_values_large, y_values_large, &[Caption("large bodies"), Color("red")])
      .points(&x_values_medium, y_values_medium, &[Caption("medium bodies"), Color("yellow")])
      .points(&x_values_small, y_values_small, &[Caption("small bodies"), Color("green")]);


    fg.echo_to_file(&format!("/usr/local/google/home/acmcarther/projects/void/cosmic_toy_{}.txt", idx));
  }
}
