extern crate init;
extern crate spect;
extern crate spect_zcfg;

use spect::SpectRenderableSubpage;
use spect_zcfg::ZcfgSpectRenderableSubpage;

fn main() {
  init::init();

  let mut zcfg_subpage = ZcfgSpectRenderableSubpage::default();

  zcfg_subpage.try_update_data(false /* force_update */);
  println!("{}", zcfg_subpage.render());
}
