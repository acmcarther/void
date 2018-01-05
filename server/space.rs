extern crate chrono;
extern crate rand;

use std::collections::HashMap;

mod positioning_2 {
  /**
   * Real life useful constants
   *
   * Length:
   * - 1 meter (base unit)
   * - 1 au: 140_000_000_000 m
   * - 1 ly: 63_241 AU
   * - 1 Galactic Diameter: 100_000 ly
   * 
   * Mass:
   * - 1 kg (base unit)
   * - 1 earth mass: 6 * 10^24 kg
   * - 1 jupiter mass: 2 * 10^27 kg
   * - 1 solar mass: 2 * 10^30 kg
   * - 1 stellar black hole mass: 10 * solar mass
   * - 1 intermediate black hole mass: 50_000 solar mass
   * - 1 supermassive black hole mass: 10_000_000 solar mass
   * 
   * Time:
   * - 1 second (base unit)
   * - 1 day: 86_400 seconds
   * - 1 year: 31_557_600 seconds
   * - 1 galactic year: 240_000_000 years
   * 
   * Speed:
   * - 1 m/s (base unit)
   * - Earth orbital speed: 30_000 m/s
   * - Solar system speed around galaxy: 230_000 m/s
   * - Speed of Light: 300_000_000 m/s
   * 
   * Size:
   * - 1 meter (base unit)
   * - 1 earth diameter: 13_000_000 m
   * - 1 solar diameter: 1_400_000_000 m
   *
   * Entity Count
   * - Stars in the galaxy: 250_000_000
   */

  /**
   * Gameplay benchmarks
   * 
   * Player Scale:
   * - Height: 2m
   * - Mass: 100kg
   * - Velocity: Circumnavigate the planet in 30h
   *
   * Terestrial Vehicle Scale:
   * - Height 5m
   * - Mass 10_000 kg
   * - Velocity: Circumnavigate the planet in 2h
   *
   * Airborne Vehicle Scale:
   * - Size 50m
   * - Mass: 400_000 kg
   * - Velocity: Circumnavigate the planet in 20m
   *
   * Primitive Space Vessel
   * - Size: 200m
   * - Mass: 60_000 kg
   * - Velocity: Circumnavigate the planet in 30s
   *             Reach satelite in 10m
   *             Reach sun in 20m
   *             Reach near star in 2d
   *
   * Advanced Space Vessel
   * - Size: 1 km
   * - Mass: ????
   * - Velocity: Circumnavigate the planet at any speed
   *             Reach satelite in 20s
   *             Reach sun in 2m
   *             Reach near star in 5h
   * 
   * Primitive Interstellar Vessel
   * - Size: ????
   * - Mass: ????
   * - Velocity: ????
   *             Reach sun in 30s
   *             Reach near star in 20m
   *             Reach farthest star in 3w
   *
   * Moderate Interstellar Vessel
   * - Size: ????
   * - Mass: ????
   * - Velocity: 5_475_000_000_000_000 m/s
   *             Reach near star in 8m
   *             Reach farthest star in 1w
   * 
   * Advanced Interstellar Vessel
   * - Size: ????
   * - Mass: ????
   * - Velocity: Reach farthest star in 6h
   */

  /**
   * Derived Physical Constants
   * - Size of galaxy: 12 light hours
   *                   1_000_000_000_000_000_000 m
   * - Speed of Light: 300_000_000_000_000 m/s
   * - Number of Stars: 500_000
   * - Gravitational Constant
   */

  /**
   * Interesting gameplay scales (one to one with physically simulated scales):
   *
   * Galactic Scale:
   * - ???
   *
   * Stellar Scale:
   * - ???
   *
   * Planetary Scale:
   * - ???
   *
   * Player Scale:
   * - ???
   */
}

mod positioning {
  pub type ObjectId = usize;
  pub type ReferenceFrameId = usize;

  // TODO(acmcarther): This should potentially be based on the mass of the objects in the frame
  const MAX_REFERENCE_FRAME_SIZE_M: u64 = 300_000_000;
  const MAX_REFERENCE_FRAME_SIZE_M_SQUARED: u64 = 90_000_000_000_000_000;
  const M_PER_LS: u64 = 299_792_000;
  const APPROX_MAX_REFERENCE_FRAME_SIZE_LS_SQUARED: u64 = 300208;

  // Unit: LS
  // (1 LS -> 299_792_000 M)
  pub struct GalacticCoordinate {
    x_ls: f64
    y_ls: f64
    z_ls: f64
  }

  pub enum MassScaleFactor {
    Kg,
    Earth,
    Solar
  }

  // Roughly 6 * 10^38 N
  pub struct GalacticForce {
    x_solar_mass_ls_per_s_sq: f64
    y_solar_mass_ls_per_s_sq: f64
    z_solar_mass_ls_per_s_sq: f64
  }

  // Unit: M
  pub struct SpatialCoordinate {
    rf_id: ReferenceFrameId
    x_m: f64,
    y_m: f64,
    z_m: f64,
  }

  // Roughly 6 * 10^24 N
  pub struct SpatialForce {
    x_earth_mass_m_per_s_sq: f64
    y_earth_mass_m_per_s_sq: f64
    z_earth_mass_m_per_s_sq: f64
  }

  struct CompleteCoordinate {
    x_ls: f64
    y_ls: f64
    z_ls: f64
    x_m: f64,
    y_m: f64,
    z_m: f64,
  }

  pub struct ReferenceFrame {
    rf_id: ReferenceFrameId
    coord: GalacticCoordinate
  }

  enum ReferenceFrameOperationError {
    ReferenceFramesFrozen,
    CombinationFailedFromFrameOverdistance,
    CombinationFailedFromObjectOverdistance,
  }

  enum ObjectOperationError {
    ReferenceFramesUnfrozen,
    ReferenceFrameDoesNotExist,
    InsertFailedFromOverdistance,
  }

  enum ReferenceFrameMutationState {
    Frozen,
    Unfrozen,
  }

  pub struct ObjectMass {
    inv_kg_masses: f64,
    inv_earth_masses: f64,
    inv_solar_masses: f64,
  }

  pub struct Object {
    o_id: ObjectId,
    coord: SpatialCoordinate,
    massless: bool,
    approx_inv_mass: ObjectMass,
  }

  pub struct PositioningData {
    // Authoritative
    next_rf_id: ReferenceFrameId,
    next_o_id: ObjectId,
    rf_mutation_state: ReferenceFrameMutationState,
    reference_frames: HashMap<ReferenceFrameId, ReferenceFrame>
    objects: HashMap<ObjectId, Object>
    // Indexes
    reference_frame_objects: HashMap<ReferenceFrameId, Vec<ObjectId>>
  }

  pub struct CosmosSeedingSystem {
    stars_to_be_seeded: u64,
    universe_diameter_ls: f64,
  }

  impl CosmosSeedingSystem {
    pub fn seed_cosmos(&self, positioning_data: &mut PositioningData) {
      let mut rng = rand::thread_rng();
      for _ : 0..self.stars_to_be_seeded {
        let galactic_x_ls = universe_diameter_ls * rng::next_f64();
        let galactic_y_ls = universe_diameter_ls * rng::next_f64();
        let galactic_z_ls = universe_diameter_ls * rng::next_f64();

        let rf_id = positioning_data.create_reference_frame(
          GalacticCoordinate {
            x_ls: galactic_x_ls,
            y_ls: galactic_y_ls,
            z_ls: galactic_z_ls,
          });

        let o_id = positioning_data.insert_object_into_frame(
          rf_id,
          SpatialCoordinate {
            0.0,
            0.0,
            0.0,
          },
          false /* massless */,
          ObjectMass {
            inv_kg_masses: 0.0,
            inv_earth_masses: 0.0,
            inv_solar_masses: 1.0,
          });
      }
    }
  }

  pub struct CoarseGrainedGravitationalSystem { }

  impl CoarseGrainedGravitationalSystem {
    pub fn apply_gravity(&self, elapsed_micros: u64, positioning_data: &PositioningData) {
      // TODO(acmcarther): Barnes-Hut Simulation
    }
  }

  impl PositioningData {
    /** 
     * Attempts to simplify the set of reference frames in the positioning system
     * 
     * This is accomplished by:
     * 1) Removing empty reference frames
     * 2) TODO(acmcarther): Merging reference frames that are close enough together to comfortably
     *    become a single frame
     * 3) TODO(acmcarther): Separating reference frames that are too spread out.
     * 
     * Simulation of nearby frames will not behave naturally until this is implemented.
     */
    pub fn optimize_reference_frames(&mut self) {
      if self.rf_mutation_state == ReferenceFrameMutationState::Frozen {
        return Err(ReferenceFrameOperationError::ReferenceFramesFrozen)
      }

      // Remove reference frames with no objects
      {
        let mut empty_frames = Vec::new();
        for (id, obj_list) : self.reference_frame_objects.iter() {
          if obj_list.is_empty() {
            empty_frames.push(id)
          }
        }
        for id : empty_frames {
          self.reference_frame_objects.remove(id);
          self.reference_frames.remove(id);
        }
      }

      // TODO(acmcarther): Attempt to merge nearby reference frames
      // TODO(acmcarther): Attempt to split reference frames that are large
      // These are deferred because the naive approach will immediately be too slow.
    }

    pub fn insert_object_into_frame(&mut self, rf_id: ReferenceFrameId, coord: SpatialCoordinate, massless: bool, inv_mass: ObjectMass) -> Result<ObjectId, ObjectOperationError> {
      // Verify references frame id consistency
      if self.rf_mutation_state == ReferenceFrameMutationState::Frozen {
        return Err(ObjectOperationError::ReferenceFramesFrozen)
      }

      // Sanity check on reference frame
      if !reference_frames.contains(rf_id) {
        return Err(ObjectOperationError::ReferenceFrameDoesNotExist)
      }

      // Verify object would be in bounds of reference frame
      for o_id : self.reference_frame_objects.get(rf_id).iter() {
        let o_pos = self.objects.get(o_id).coord;
        let d_o_x_m_sq = (o_pos.x_m - coord.x_m).powi(2);
        let d_o_y_m_sq = (o_pos.y_m - coord.y_m).powi(2);
        let d_o_z_m_sq = (o_pos.z_m - coord.z_m).powi(2);
        let d_o_m_sq = d_o_x_m_sq + d_o_y_m_sq + d_o_z_m_sq;

        if d_o_m_sq > APPROX_MAX_REFERENCE_FRAME_SIZE_M_SQUARED {
          return Err(ObjectOperationError::InsertFailedFromOverdistance)
        }
      }

      // Insert object into reference frame
      let o_id = self.next_o_id;
      self.next_oid = o_id + 1;
      self.objects.insert(o_id, Object {
        o_id; o_id,
        coord: coord,
        massless: bool,
        inv_mass: inv_mass,
      });
      self.reference_frame_objects.get_mut(rf_id).push(o_id);

      Ok(o_id)
    }

    /**
     * Disable manipulating reference frames directly.
     *
     * This is a safety measure to guarantee that ids collected after this point (until the next
     * thaw) will remain valid.
     */
    pub fn freeze_frames(&mut self) {
      self.rf_mutation_state = ReferenceFrameMutationState::Frozen
    }

    /**
     * Enable manipulating reference frames directly.
     *
     * This disables the insertion of objects, (whereby the callers may be holding onto invalid
     * reference frame ids).
     */
    pub fn unfreeze_frames(&mut self) {
      self.rf_mutation_state = ReferenceFrameMutationState::Unfrozen
    }

    /**
     * Insert a reference frame directly.
     *
     * This function does not prohibit the creation of overlapping reference frames (who should be
     * merged by calling `optimize_reference_frames` after objects have been inserted
     * 
     * It is safe to perform this action while reference frame mutation is frozen, as it cannot
     * invalidate existing reference frame ids.
     */
    pub fn create_reference_frame(&mut self, coord: GalacticCoordinate) -> ReferenceFrameId {
      let rf_id = self.next_rf_id;
      self.next_rf_id = rf_id + 1;

      self.reference_frames.insert(rf_id, ReferenceFrame {
        rf_id: rf_id,
        coord: coord,
      })
      self.reference_frame_objects.insert(rf_id, Vec::new());
      rf_id
    }

    /**
     * Attempts to merge RF2 into RF1, and recenter RF1.
     *
     * If the merge is possible, RF2 will be removed, and all objects will be moved into RF1.
     */
    fn combine_reference_frames(&mut self, rf_id_1: ReferenceFrameId, rf_id_2: ReferenceFrameId) -> Result<ReferenceFrameId, ReferenceFrameOperationError> {
      if self.rf_mutation_state == ReferenceFrameMutationState::Frozen {
        return Err(ReferenceFrameOperationError::MutationStateFrozen)
      }

      let rf_1 = self.reference_frames.get(rf_id_1).clone();
      let rf_2 = self.reference_frames.get(rf_id_2).clone();

      let d_rf_x_ls = rf_1.coord.x_ls - rf_2.coord.x_ls;
      let d_rf_y_ls = rf_1.coord.y_ls - rf_2.coord.y_ls;
      let d_rf_z_ls = rf_1.coord.z_ls - rf_2.coord.z_ls;

      // Attempt to exit from frame overdistance
      {
        let d_rf_x_ls_sq = d_rf_x_ls.powi(2);
        let d_rf_y_ls_sq = d_rf_y_ls.powi(2);
        let d_rf_z_ls_sq = d_rf_z_ls.powi(2);
        let d_rf_ls_sq = d_rf_x_ls_sq + d_rf_y_ls_sq + d_rf_z_ls_sq;
        if d_rf_ls_sq > APPROX_MAX_REFERENCE_FRAME_SIZE_LS_SQUARED {
          return Err(ReferenceFrameOperationError::CombinationFailedFromFrameOverdistance)
        }
      }
      let d_rf_x_m = d_rf_x_ls * M_PER_LS;
      let d_rf_y_m = d_rf_y_ls * M_PER_LS;
      let d_rf_z_m = d_rf_z_ls * M_PER_LS;

      // TODO(acmcarther): A more efficient algorithm
      // Attempt to exit from object overdistance
      {
        let rf_1_os = self.reference_frame_objects.get(rf_id_1)
        let rf_2_os = self.reference_frame_objects.get(rf_id_2)

        let rf_2_o_coords_from_rf_1 = rf_2_os.iter()
          .map(|o_id| self.objects.get(o_id).coord)
          .map(|pos| SpatialCoordinate {
            rf_id: rf_id_1,
            x_m: pos.x_m + d_rf_x_m,
            y_m: pos.y_m + d_rf_y_m,
            z_m: pos.z_m + d_rf_z_m,
          })
          .collect::<Vec<_>>()

        for rf_1_o_coord : self.reference_frame_objects.iter().map(|o_id| self.objects.get(o_id).coord) {
          for rf_2_o_coord : rf_2_o_coords_from_rf_1.iter() {
            let d_o_x_m_sq = (rf_1_o_coord.x_m - rf_2_o_coord.x_m).powi(2);
            let d_o_y_m_sq = (rf_1_o_coord.y_m - rf_2_o_coord.y_m).powi(2);
            let d_o_z_m_sq = (rf_1_o_coord.z_m - rf_2_o_coord.z_m).powi(2);
            let d_o_m_sq = d_o_x_m_sq + d_o_y_m_sq + d_o_z_m_sq;

            if d_o_m_sq > APPROX_MAX_REFERENCE_FRAME_SIZE_M_SQUARED {
              return Err(ReferenceFrameOperationError::CombinationFailedFromObjectOverdistance)
            }
          }
        }

        // Depopulate RF2 and remove it
        {
          let mut rf_1_objects = self.reference_frame_objects.get_mut(rf_1_id);
          for rf_2_o_id : rf_2_os.iter() {
            let mut rf_2_o = self.objects.get_mut(rf_2_o_id)
            rf_2_o.coord.rf_id = rf_id_1;
            rf_2_o.coord.x_m = rf_2_o.coord.x_m + d_rf_x_m;
            rf_2_o.coord.y_m = rf_2_o.coord.y_m + d_rf_y_m;
            rf_2_o.coord.z_m = rf_2_o.coord.z_m + d_rf_z_m;
            rf_1_objects.insert(rf_2_o);
          }
          self.reference_frame_objects.remove(rf_2_id);
          self.reference_frames.remove(rf_2_id);
        }

        // Recenter RF1
        {
          let mut min_x_m = 0.0;
          let mut max_x_m = 0.0;
          let mut min_y_m = 0.0;
          let mut max_y_m = 0.0;
          let mut min_z_m = 0.0;
          let mut max_z_m = 0.0;
          {
            for o_coord : rf_2_coords_from_rf_1 {
              if o_coord.x_m < min_x_m { min_x_m = o_coord.x_m; }
              if o_coord.x_m > max_x_m { max_x_m = o_coord.x_m; }
              if o_coord.y_m < min_y_m { min_y_m = o_coord.y_m; }
              if o_coord.y_m > max_y_m { max_y_m = o_coord.y_m; }
              if o_coord.z_m < min_z_m { min_z_m = o_coord.z_m; }
              if o_coord.z_m > max_z_m { max_z_m = o_coord.z_m; }
            }
            for o_coord : self.reference_frame_objects.iter().map(|o_id| self.objects.get(o_id).coord) {
              if o_coord.x_m < min_x_m { min_x_m = o_coord.x_m; }
              if o_coord.x_m > max_x_m { max_x_m = o_coord.x_m; }
              if o_coord.y_m < min_y_m { min_y_m = o_coord.y_m; }
              if o_coord.y_m > max_y_m { max_y_m = o_coord.y_m; }
              if o_coord.z_m < min_z_m { min_z_m = o_coord.z_m; }
              if o_coord.z_m > max_z_m { max_z_m = o_coord.z_m; }
            }
          }
          // How far rf1 should move (and the negative how far objects inside it should move)
          let rf_1_move_x_m = (max_x_m + min_x_m) / 2.0;
          let rf_1_move_y_m = (max_y_m + min_y_m) / 2.0;
          let rf_1_move_z_m = (max_z_m + min_z_m) / 2.0;
          let rf_1_move_x_ls = rf_move_x_m / M_PER_LS;
          let rf_1_move_y_ls = rf_move_y_m / M_PER_LS;
          let rf_1_move_z_ls = rf_move_z_m / M_PER_LS;

          // Move rf1
          {
            let mut rf_1 = self.reference_frames.get_mut(rf_id_1);
            rf_1.coord.x_ls = rf_1.coord.x_ls + rf_1_move_x_ls;
            rf_1.coord.y_ls = rf_1.coord.y_ls + rf_1_move_y_ls;
            rf_1.coord.z_ls = rf_1.coord.z_ls + rf_1_move_z_ls;
          }

          // Move objects inside of rf1 (the negative of the movement of the frame itself
          for o_id in self.reference_frame_objects.get(rf_1_id) {
            let mut o_pos = self.objects.get_mut(o_id).coord;
            o_pos.x_m = o_pos.x_m - rf_1_move_x_m;
            o_pos.y_m = o_pos.y_m - rf_1_move_y_m;
            o_pos.z_m = o_pos.z_m - rf_1_move_z_m;
          }
        }
      }
      Ok(rf_1)
    }

    pub fn get_local_position(&self, o_id: ObjectId) -> SpatialPosition {
      self.objects.get(o_id).coord.clone()
    }
  }
}

struct Universe {
  positioning_data: PositioningData,
}

mod spatial {
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
