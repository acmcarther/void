extern crate chrono;

mod spatial {
  struct Domain {
    did: u64,
  }

  /**
   * Something in the domain
   *
   * Carries no information of its own, properties must be associated to it.
   */
  pub struct Entity {
    e_id: u64,
    d_id: u64
  }

  mod forces {
    pub struct Force {
      vector: [f64; 3],
      mass_scale: ::spatial::property::MassScale,
    }
    pub struct AppliedForce {
      d_id: u64,
      e_id: u64,
      force_vector: Force,
    }

    fn gravitation(presences: &mut [::spatial::property::Presence3d]) -> Vec<AppliedForce> {
      // TODO: Very simple heuristic: order mass scales so that we can skip applying smaller scales to
      // larger bodies.
      //{
      //  let mut presence_refs = presences.iter_mut().collect::<Vec<&mut ::spatial::property::Presence3d>>();

      //  presence_refs.sort_by_key(|p| p.mass_scale);
      //}

      let len = presences.len();
      let mut forces = Vec::new();

      // TODO(acmcarther): Should do back and forward simulataneously
      for i in 0..len {
        for j in 0..len {
          if i == j {
            continue
          }

          let (mut p1, mut p2) = unsafe {
            (&mut presences.get_unchecked(i), &mut presences.get_unchecked(j))
          };
          let d_position = {
            let p1_position = p1.position;
            let p2_position = p2.position;
            [
              p1_position[0], p2_position[0],
              p1_position[1], p2_position[1],
              p1_position[2], p2_position[2],
            ]
          };
          let d_squared = {
            (d_position[0] * d_position[0])
            + (d_position[1] * d_position[1])
            + (d_position[2] * d_position[2])
          };

          // TODO: Multiply masses. Nontrivial because the scales aren't equal.
        }
      }

      forces
    }
  }

  mod property {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
    pub enum MassScale {
      // 1 Kg
      Kg,
      // TODO: something between Kg and EarthLike
      // 6 * 10^24 Kg
      EarthLike,
      // 2 * 10^30 Kg
      // 333_333 Earth
      Solar,
    }

    /**
     * A physical body in the domain.
     *
     * Some important properties
     * - inverse_mass is ignored if massless is true
     * - velocity is reduced to a unit vector * c if massless is true
     */
    pub struct Presence3d {
      pub e_id: u64,
      pub massless: bool,
      pub inverse_mass: f64,
      pub mass_scale: MassScale,
      pub position: [f64; 3],
      pub velocity: [f64; 3],
    }
  }

  fn generic_object(d_id: u64, e_id: u64, mass_scale: property::MassScale) -> (Entity, property::Presence3d) {
    let ent = Entity {
      e_id: e_id,
      d_id: d_id,
    };

    let presence = property::Presence3d {
      e_id: e_id,
      massless: false,
      inverse_mass: 1.0,
      mass_scale: mass_scale,
      position: [0.0, 0.0, 0.0],
      velocity: [0.0, 0.0, 0.0],
    };

    (ent, presence)
  }

  fn example_kg_object(d_id: u64, e_id: u64) -> (Entity, property::Presence3d) {
    generic_object(d_id, e_id, property::MassScale::Kg)
  }


  fn example_earth_object(d_id: u64, e_id: u64) -> (Entity, property::Presence3d) {
    generic_object(d_id, e_id, property::MassScale::EarthLike)
  }

  fn example_solar_object(d_id: u64, e_id: u64) -> (Entity, property::Presence3d) {
    generic_object(d_id, e_id, property::MassScale::Solar)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {
  }
}
