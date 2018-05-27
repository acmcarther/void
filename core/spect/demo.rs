extern crate init;
extern crate spect;
extern crate spect_zcfg;

use spect::SpectServer;
use spect::SpectPageModuleManager;
use spect::SpectServerParams;

fn main() {
  init::init();

  let spect_server = SpectServer::new(
    SpectServerParams::default(),
    SpectPageModuleManager::new(vec![spect_zcfg::get_zcfg_page_module()]).unwrap(),
  );

  spect_server.run().unwrap()
}
