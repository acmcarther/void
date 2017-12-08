extern crate specs;

use specs::Component;
use specs::VecStorage;

/** The position of an abstract entity measured in meters from the origin */
pub struct Position {
  pub x_m: f64,
  pub y_m: f64,
  pub z_m: f64,
}

impl Component for Position {
  type Storage = VecStorage<Self>;
}

/** The velocity of an abstract entity measured in meters per second. */
pub struct Velocity {
  pub x_ms: f64,
  pub y_ms: f64,
  pub z_ms: f64,
}

impl Component for Velocity {
  type Storage = VecStorage<Self>;
}

/** 
 * The mass of an abstract entity, measured in inverse kilos.
 * 
 * Using the inverse makes infinite mass representable as zero, which is useful, while making zero
 * mass unrepresentable, which is unnecessary (just remove the component). Zero mass makes the sim
 * weird anyway.
 */
pub struct InverseMass {
  pub inverse_kilos: f64,
}

impl Component for InverseMass {
  type Storage = VecStorage<Self>;
}
