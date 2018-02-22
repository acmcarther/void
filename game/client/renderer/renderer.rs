#[macro_use(log, debug)]
extern crate log;
extern crate vk_device_support as vkds;
extern crate vk_instance_support as vkis;
#[macro_use(do_or_die)]
extern crate vk_lite as vkl;
extern crate vk_pipeline_support as vkps;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

use std::marker::PhantomData;

pub struct BaseRenderer<'window> {
  pub vulkan: vkl::Vulkan,
  pub config: BaseRendererConfig,
  pub instance: vkl::LInstance,
  pub surface: vk::SurfaceKHR,
  pub device_spec: vkds::SelectedPhysicalDeviceSpec,
  pub device: vkl::LDevice,
  pub swapchain: vkss::LoadedSwapchain,
  pub image_views: Vec<vk::ImageView>,
  pub render_pass: vk::RenderPass,
  pub depth_format: vk::Format,
  pub gfx_command_pool: vk::CommandPool,
  pub transfer_command_pool_opt: Option<vk::CommandPool>,
  pub debug_report_callback: vk::DebugReportCallbackEXT,
  _window_lifetime: PhantomData<&'window ()>,
}

pub struct BaseRendererConfig {
  pub extension_spec: vkl::FeatureSpec,
  pub layer_spec: vkl::FeatureSpec,
}

impl<'window> BaseRenderer<'window> {
  pub fn new(
    vulkan: vkl::Vulkan,
    window_system_plugin: &mut vkl::WindowSystemPlugin<'window>,
    config: BaseRendererConfig,
  ) -> BaseRenderer<'window> {
    let enabled_extensions = do_or_die!(vulkan.select_extensions(config.extension_spec.clone()));
    let enabled_layers = do_or_die!(vulkan.select_layers(config.layer_spec.clone()));
    let instance = do_or_die!(vkis::make_instance(
      vkis::InstanceCfgBuilder::default().build().unwrap(),
      &enabled_extensions,
      &enabled_layers,
      &|a| vulkan.create_instance(a),
    ));

    let debug_report_callback = do_or_die!(vkl::builtins::make_debug_report_callback(
      &instance,
      vkl::builtins::vk_debug_report_callback_ext,
    ));

    // UNSAFE: Window lifetime encoded in renderer
    let surface = unsafe { do_or_die!(window_system_plugin.create_surface(&instance)) };

    let (device_cfg, device_spec) = select_device(&instance, &surface);
    debug!("Device cfg: {:?}", device_cfg);

    let device = do_or_die!(vkds::make_logical_device(
      &instance,
      &device_cfg,
      &device_spec,
      &enabled_layers,
    ));

    let swapchain = do_or_die!(vkss::make_swapchain(&device, &device_spec, &surface));
    let image_views = {
      // UNSAFE: Swapchain images destroyed on BaseRenderer drop.
      let swapchain_images =
        do_or_die!(unsafe { device.get_swapchain_images(&swapchain.swapchain) });
      do_or_die!(vkss::make_image_views(
        &device,
        &swapchain_images,
        &swapchain
      ))
    };

    let depth_format = select_depth_format(&instance, &device_spec);
    let render_pass = do_or_die!(vkps::make_render_pass(&device, depth_format, &swapchain));
    let gfx_command_pool = do_or_die!(vkl::builtins::make_command_pool(
      &device,
      device_spec.gfx_queue_family_idx
    ));

    let transfer_command_pool_opt = if device_spec
      .dedicated_transfer_queue_family_idx_opt
      .is_some()
    {
      Some(do_or_die!(vkl::builtins::make_command_pool(
        &device,
        device_spec.dedicated_transfer_queue_family_idx_opt.unwrap()
      )))
    } else {
      None
    };

    BaseRenderer {
      vulkan: vulkan,
      config: config,
      instance: instance,
      surface: surface,
      device_spec: device_spec,
      swapchain: swapchain,
      image_views: image_views,
      device: device,
      render_pass: render_pass,
      depth_format: depth_format,
      gfx_command_pool: gfx_command_pool,
      transfer_command_pool_opt: transfer_command_pool_opt,
      debug_report_callback: debug_report_callback,
      _window_lifetime: PhantomData,
    }
  }
}

impl<'window> Drop for BaseRenderer<'window> {
  fn drop(&mut self) {
    do_or_die!(self.device.device_wait_idle());
    self.device.destroy_render_pass(self.render_pass);
    for image_view in self.image_views.drain(..) {
      self.device.destroy_image_view(image_view);
    }

    self.device.destroy_swapchain(self.swapchain.swapchain);
    self
      .instance
      .destroy_debug_callback(self.debug_report_callback)

    // device: Destroyed on drop
    // instance: Destroyed on drop
  }
}

/** Uses instance and surface info to pick a device and provide its details. */
fn select_device(
  instance: &vkl::LInstance,
  surface: &vk::SurfaceKHR,
) -> (vkds::LogicalDeviceCfg, vkds::SelectedPhysicalDeviceSpec) {
  let physical_devices = do_or_die!(instance.list_physical_devices());
  let devices_details = do_or_die!(vkds::CandidateDeviceDetails::inspect_devices(
    &instance,
    &physical_devices,
    &surface,
    &vec!["VK_KHR_swapchain"],
  ));
  let devices_queues_details =
    do_or_die!(vkds::CandidateDeviceQueueDetails::inspect_queue_families(
      &instance,
      &devices_details,
      &surface
    ));

  let device_specs = vkds::select_best_device_and_queue(devices_details, devices_queues_details);

  let mut device_cfg_builder = vkds::LogicalDeviceCfgBuilder::default();
  if device_specs
    .dedicated_transfer_queue_family_idx_opt
    .is_some()
  {
    device_cfg_builder.transfer_queues(vec![vkds::QueueCfg::default()]);
  }

  (device_cfg_builder.build().unwrap(), device_specs)
}

/** Pick the best depth format available. */
fn select_depth_format(
  instance: &vkl::LInstance,
  device_spec: &vkds::SelectedPhysicalDeviceSpec,
) -> vk::Format {
  let candidates = vec![
    vk::FORMAT_D32_SFLOAT,
    vk::FORMAT_D32_SFLOAT_S8_UINT,
    vk::FORMAT_D24_UNORM_S8_UINT,
  ];
  let tiling = vk::IMAGE_TILING_OPTIMAL;
  let features = vk::FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT;

  for candidate in candidates.iter() {
    let format_properties =
      instance.get_physical_device_format_properties(device_spec.physical_device, candidate);
    let linear_tiling_features_matches =
      format_properties.linearTilingFeatures & features == features;
    let optimal_tiling_features_matches =
      format_properties.optimalTilingFeatures & features == features;

    if tiling == vk::IMAGE_TILING_LINEAR && linear_tiling_features_matches {
      return *candidate;
    }

    if tiling == vk::IMAGE_TILING_OPTIMAL && optimal_tiling_features_matches {
      return *candidate;
    }
  }

  panic!("Vulkan detected no viable candidate for depth buffer formatting.");
}
