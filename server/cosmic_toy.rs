extern crate cosmic_physics;
extern crate gnuplot;

use cosmic_physics::CelestialGrid;
use cosmic_physics::CelestialVector;
use cosmic_physics::CosmicParams;

use gnuplot::Figure;
use gnuplot::Caption;
use gnuplot::Color;

fn main() {
  let cosmic_params = CosmicParams {
    gravitational_constant: 1000.0,
  };

  let mut grid = CelestialGrid::new();
  let system_1 = grid.insert_system(
    CelestialVector { x: -10000, y: 0, z: 0, },
    CelestialVector { x: 0, y: -1000, z: 0, },
    100_000_000i64
  );

  let system_2 = grid.insert_system(
    CelestialVector { x: 10000, y: 0, z: 0, },
    CelestialVector { x: 0, y: 1000, z: 0, },
    100_000_000i64
  );

  let system_3 = grid.insert_system(
    CelestialVector { x: 0, y: 50000, z: 0, },
    CelestialVector { x: -1200, y: 0, z: 0, },
    1_000i64
  );

  let mut position_1_x = Vec::new();
  let mut position_1_y = Vec::new();
  let mut position_2_x = Vec::new();
  let mut position_2_y = Vec::new();
  let mut position_3_x = Vec::new();
  let mut position_3_y = Vec::new();

  for _ in 0..9000 {
    grid.tick_celestial_grid(&cosmic_params, 50_000u64);

    let system_details_1 = grid.get_system_details(system_1).unwrap();
    let system_details_2 = grid.get_system_details(system_2).unwrap();
    let system_details_3 = grid.get_system_details(system_3).unwrap();
    {
      position_1_x.push(system_details_1.coords.x);
      position_1_y.push(system_details_1.coords.y);
      position_2_x.push(system_details_2.coords.x);
      position_2_y.push(system_details_2.coords.y);
      position_3_x.push(system_details_3.coords.x);
      position_3_y.push(system_details_3.coords.y);
    }
  }

  let mut fg = Figure::new();
  fg.axes2d()
    .lines(&position_1_x, position_1_y, &[Caption("Object 1 pos"), Color("red")])
    .lines(&position_2_x, position_2_y, &[Caption("Object 2 pos"), Color("blue")])
    .lines(&position_3_x, position_3_y, &[Caption("Object 3 pos"), Color("green")]);

  fg.show();
}
