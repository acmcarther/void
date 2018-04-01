#![feature(used)]
extern crate geometry;
extern crate icosphere;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rand;
#[macro_use]
extern crate zcfg;

use geometry::Mesh;

mod flags {}

pub struct PlanetGenConfig {}

pub struct PlanetGen {
  config: PlanetGenConfig,
}

pub struct PlanetSimConfig {}

pub struct PlanetSim {
  config: PlanetSimConfig,
  planet: Planet,
}

pub struct Planet {
  mesh: Mesh,
}

impl Default for PlanetGenConfig {
  fn default() -> PlanetGenConfig {
    PlanetGenConfig {}
  }
}

impl PlanetGen {
  pub fn new(config: PlanetGenConfig) -> PlanetGen {
    PlanetGen { config: config }
  }

  pub fn smooth(&self) -> Planet {
    Planet {
      mesh: icosphere::icosphere(5 /* iterations */),
    }
  }
}

impl Default for PlanetSimConfig {
  fn default() -> PlanetSimConfig {
    PlanetSimConfig {}
  }
}

impl PlanetSim {
  pub fn new(config: PlanetSimConfig, planet: Planet) -> PlanetSim {
    PlanetSim {
      config: config,
      planet: planet,
    }
  }
}
