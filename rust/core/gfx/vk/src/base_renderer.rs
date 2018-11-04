#[macro_use(do_or_die)]
use lite;
use device_support as ds;
use instance_support as is;
use pipeline_support as vkps;
use swapchain_support;
use vk_sys;
use std::marker::PhantomData;

pub struct BaseRenderer<'window> {
  pub vulkan: lite::Vulkan,
  pub config: BaseRendererConfig,
  pub instance: lite::LInstance,
  pub surface: vk_sys::SurfaceKHR,
  pub device_spec: ds::SelectedPhysicalDeviceSpec,
  pub device: lite::LDevice,
  pub swapchain: swapchain_support::LoadedSwapchain,
  pub image_views: Vec<vk_sys::ImageView>,
  pub render_pass: vk_sys::RenderPass,
  pub depth_format: vk_sys::Format,
  pub gfx_command_pool: vk_sys::CommandPool,
  pub transfer_command_pool_opt: Option<vk_sys::CommandPool>,
  pub debug_report_callback: vk_sys::DebugReportCallbackEXT,
  _window_lifetime: PhantomData<&'window ()>,
}

pub struct BaseRendererConfig {
  pub extension_spec: lite::FeatureSpec,
  pub layer_spec: lite::FeatureSpec,
}

impl<'window> BaseRenderer<'window> {
  pub fn new(
    vulkan: lite::Vulkan,
    window_system_plugin: &mut lite::WindowSystemPlugin<'window>,
    config: BaseRendererConfig,
  ) -> BaseRenderer<'window> {
    let enabled_extensions = do_or_die!(vulkan.select_extensions(config.extension_spec.clone()));
    let enabled_layers = do_or_die!(vulkan.select_layers(config.layer_spec.clone()));
    let instance = do_or_die!(is::make_instance(
      is::InstanceCfgBuilder::default().build().unwrap(),
      &enabled_extensions,
      &enabled_layers,
      &|a| vulkan.create_instance(a),
    ));

    let debug_report_callback = do_or_die!(lite::builtins::make_debug_report_callback(
      &instance,
      lite::builtins::vk_debug_report_callback_ext,
    ));

    // UNSAFE: Window lifetime encoded in renderer
    let surface = unsafe { do_or_die!(window_system_plugin.create_surface(&instance)) };

    let (device_cfg, device_spec) = select_device(&instance, &surface);
    debug!("Device cfg: {:?}", device_cfg);

    let device = do_or_die!(ds::make_logical_device(
      &instance,
      &device_cfg,
      &device_spec,
      &enabled_layers,
    ));

    let swapchain = do_or_die!(swapchain_support::make_swapchain(&device, &device_spec, &surface));
    let image_views = {
      // UNSAFE: Swapchain images destroyed on BaseRenderer drop.
      let swapchain_images =
        do_or_die!(unsafe { device.get_swapchain_images(&swapchain.swapchain) });
      do_or_die!(swapchain_support::make_image_views(
        &device,
        &swapchain_images,
        &swapchain
      ))
    };

    let depth_format = select_depth_format(&instance, &device_spec);
    let render_pass = do_or_die!(pipeline_support::make_render_pass(&device, depth_format, &swapchain));
    let gfx_command_pool = do_or_die!(lite::builtins::make_command_pool(
      &device,
      device_spec.gfx_queue_family_idx
    ));

    let transfer_command_pool_opt = if device_spec
      .dedicated_transfer_queue_family_idx_opt
      .is_some()
    {
      Some(do_or_die!(lite::builtins::make_command_pool(
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
  instance: &lite::LInstance,
  surface: &vk_sys::SurfaceKHR,
) -> (ds::LogicalDeviceCfg, ds::SelectedPhysicalDeviceSpec) {
  let physical_devices = do_or_die!(instance.list_physical_devices());
  let devices_details = do_or_die!(ds::CandidateDeviceDetails::inspect_devices(
    &instance,
    &physical_devices,
    &surface,
    &vec!["VK_KHR_swapchain"],
  ));
  let devices_queues_details =
    do_or_die!(ds::CandidateDeviceQueueDetails::inspect_queue_families(
      &instance,
      &devices_details,
      &surface
    ));

  let device_specs = ds::select_best_device_and_queue(devices_details, devices_queues_details);

  let mut device_cfg_builder = ds::LogicalDeviceCfgBuilder::default();
  if device_specs
    .dedicated_transfer_queue_family_idx_opt
    .is_some()
  {
    device_cfg_builder.transfer_queues(vec![ds::QueueCfg::default()]);
  }

  (device_cfg_builder.build().unwrap(), device_specs)
}

/** Pick the best depth format available. */
fn select_depth_format(
  instance: &lite::LInstance,
  device_spec: &ds::SelectedPhysicalDeviceSpec,
) -> vk_sys::Format {
  let candidates = vec![
    vk_sys::FORMAT_D32_SFLOAT,
    vk_sys::FORMAT_D32_SFLOAT_S8_UINT,
    vk_sys::FORMAT_D24_UNORM_S8_UINT,
  ];
  let tiling = vk_sys::IMAGE_TILING_OPTIMAL;
  let features = vk_sys::FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT;

  for candidate in candidates.iter() {
    let format_properties =
      instance.get_physical_device_format_properties(device_spec.physical_device, candidate);
    let linear_tiling_features_matches =
      format_properties.linearTilingFeatures & features == features;
    let optimal_tiling_features_matches =
      format_properties.optimalTilingFeatures & features == features;

    if tiling == vk_sys::IMAGE_TILING_LINEAR && linear_tiling_features_matches {
      return *candidate;
    }

    if tiling == vk_sys::IMAGE_TILING_OPTIMAL && optimal_tiling_features_matches {
      return *candidate;
    }
  }

  panic!("Vulkan detected no viable candidate for depth buffer formatting.");
}
