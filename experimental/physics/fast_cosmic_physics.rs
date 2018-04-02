extern crate chrono;
extern crate log;
extern crate octree;
extern crate rand;

use octree::ForwardTraversalResult;
use octree::NodeTraversalData;
use octree::OctreeInitParams;
use octree::OctreeRootNode;
use std::collections::BTreeMap;

const X_INDEX: usize = 0;
const Y_INDEX: usize = 1;
const Z_INDEX: usize = 2;
const NULL_SYSTEM: i64 = -1;
const AGGREGATE_SYSTEM: i64 = -2;

/** The complete cosmos, containing point masses representing system masses */
pub type SystemId = i64;

#[derive(Clone, Debug)]
pub struct PointMass {
  system_id: SystemId,
  coord: [f32; 3],
  mass: f32,
}

pub struct SystemData {
  params: SystemParams,
  last_coord: [f32; 3],
  velocity: [f32; 3],
  mass: f32,
}

pub struct SystemParams {
  coord: [f32; 3],
  velocity: [f32; 3],
  mass: f32,
}

pub struct Cosmos {
  params: CosmosParams,
  next_system_id: SystemId,
  systems: BTreeMap<SystemId, SystemData>,
  octree: OctreeRootNode<PointMass, PointMass>,
}

#[derive(Clone)]
pub struct CosmosParams {
  // (Width of space / distance) threshold below which to use the barnes-hut simplification schema
  // to calculate the force of gravity.
  simplified_calculation_theta: f32,
  // A flat multiplier applied to gravitational effects
  gravitational_constant: f32,
}

impl Default for PointMass {
  fn default() -> PointMass {
    PointMass {
      system_id: NULL_SYSTEM,
      coord: [0.0, 0.0, 0.0],
      mass: 0.0,
    }
  }
}

impl octree::AsCoord for PointMass {
  fn get_coord(&self) -> &[f32; 3] {
    &self.coord
  }
}

impl octree::UpdateCoord for PointMass {
  fn update_coord(&mut self, to: [f32; 3]) {
    self.coord = to;
  }
}

impl Cosmos {
  pub fn new(params: CosmosParams) -> Cosmos {
    Cosmos {
      params: params,
      next_system_id: 1i64,
      systems: BTreeMap::new(),
      octree: OctreeRootNode::<PointMass, PointMass>::new(OctreeInitParams::default()),
    }
  }

  pub fn add_system(&mut self, params: SystemParams) -> SystemId {
    let system_id = self.next_system_id;
    self.next_system_id += 1;

    self.octree.insert(PointMass {
      system_id: system_id,
      coord: params.coord.clone(),
      mass: params.mass.clone(),
    });

    self.systems.insert(
      system_id,
      SystemData {
        last_coord: params.coord.clone(),
        velocity: params.velocity.clone(),
        mass: params.mass.clone(),
        params: params,
      },
    );

    system_id
  }

  pub fn remove_system(&mut self, system_id: SystemId) -> Option<SystemData> {
    if !self.systems.contains_key(&system_id) {
      return None;
    }

    // UNWRAP: Known to exist from above guard
    let system_data = self.systems.remove(&system_id).unwrap();

    self.octree.remove(&system_data.last_coord);

    Some(system_data)
  }

  pub fn get_system_ids(&self) -> Vec<SystemId> {
    self.systems.keys().cloned().collect()
  }

  pub fn get_system_data<'a>(&'a self, system_id: &SystemId) -> Option<&'a SystemData> {
    self.systems.get(system_id)
  }

  pub fn tick(&mut self, dt_s: f32) {
    // Integrate velocity
    for mut system in self.systems.values_mut() {
      let new_coord = [
        system.last_coord[X_INDEX] * system.velocity[X_INDEX] * dt_s,
        system.last_coord[Y_INDEX] * system.velocity[X_INDEX] * dt_s,
        system.last_coord[Z_INDEX] * system.velocity[Z_INDEX] * dt_s,
      ];

      self.octree.update(&system.last_coord, new_coord.clone());
      system.last_coord = new_coord;
    }

    // Determine effect of gravity
    self
      .octree
      .in_place_map_reduce(&map_point_mass, &reduce_point_masses);

    let cosmic_params = self.params.clone();
    for mut system in self.systems.values_mut() {
      let last_coord: &[f32; 3] = &system.last_coord;
      let applied_velocity: [f32; 3] = self.octree.traverse_reduce(
        [0f32, 0f32, 0f32],
        &|traversal_data: NodeTraversalData<PointMass, PointMass>| {
          let mut coord_is_out_of_bounds: bool = traversal_data.coord_is_out_of_bounds(last_coord);
          if traversal_data.negate_specified_bouunds {
            coord_is_out_of_bounds = !coord_is_out_of_bounds;
          }

          if !coord_is_out_of_bounds {
            if traversal_data.is_leaf {
              // We're in this sample, but this is the finest level of detail. Need to use actual
              // data instead of metadata.

              let mut aggregate_result = [0f32, 0f32, 0f32];
              for entry in traversal_data.data {
                if &entry.coord == last_coord {
                  // Don't use our own location for calculations
                  continue;
                }

                let mass_product = system.mass * entry.mass;
                let distance = [
                  last_coord[X_INDEX] - entry.coord[X_INDEX],
                  last_coord[Y_INDEX] - entry.coord[Y_INDEX],
                  last_coord[Z_INDEX] - entry.coord[Z_INDEX],
                ];
                let distance_magnitude = ((distance[X_INDEX].powi(2) + distance[Y_INDEX].powi(2)
                  + distance[Z_INDEX].powi(2)) as f32)
                  .sqrt();
                let gravitational_effect = cosmic_params.gravitational_constant * mass_product
                  * (distance_magnitude.powi(2));

                aggregate_result[X_INDEX] += distance[X_INDEX] * gravitational_effect * dt_s;
                aggregate_result[Y_INDEX] += distance[Y_INDEX] * gravitational_effect * dt_s;
                aggregate_result[Z_INDEX] += distance[Z_INDEX] * gravitational_effect * dt_s;
              }
              return ForwardTraversalResult {
                should_continue: false,
                partial_result: aggregate_result,
              };
            } else {
              // We need to recurse further -- our own point mass is in this sample, so we cannot
              // accurately draw any conclusion
              return ForwardTraversalResult {
                should_continue: true,
                partial_result: [0f32, 0f32, 0f32],
              };
            }
          } else {
            // Need to decide if metadata's level of detail is good enough, or we need to recurse
            // further
            let distance = [
              last_coord[X_INDEX] - traversal_data.metadata.coord[X_INDEX],
              last_coord[Y_INDEX] - traversal_data.metadata.coord[Y_INDEX],
              last_coord[Z_INDEX] - traversal_data.metadata.coord[Z_INDEX],
            ];
            let distance_magnitude = ((distance[X_INDEX].powi(2) + distance[Y_INDEX].powi(2)
              + distance[Z_INDEX].powi(2)) as f32)
              .sqrt();
            let detail_factor = traversal_data.half_size[X_INDEX] / distance_magnitude;

            if detail_factor < cosmic_params.simplified_calculation_theta {
              // We don't need to recurse further -- we can use the metadata approximation
              let mass_product = system.mass * traversal_data.metadata.mass;
              let gravitational_effect =
                cosmic_params.gravitational_constant * mass_product * (distance_magnitude.powi(2));
              return ForwardTraversalResult {
                should_continue: false,
                partial_result: [
                  distance[X_INDEX] * gravitational_effect * dt_s,
                  distance[Y_INDEX] * gravitational_effect * dt_s,
                  distance[Z_INDEX] * gravitational_effect * dt_s,
                ],
              };
            } else {
              // The distance is either too small or the region too large for us to use the
              // current level of detail for our computation.

              if traversal_data.is_leaf {
                // We need to use the individual points in this sample since there is no finer
                // level of detail
                let mut aggregate_result = [0f32, 0f32, 0f32];
                for entry in traversal_data.data {
                  let mass_product = system.mass * entry.mass;
                  let distance = [
                    last_coord[X_INDEX] - entry.coord[X_INDEX],
                    last_coord[Y_INDEX] - entry.coord[Y_INDEX],
                    last_coord[Z_INDEX] - entry.coord[Z_INDEX],
                  ];
                  let distance_magnitude = ((distance[X_INDEX].powi(2) + distance[Y_INDEX].powi(2)
                    + distance[Z_INDEX].powi(2))
                    as f32)
                    .sqrt();
                  let gravitational_effect = cosmic_params.gravitational_constant * mass_product
                    * (distance_magnitude.powi(2));

                  aggregate_result[X_INDEX] += distance[X_INDEX] * gravitational_effect * dt_s;
                  aggregate_result[Y_INDEX] += distance[Y_INDEX] * gravitational_effect * dt_s;
                  aggregate_result[Z_INDEX] += distance[Z_INDEX] * gravitational_effect * dt_s;
                }
                return ForwardTraversalResult {
                  should_continue: false,
                  partial_result: aggregate_result,
                };
              } else {
                // Lets recurse another level
                return ForwardTraversalResult {
                  should_continue: true,
                  partial_result: [0f32, 0f32, 0f32],
                };
              }
            }
          }
        },
        &|left: [f32; 3], right: [f32; 3]| {
          [
            left[X_INDEX] + right[X_INDEX],
            left[Y_INDEX] + right[Y_INDEX],
            left[Z_INDEX] + right[Z_INDEX],
          ]
        },
      );

      system.velocity[X_INDEX] += applied_velocity[X_INDEX];
      system.velocity[Y_INDEX] += applied_velocity[Y_INDEX];
      system.velocity[Z_INDEX] += applied_velocity[Z_INDEX];
    }
  }
}

impl Default for CosmosParams {
  fn default() -> CosmosParams {
    CosmosParams {
      simplified_calculation_theta: 0.5,
      gravitational_constant: 10.0,
    }
  }
}

fn map_point_mass(i: &PointMass) -> PointMass {
  i.clone()
}

fn reduce_point_masses(i: PointMass, j: PointMass) -> PointMass {
  if i.mass == 0.0 {
    return j;
  } else if j.mass == 0.0 {
    return i;
  }

  let mass_sum = i.mass + j.mass;
  let i_mass_fraction = i.mass / mass_sum;
  let j_mass_fraction = j.mass / mass_sum;

  // Positive + Negative mass that exactly cancels? WTF?
  debug_assert!(mass_sum != 0.0);

  return PointMass {
    system_id: AGGREGATE_SYSTEM,
    coord: [
      i.coord[X_INDEX] * i_mass_fraction + j.coord[X_INDEX] * j_mass_fraction,
      i.coord[Y_INDEX] * i_mass_fraction + j.coord[Y_INDEX] * j_mass_fraction,
      i.coord[Z_INDEX] * i_mass_fraction + j.coord[Z_INDEX] * j_mass_fraction,
    ],
    mass: mass_sum,
  };
}
