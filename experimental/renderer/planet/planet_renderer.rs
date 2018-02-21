extern crate renderer;
extern crate vk_device_support as vkds;
extern crate vk_instance_support as vkis;
#[macro_use(do_or_die)]
extern crate vk_lite as vkl;
extern crate vk_pipeline_support as vkps;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;
#[macro_use]
use renderer::BaseRenderer;

pub struct PlanetRenderer<'window> {
  base_renderer: BaseRenderer<'window>,
}

impl<'window> PlanetRenderer<'window> {
  pub fn new(base_renderer: BaseRenderer<'window>) -> PlanetRenderer<'window> {
    // TODO(acmcarther): Perform initialization specific to planet demo
    PlanetRenderer {
      base_renderer: base_renderer,
    }
  }
}
