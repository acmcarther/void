#![feature(used)]

extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate zcfg;

use hyper::Result as HyperResult;

define_pub_cfg!(
  spect_server_port,
  u32,
  3663u32,
  "Port to run the spect server on"
);

struct SpectServerParams {
  port: u32,
}

struct SpectPageModule {
  address_suffix: String,
}

struct SpectServer {}

pub struct SpectRenderableSubpage {}

impl Default for SpectServerParams {
  fn default() -> SpectServerParams {
    SpectServerParams {
      port: spect_server_port::CONFIG.get_value(),
    }
  }
}

impl SpectServer {
  pub fn new(params: SpectServerParams, page_modules: Vec<SpectPageModule>) -> SpectServer {
    SpectServer {}
  }

  pub fn run(&mut self) -> HyperResult<()> {
    Ok(())
  }
}
