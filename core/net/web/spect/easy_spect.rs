#![feature(used)]
#[macro_use]
extern crate lazy_static;
extern crate spect;
extern crate spect_zcfg;
#[macro_use]
extern crate zcfg;

use std::thread;
use spect::SpectSubpageModule;
use spect::SpectServerParams;
use spect::SpectServer;
use spect::SpectSubpageModuleManager;

define_pub_cfg!(
  easy_spect_include_zcfg,
  bool,
  true,
  "Whether or not spect_zcfg should be autoincluded"
);

pub fn start(extra_modules: Vec<SpectSubpageModule>) {
  thread::spawn(move || {
    let mut modules = extra_modules;

    if easy_spect_include_zcfg::CONFIG.get_value() {
      modules.push(spect_zcfg::get_zcfg_subpage_module())
    }

    let spect_server = SpectServer::new(
      SpectServerParams::default(),
      SpectSubpageModuleManager::new(modules).unwrap(),
    );

    spect_server.run();
  });
}
