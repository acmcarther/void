extern crate init;
extern crate spect;
extern crate spect_zcfg;

use spect::SpectServer;
use spect::SpectSubpageModuleManager;
use spect::SpectServerParams;

fn main() {
  init::init();

  let spect_server = SpectServer::new(
    SpectServerParams::default(),
    SpectSubpageModuleManager::new(vec![spect_zcfg::get_zcfg_subpage_module()]).unwrap(),
  );

  spect_server.run()
}
