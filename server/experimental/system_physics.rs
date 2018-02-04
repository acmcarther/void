type ObjectId = u64;
struct Cosmos {
  system_grids: HashMap<SystemId, SystemGrid>,
  celestial_grid: CelestialGrid,
}


struct SystemGrid {
  system_id: SystemId,
  system_mass: CeleplanetaryMass,
  object_coordinates: HashMap<ObjectId, SystemVector>,
  object_trajectories: HashMap<ObjectId, SystemVector>,
  object_celeplanetary_masses: HashMap<SystemId, CeleplanetaryMass>,
}

struct Object {
  object_id: ObjectId,
  parent_system: SystemId,
  mass: GameplayMass,
}

/** A spatial vector in Terestrefundamental units. */
struct SystemVector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

struct GameplayMass {
  cp_m: i64 /* celeplanetary_mass:
                 (  1_000_000_000_000_000_000): Galactic center black hole
                 (     10_000_000_000_000_000): Small stellar black hole mass
                 (      2_000_000_000_000_000): Solar Mass
                 (          2_000_000_000_000): Jupiter Mass
                 (              1_000_000_000): Earth Mass
                 (              1_000_000_000): Earth Mass
                 (                 37_000_000): Lunar Mass
                 (                    500_000): Ceres Mass
                 (                      1_000): ~200km asteroid mass */,
  ot_m: i64 /* orbitoterestrial_mass:
                 ( 10_000_000_000_000_000_000): ~30km asteroid mass
                 (     10_000_000_000_000_000): ~3km Rosetta comet mass
                 (          6_000_000_000_000): Pyramid of Giza mass
                 (            700_000_000_000): Heaviest conventional building mass
                 (             50_000_000_000): Heavy seafaring vessel mass
                 (              1_000_000_000): Small spacecraft mass
                 (                 50_000_000): Tank mass
                 (                     70_000): Human mass
                 (                     15_000): Dog mass
                 (                        100): A fruit's mass */,
  f_m: i32 /* fundamental_mass:
                 (  1_000_000_000): Smallest human scale mass,
                 (        500_000): Compound mass,
                 (          1_100): Atomic mass
                 (             17): Fundamental particle mass */,
}

