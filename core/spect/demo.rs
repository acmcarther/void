extern crate init;
extern crate spect;
extern crate spect_zcfg;

use spect_zcfg::ZcfgSpectDataRenderer;

fn main() {
  init::init();

  let mut data_renderer = ZcfgSpectDataRenderer::default();

  data_renderer.update_data();
  println!("{}", data_renderer.render());
}
