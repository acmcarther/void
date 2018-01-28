#[macro_use]
extern crate log;
#[macro_use]
extern crate memoffset;
extern crate vk_buffer_support as vkbs;
extern crate vk_device_support as vkds;
extern crate vk_instance_support as vkis;
#[macro_use(do_or_die)]
extern crate vk_lite as vkl;
extern crate vk_pipeline_support as vkps;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

use std::ptr;

/** Dumps hardcoded x11-related extensions into a FeatureSpec */
fn x11_related_extension_spec() -> vkl::FeatureSpec {
  vkl::FeatureSpec {
    wanted: vec![
      "VK_EXT_acquire_xlib_display",
      //"VK_EXT_display_surface_counter",
      "VK_KHR_display",
      "VK_KHR_get_physical_device_properties2",
      "VK_KHR_get_surface_capabilities2",
      "VK_KHR_surface",
      //"VK_KHR_xcb_surface",
      "VK_KHR_xlib_surface",
      "VK_KHX_device_group_creation",
    ],
    required: vec!["VK_EXT_debug_report"],
  }
}

/** Dumps hardcoded x11-related layers into a FeatureSpec */
fn x11_related_layer_spec() -> vkl::FeatureSpec {
  vkl::FeatureSpec {
    wanted: vec![
      "VK_LAYER_LUNARG_core_validation",
      "VK_LAYER_LUNARG_parameter_validation",
    ],
    required: vec!["VK_LAYER_LUNARG_standard_validation"],
  }
}

#[repr(C, packed)]
struct TriangleData {
  pos: [f32; 2],
  color: [f32; 3],
}

pub struct VertexInputProps {
  pos_attr_desc: vk::VertexInputAttributeDescription,
  color_attr_desc: vk::VertexInputAttributeDescription,
  binding_description: vk::VertexInputBindingDescription,
}

pub struct TransferableVertexBuffer {
  vertex_input_props: VertexInputProps,
  transfer_buffer: vkbs::PreparedBuffer,
  buffer: vkbs::PreparedBuffer,
  buffer_size: u64,
}

pub fn make_vertex_buffer(
  device: &vkl::LDevice,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> vkl::RawResult<TransferableVertexBuffer> {
  let vertices = vec![
    TriangleData {
      pos: [0.0f32, -0.5f32],
      color: [1.0f32, 0.0f32, 0.0f32],
    },
    TriangleData {
      pos: [0.5f32, 0.5f32],
      color: [0.0f32, 1.0f32, 0.0f32],
    },
    TriangleData {
      pos: [-0.5f32, 0.5f32],
      color: [0.0f32, 0.0f32, 1.0f32],
    },
  ];

  let buffer_size = (std::mem::size_of::<TriangleData>() * vertices.len()) as u64;

  let vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory) = try!(vkbs::make_buffer(
    device,
    buffer_size,
    vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
    vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
    memory_properties
  ));

  unsafe {
    try!(device.bind_buffer_memory(&transfer_buffer, &transfer_device_memory));
    try!(device.map_data_to_memory(&transfer_device_memory, &vertices));
  }

  let vkbs::PreparedBuffer(buffer, device_memory) = try!(vkbs::make_buffer(
    device,
    buffer_size,
    vk::BUFFER_USAGE_TRANSFER_DST_BIT | vk::BUFFER_USAGE_VERTEX_BUFFER_BIT,
    vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
    memory_properties
  ));
  unsafe {
    try!(device.bind_buffer_memory(&buffer, &device_memory));
  }

  let pos_attr_desc = vk::VertexInputAttributeDescription {
    binding: 0,
    location: 0,
    format: vk::FORMAT_R32G32_SFLOAT,
    offset: offset_of!(TriangleData, pos) as u32,
  };

  let color_attr_desc = vk::VertexInputAttributeDescription {
    binding: 0,
    location: 1,
    format: vk::FORMAT_R32G32B32_SFLOAT,
    offset: offset_of!(TriangleData, color) as u32,
  };

  let binding_description = vk::VertexInputBindingDescription {
    binding: 0,
    stride: std::mem::size_of::<TriangleData>() as u32,
    inputRate: vk::VERTEX_INPUT_RATE_VERTEX, /* advance per vertex (instead of per instance) */
  };

  Ok(TransferableVertexBuffer {
    transfer_buffer: vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory),
    buffer: vkbs::PreparedBuffer(buffer, device_memory),
    vertex_input_props: VertexInputProps {
      pos_attr_desc: pos_attr_desc,
      color_attr_desc: color_attr_desc,
      binding_description: binding_description,
    },
    buffer_size: buffer_size,
  })
}

pub fn record_command_buffers(
  device: &vkl::LDevice,
  swapchain: &vkss::LoadedSwapchain,
  framebuffers: &Vec<vk::Framebuffer>,
  render_pass: &vk::RenderPass,
  vertex_buffer: &vk::Buffer,
  graphics_pipeline: &vk::Pipeline,
  command_buffers: &Vec<vk::CommandBuffer>,
) {
  for (idx, command_buffer) in command_buffers.iter().enumerate() {
    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
      sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
      pNext: ptr::null(),
      flags: vk::COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
      pInheritanceInfo: ptr::null(),
    };

    do_or_die!(vkl::util::dooy("start command buffer", &|| unsafe {
      device
        .ptrs()
        .BeginCommandBuffer(*command_buffer, &command_buffer_begin_info)
    }));

    {
      let clear_color = vk::ClearValue {
        color: vk::ClearColorValue {
          float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32],
        },
      };
      let render_pass_begin_info = vk::RenderPassBeginInfo {
        sType: vk::STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        pNext: ptr::null(),
        renderPass: *render_pass,
        framebuffer: *framebuffers.get(idx).unwrap(),
        renderArea: vk::Rect2D {
          offset: vk::Offset2D { x: 0, y: 0 },
          extent: vk::Extent2D {
            width: swapchain.surface_extent.width,
            height: swapchain.surface_extent.height,
          },
        },
        clearValueCount: 1,
        pClearValues: &clear_color,
      };

      unsafe {
        device.ptrs().CmdBeginRenderPass(
          *command_buffer,
          &render_pass_begin_info,
          vk::SUBPASS_CONTENTS_INLINE,
        );
        device.ptrs().CmdBindPipeline(
          *command_buffer,
          vk::PIPELINE_BIND_POINT_GRAPHICS,
          *graphics_pipeline,
        );
        let all_vertex_buffers = [*vertex_buffer];
        let all_buffer_offsets = [0];
        device.ptrs().CmdBindVertexBuffers(
          *command_buffer,
          0,
          1,
          all_vertex_buffers.as_ptr(),
          all_buffer_offsets.as_ptr(),
        );
        device.ptrs().CmdDraw(*command_buffer, 3, 1, 0, 0);
        device.ptrs().CmdEndRenderPass(*command_buffer);
      }
    }

    do_or_die!(vkl::util::dooy("end command buffer", &|| unsafe {
      device.ptrs().EndCommandBuffer(*command_buffer)
    }))
  }
}

pub struct VulkanTriangle {
  instance: vkl::LInstance,
  device: vkl::LDevice,
  debug_report_callback: vk::DebugReportCallbackEXT,
  device_spec: vkds::SelectedPhysicalDeviceSpec,
  swapchain: vkss::LoadedSwapchain,
  image_views: Vec<vk::ImageView>,
  render_pass: vk::RenderPass,
  vert_shader_module: vk::ShaderModule,
  frag_shader_module: vk::ShaderModule,
  transfer_buffer: vkbs::PreparedBuffer,
  buffer: vkbs::PreparedBuffer,
  pipeline_layout: vk::PipelineLayout,
  graphics_pipeline: vk::Pipeline,
  framebuffers: Vec<vk::Framebuffer>,
  gfx_command_pool: vk::CommandPool,
  transfer_command_pool_opt: Option<vk::CommandPool>,
  command_buffers: Vec<vk::CommandBuffer>,
  image_available_semaphore: vk::Semaphore,
  render_finished_semaphore: vk::Semaphore,
}


pub fn vulkan_triangle<'a, W: vkl::WindowSystemPlugin>(
  vulkan: &'a vkl::Vulkan,
  window_system_plugin: &mut W,
) -> VulkanTriangle {
  let enabled_extensions = do_or_die!(vulkan.select_extensions(x11_related_extension_spec()));
  let enabled_layers = do_or_die!(vulkan.select_layers(x11_related_layer_spec()));

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

  let v_surface = do_or_die!(window_system_plugin.create_surface(&instance));
  let (device_cfg, device_spec) = {
    let physical_devices = do_or_die!(instance.list_physical_devices());
    let devices_details = do_or_die!(vkds::CandidateDeviceDetails::inspect_devices(
      &instance,
      &physical_devices,
      &v_surface,
      &vec!["VK_KHR_swapchain"],
    ));
    let devices_queues_details =
      do_or_die!(vkds::CandidateDeviceQueueDetails::inspect_queue_families(
        &instance,
        &devices_details,
        &v_surface
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
  };

  info!("Device cfg: {:?}", device_cfg);

  let device = do_or_die!(vkds::make_logical_device(
    &instance,
    &device_cfg,
    &device_spec,
    &enabled_layers,
  ));

  let swapchain = do_or_die!(vkss::make_swapchain(&device, &device_spec, &v_surface));
  let swapchain_images = do_or_die!(unsafe { device.get_swapchain_images(&swapchain.swapchain) });
  let image_views = do_or_die!(vkss::make_image_views(
    &device,
    &swapchain_images,
    &swapchain
  ));

  let render_pass = do_or_die!(vkps::make_render_pass(&device, &swapchain));
  let pipeline_layout = do_or_die!(vkps::make_pipeline_layout(&device));

  let TransferableVertexBuffer {
    transfer_buffer,
    buffer,
    vertex_input_props,
    buffer_size,
  } = do_or_die!(make_vertex_buffer(&device, &device_spec.memory_properties));

  let vert_shader_module = do_or_die!(vkl::builtins::make_shader_module(
    &device,
    include_bytes!("../../bazel-genfiles/client/presentation/triangle_vert_shader.spv"),
  ));
  let frag_shader_module = do_or_die!(vkl::builtins::make_shader_module(
    &device,
    include_bytes!("../../bazel-genfiles/client/presentation/triangle_frag_shader.spv"),
  ));

  let graphics_pipeline = do_or_die!(vkps::make_graphics_pipeline(
    &device,
    &vert_shader_module,
    &frag_shader_module,
    vertex_input_props.pos_attr_desc,
    vertex_input_props.color_attr_desc,
    vertex_input_props.binding_description,
    &render_pass,
    &swapchain,
    &pipeline_layout
  ));

  let framebuffers = do_or_die!(vkbs::make_framebuffers(
    &device,
    &image_views,
    &swapchain,
    &render_pass
  ));

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

  // Perform device copy in either transfer queue, or graphics queue (if we must)
  {
    let copy_command_pool = transfer_command_pool_opt
      .as_ref()
      .unwrap_or(&gfx_command_pool);
    let queue_family_idx = device_spec
      .dedicated_transfer_queue_family_idx_opt
      .unwrap_or(device_spec.gfx_queue_family_idx);
    let queue = device.get_device_queue(queue_family_idx, 0 /* queueIdx */);
    do_or_die!(vkbs::copy_buffer(
      &device,
      copy_command_pool,
      &transfer_buffer.0, /* buffer */
      &buffer.0,          /* buffer */
      buffer_size,
      &queue
    ));
  }

  let command_buffers = do_or_die!(vkbs::make_command_buffers(
    &device,
    &gfx_command_pool,
    framebuffers.len() as u32
  ));


  record_command_buffers(
    &device,
    &swapchain,
    &framebuffers,
    &render_pass,
    &buffer.0, /* buffer */
    &graphics_pipeline,
    &command_buffers,
  );

  let image_available_semaphore = do_or_die!(vkl::builtins::make_semaphore(&device));
  let render_finished_semaphore = do_or_die!(vkl::builtins::make_semaphore(&device));

  VulkanTriangle {
    instance: instance,
    device: device,
    debug_report_callback: debug_report_callback,
    device_spec: device_spec,
    swapchain: swapchain,
    image_views: image_views,
    render_pass: render_pass,
    vert_shader_module: vert_shader_module,
    frag_shader_module: frag_shader_module,
    transfer_buffer: transfer_buffer,
    buffer: buffer,
    pipeline_layout: pipeline_layout,
    graphics_pipeline: graphics_pipeline,
    framebuffers: framebuffers,
    gfx_command_pool: gfx_command_pool,
    transfer_command_pool_opt: transfer_command_pool_opt,
    command_buffers: command_buffers,
    image_available_semaphore: image_available_semaphore,
    render_finished_semaphore: render_finished_semaphore,
  }
}

impl VulkanTriangle {
  pub fn draw_demo_frame(&self) {
    let mut image_index = 0;
    unsafe {
      let image_index = do_or_die!(vkl::util::loady("next image", &|a| {
        self.device.ptrs().AcquireNextImageKHR(
          self.device.logical_device,
          self.swapchain.swapchain,
          u64::max_value(),
          self.image_available_semaphore,
          0, /* vk_null_handle */
          a,
        )
      }));

      let wait_semaphores = [self.image_available_semaphore];
      let wait_stages = [vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];
      let signal_semaphores = [self.render_finished_semaphore];
      let submit_info = vk::SubmitInfo {
        sType: vk::STRUCTURE_TYPE_SUBMIT_INFO,
        pNext: ptr::null(),
        waitSemaphoreCount: 1,
        pWaitSemaphores: wait_semaphores.as_ptr(),
        pWaitDstStageMask: wait_stages.as_ptr(),
        commandBufferCount: 1,
        pCommandBuffers: self.command_buffers.get(image_index as usize).unwrap(),
        signalSemaphoreCount: 1,
        pSignalSemaphores: signal_semaphores.as_ptr(),
      };

      let queue = self.device.get_device_queue(
        self.device_spec.gfx_queue_family_idx,
        0, /* queue_index */
      );

      do_or_die!(vkl::util::dooy("queue submit", &|| {
        self
          .device
          .ptrs()
          .QueueSubmit(queue, 1, &submit_info, 0 /* vk_null_handle */)
      }));

      let swapchains = [self.swapchain.swapchain];
      let present_info_khr = vk::PresentInfoKHR {
        sType: vk::STRUCTURE_TYPE_PRESENT_INFO_KHR,
        pNext: ptr::null(),
        waitSemaphoreCount: 1,
        pWaitSemaphores: signal_semaphores.as_ptr(),
        swapchainCount: 1,
        pSwapchains: swapchains.as_ptr(),
        pImageIndices: &image_index,
        pResults: ptr::null_mut(),
      };

      do_or_die!(vkl::util::dooy("queue present", &|| {
        self.device.ptrs().QueuePresentKHR(queue, &present_info_khr)
      }));
    }
  }
}

impl Drop for VulkanTriangle {
  fn drop(&mut self) {
    do_or_die!(self.device.device_wait_idle());
    self
      .device
      .destroy_semaphore(self.render_finished_semaphore);
    self
      .device
      .destroy_semaphore(self.image_available_semaphore);
    self.device.destroy_command_pool(self.gfx_command_pool);
    if let Some(command_pool) = self.transfer_command_pool_opt {
      self.device.destroy_command_pool(command_pool);
    }
    for framebuffer in self.framebuffers.drain(..) {
      self.device.destroy_framebuffer(framebuffer);
    }
    self.device.destroy_pipeline(self.graphics_pipeline);
    self
      .device
      .destroy_buffer(self.transfer_buffer.0 /* buffer */);
    self.device.destroy_buffer(self.buffer.0 /* buffer */);
    self
      .device
      .free_memory(self.transfer_buffer.1 /* deviceMemory */);
    self.device.free_memory(self.buffer.1 /* deviceMemory */);
    self.device.destroy_pipeline_layout(self.pipeline_layout);
    self.device.destroy_shader_module(self.vert_shader_module);
    self.device.destroy_shader_module(self.frag_shader_module);
    self.device.destroy_render_pass(self.render_pass);
    for image_view in self.image_views.drain(..) {
      self.device.destroy_image_view(image_view);
    }

    self.device.destroy_swapchain(self.swapchain.swapchain);
    self
      .instance
      .destroy_debug_callback(self.debug_report_callback)

    // swapchain_params: does not need explicit destruction
    // capable_physical_device: does not need explicit destruction
    // device: Destroyed on drop
    // instance: Destroyed on drop
  }
}
