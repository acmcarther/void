extern crate chrono;
extern crate rand;

use std::collections::HashMap;

/** The complete cosmos, containing point masses representing system masses */
pub type SystemId = u64;
/** 1 -> 1_000_000_000_000_000_000 terestrial masses */
pub type CeleplanetaryMass = i64;
/** 1 -> 1 terestrial length */
pub type CeleterestrialLength = i64;

pub struct CosmicParams {
  pub gravitational_constant: f64,
}

/** A spatial vector in Celestrial units. */
#[derive(Clone, Debug)]
pub struct CelestialVector {
  pub x: i64,
  pub y: i64,
  pub z: i64,
}

/** A spatial vector in Celestrial units. */
#[derive(Clone, Debug)]
pub struct CelestialVectorF {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

pub struct CelestialGrid {
  next_system_id: SystemId,
  system_ids: Vec<SystemId>,
  system_coordinates: HashMap<SystemId, CelestialVector>,
  system_trajectories: HashMap<SystemId, CelestialVector>,
  system_masses: HashMap<SystemId, CeleplanetaryMass>,
}

#[derive(Debug)]
pub struct SystemDetails<'a> {
  pub coords: &'a CelestialVector,
  pub trajectory: &'a CelestialVector,
  pub mass: &'a CeleplanetaryMass,
}

/** Apply cosmic gravity, and apply system trajectories. */
impl CelestialGrid {
  pub fn new() -> CelestialGrid {
    CelestialGrid {
      next_system_id: 1u64,
      system_ids: Vec::new(),
      system_coordinates: HashMap::new(),
      system_trajectories: HashMap::new(),
      system_masses: HashMap::new(),
    }
  }

  pub fn insert_system(&mut self, coords: CelestialVector, trajectory: CelestialVector, mass: CeleplanetaryMass) -> SystemId {
    let system_id = self.next_system_id;
    self.next_system_id = self.next_system_id + 1;

    self.system_ids.push(system_id);
    self.system_coordinates.insert(system_id, coords);
    self.system_trajectories.insert(system_id, trajectory);
    self.system_masses.insert(system_id, mass);

    system_id
  }

  pub fn get_system_details<'a>(&'a self, system_id: SystemId) -> Option<SystemDetails<'a>> {
    if !self.system_coordinates.contains_key(&system_id) {
      return None
    }

    Some(SystemDetails {
      coords: self.system_coordinates.get(&system_id).unwrap(),
      trajectory: self.system_trajectories.get(&system_id).unwrap(),
      mass: self.system_masses.get(&system_id).unwrap(),
    })
  }

  pub fn tick_celestial_grid(&mut self, cosmic_params: &CosmicParams, dt_micros: u64) {
    // Tick gravitational force
    let dt_s = dt_micros as f64 / 1000000.0;
    {
      let mut force_sets: HashMap<SystemId, CelestialVectorF> = HashMap::new();
      for id_idx_1 in 0..self.system_ids.len() {
        for id_idx_2 in (id_idx_1+1)..self.system_ids.len() {
          let id_1 = self.system_ids.get(id_idx_1).unwrap();
          let id_2 = self.system_ids.get(id_idx_2).unwrap();
          let mass_product = {
            self.system_masses.get(id_1).unwrap()
              * self.system_masses.get(id_2).unwrap()
          };

          if mass_product == 0 {
            continue
          }

          let (distance_unit_vector, distance) = {
            let id_1_coords = self.system_coordinates.get(id_1).unwrap();
            let id_2_coords = self.system_coordinates.get(id_2).unwrap();

            let distance_vector = CelestialVector {
              x: id_1_coords.x - id_2_coords.x,
              y: id_1_coords.y - id_2_coords.y,
              z: id_1_coords.z - id_2_coords.z,
            };
            let distance =
              ((distance_vector.x.pow(2)
                + distance_vector.y.pow(2)
                + distance_vector.z.pow(2)) as f64).sqrt();

            let distance_unit_vector = CelestialVectorF {
              x: (distance_vector.x as f64) / distance,
              y: (distance_vector.y as f64) / distance,
              z: (distance_vector.z as f64) / distance,
            };
            (distance_unit_vector, distance)
          };

          let force_magnitude = cosmic_params.gravitational_constant
                                * (mass_product as f64) / (distance * distance);

          let force_vector = {
            CelestialVectorF {
              x: distance_unit_vector.x * force_magnitude * dt_s,
              y: distance_unit_vector.y * force_magnitude * dt_s,
              z: distance_unit_vector.z * force_magnitude * dt_s,
            }
          };

          let negative_force_vector = CelestialVectorF {
            x: -force_vector.x,
            y: -force_vector.y,
            z: -force_vector.z,
          };

          if force_sets.contains_key(&id_1) {
            let mut last_force = force_sets.get_mut(&id_1).unwrap();
            last_force.x = last_force.x + negative_force_vector.x;
            last_force.y = last_force.y + negative_force_vector.y;
            last_force.z = last_force.z + negative_force_vector.z;
          } else {
            force_sets.insert(id_1.clone(), negative_force_vector.clone());
          }

          if force_sets.contains_key(&id_2) {
            let mut last_force = force_sets.get_mut(&id_2).unwrap();
            last_force.x = last_force.x + force_vector.x;
            last_force.y = last_force.y + force_vector.y;
            last_force.z = last_force.z + force_vector.z;
          } else {
            force_sets.insert(id_2.clone(), force_vector);
          }
        }
      }

      for (id, force_vector) in force_sets.into_iter() {
        let mass = (*self.system_masses.get(&id).unwrap()) as f64;

        let mut trajectory = self.system_trajectories.get_mut(&id).unwrap();

        trajectory.x = trajectory.x + (force_vector.x / mass) as i64;
        trajectory.y = trajectory.y + (force_vector.y / mass) as i64;
        trajectory.z = trajectory.z + (force_vector.z / mass) as i64;
      }
    }

    // Apply velocities
    {
      for id_idx in 0..self.system_ids.len() {
        let id = self.system_ids.get(id_idx).unwrap();
        let system_trajectory = self.system_trajectories.get(id).unwrap();
        let mut system_coordinates = self.system_coordinates.get_mut(id).unwrap();
        system_coordinates.x = system_coordinates.x + (system_trajectory.x as f64 * dt_s) as i64;
        system_coordinates.y = system_coordinates.y + (system_trajectory.y as f64 * dt_s) as i64;
        system_coordinates.z = system_coordinates.z + (system_trajectory.z as f64 * dt_s) as i64;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn single_stationary_system_does_not_move() {
    let cosmic_params = CosmicParams {
      gravitational_constant: 1.0,
    };

    let mut grid = CelestialGrid::new();
    let system_1 = grid.insert_system(
      CelestialVector { x: 0, y: 0, z: 0, },
      CelestialVector { x: 0, y: 0, z: 0, },
      100i64
    );

    grid.tick_celestial_grid(&cosmic_params, 1_000_000u64);

    let system_details = grid.get_system_details(system_1).unwrap();

    assert_eq!(system_details.coords.x, 0i64);
    assert_eq!(system_details.coords.y, 0i64);
    assert_eq!(system_details.coords.z, 0i64);
    assert_eq!(system_details.trajectory.x, 0i64);
    assert_eq!(system_details.trajectory.y, 0i64);
    assert_eq!(system_details.trajectory.z, 0i64);
  }

  #[test]
  fn single_system_moves_predictably() {
    let cosmic_params = CosmicParams {
      gravitational_constant: 1.0,
    };

    let mut grid = CelestialGrid::new();
    let system_1 = grid.insert_system(
      CelestialVector { x: 0, y: 0, z: 0, },
      CelestialVector { x: 1, y: -1, z: 5, },
      100i64
    );

    grid.tick_celestial_grid(&cosmic_params, 2_000_000u64);

    let system_details = grid.get_system_details(system_1).unwrap();

    assert_eq!(system_details.coords.x, 2i64);
    assert_eq!(system_details.coords.y, -2i64);
    assert_eq!(system_details.coords.z, 10i64);
    assert_eq!(system_details.trajectory.x, 1i64);
    assert_eq!(system_details.trajectory.y, -1i64);
    assert_eq!(system_details.trajectory.z, 5i64);
  }

  #[test]
  fn two_systems_interact_predictably() {
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

    for _ in 0..9000 {
      grid.tick_celestial_grid(&cosmic_params, 50_000u64);

    }

    let system_details_1 = grid.get_system_details(system_1).unwrap();
    let system_details_2 = grid.get_system_details(system_2).unwrap();

    // Verify that system did not dissociate
    assert!(system_details_1.coords.x < 20000 && system_details_1.coords.x > -20000);
    assert!(system_details_1.coords.y < 20000 && system_details_1.coords.y > -20000);
    assert!(system_details_1.coords.z < 20000 && system_details_1.coords.z > -20000);
    assert!(system_details_2.coords.x < 20000 && system_details_2.coords.x > -20000);
    assert!(system_details_2.coords.y < 20000 && system_details_2.coords.y > -20000);
    assert!(system_details_2.coords.z < 20000 && system_details_2.coords.z > -20000);

    // Verify that system did not converge at some point
    assert!(system_details_1.coords.x != system_details_2.coords.x);
    assert!(system_details_1.coords.y != system_details_2.coords.y);
  }
}