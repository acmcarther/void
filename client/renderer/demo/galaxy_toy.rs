extern crate cgmath;
extern crate chrono;
extern crate cosmic_physics as cp;
extern crate fern;
#[macro_use]
extern crate log;
#[macro_use]
extern crate memoffset;
extern crate png;
extern crate rand;
extern crate sdl2;
extern crate sdl2_vulkan_interop;
extern crate vk_buffer_support as vkbs;
extern crate vk_descriptor_support as vkdrs;
extern crate vk_device_support as vkds;
extern crate vk_instance_support as vkis;
#[macro_use(do_or_die)]
extern crate vk_lite as vkl;
extern crate vk_pipeline_support as vkps;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

use cgmath::Angle;
use rand::Rng;
use std::os::raw::c_void;
use std::ptr;

mod x11 {
  use vkl;

  /** Dumps hardcoded x11-related extensions into a FeatureSpec */
  pub fn extension_spec() -> vkl::FeatureSpec {
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
  pub fn layer_spec() -> vkl::FeatureSpec {
    vkl::FeatureSpec {
      wanted: vec![
        "VK_LAYER_LUNARG_core_validation",
        "VK_LAYER_LUNARG_parameter_validation",
      ],
      required: vec!["VK_LAYER_LUNARG_standard_validation"],
    }
  }
}

#[repr(C, packed)]
struct VertexData {
  pos: [f32; 3],
  tex: [f32; 2],
}

pub struct VertexInputProps {
  pos_attr_desc: vk::VertexInputAttributeDescription,
  tex_attr_desc: vk::VertexInputAttributeDescription,
  binding_description: vk::VertexInputBindingDescription,
}

pub struct VertexBufferDetails {
  vertex_input_props: VertexInputProps,
  buffer: vkbs::PreparedBuffer,
}

pub fn make_vertex_buffer(
  device: &vkl::LDevice,
  command_pool: &vk::CommandPool,
  queue: &vk::Queue,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> vkl::RawResult<VertexBufferDetails> {
  let vertices = vec![
    VertexData {
      pos: [-0.5f32, -0.5f32, 0.0f32],
      tex: [1.0f32, 0.0f32],
    },
    VertexData {
      pos: [0.5f32, -0.5f32, 0.0f32],
      tex: [0.0f32, 0.0f32],
    },
    VertexData {
      pos: [0.5f32, 0.5f32, 0.0f32],
      tex: [0.0f32, 1.0f32],
    },
    VertexData {
      pos: [-0.5f32, 0.5f32, 0.0f32],
      tex: [1.0f32, 1.0f32],
    },
  ];

  let buffer_size = (std::mem::size_of::<VertexData>() * vertices.len()) as u64;

  let vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory) = try!(vkbs::make_buffer(
    device,
    buffer_size,
    vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
    vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
    memory_properties
  ));

  unsafe {
    try!(device.bind_buffer_memory(&transfer_buffer, &transfer_device_memory));
    try!(device.map_vec_data_to_memory(&transfer_device_memory, &vertices));
  }

  let vkbs::PreparedBuffer(buffer, device_memory) = try!(vkbs::make_buffer(
    device,
    buffer_size,
    vk::BUFFER_USAGE_TRANSFER_DST_BIT | vk::BUFFER_USAGE_VERTEX_BUFFER_BIT,
    vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
    memory_properties
  ));
  unsafe {
    try!(device.bind_buffer_memory(&buffer, &device_memory));
  }

  let pos_attr_desc = vk::VertexInputAttributeDescription {
    binding: 0,
    location: 0,
    format: vk::FORMAT_R32G32B32_SFLOAT,
    offset: offset_of!(VertexData, pos) as u32,
  };

  let tex_attr_desc = vk::VertexInputAttributeDescription {
    binding: 0,
    location: 1,
    format: vk::FORMAT_R32G32_SFLOAT,
    offset: offset_of!(VertexData, tex) as u32,
  };

  let binding_description = vk::VertexInputBindingDescription {
    binding: 0,
    stride: std::mem::size_of::<VertexData>() as u32,
    inputRate: vk::VERTEX_INPUT_RATE_VERTEX, /* advance per vertex (instead of per instance) */
  };

  // Perform device copy in either transfer queue, or graphics queue (if we must)
  {
    do_or_die!(vkbs::copy_buffer(
      &device,
      command_pool,
      &transfer_buffer, /* buffer */
      &buffer,          /* buffer */
      buffer_size,
      queue
    ));
  }

  device.destroy_buffer(transfer_buffer);
  device.free_memory(transfer_device_memory);

  Ok(VertexBufferDetails {
    buffer: vkbs::PreparedBuffer(buffer, device_memory),
    vertex_input_props: VertexInputProps {
      pos_attr_desc: pos_attr_desc,
      tex_attr_desc: tex_attr_desc,
      binding_description: binding_description,
    },
  })
}

pub struct IndexBufferDetails {
  buffer: vkbs::PreparedBuffer,
  num_indexes: u32,
}

pub fn make_index_buffer(
  device: &vkl::LDevice,
  command_pool: &vk::CommandPool,
  queue: &vk::Queue,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> vkl::RawResult<IndexBufferDetails> {
  let indexes = vec![0u16, 1u16, 2u16, 2u16, 3u16, 0u16];

  let buffer_size = (std::mem::size_of::<u16>() * indexes.len()) as u64;

  let vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory) = try!(vkbs::make_buffer(
    device,
    buffer_size,
    vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
    vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
    memory_properties
  ));

  unsafe {
    try!(device.bind_buffer_memory(&transfer_buffer, &transfer_device_memory));
    try!(device.map_vec_data_to_memory(&transfer_device_memory, &indexes));
  }

  let vkbs::PreparedBuffer(buffer, device_memory) = try!(vkbs::make_buffer(
    device,
    buffer_size,
    vk::BUFFER_USAGE_TRANSFER_DST_BIT | vk::BUFFER_USAGE_INDEX_BUFFER_BIT,
    vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
    memory_properties
  ));
  unsafe {
    try!(device.bind_buffer_memory(&buffer, &device_memory));
  }

  // Perform device copy in either transfer queue, or graphics queue (if we must)
  {
    do_or_die!(vkbs::copy_buffer(
      &device,
      command_pool,
      &transfer_buffer, /* buffer */
      &buffer,          /* buffer */
      buffer_size,
      queue
    ));
  }

  device.destroy_buffer(transfer_buffer);
  device.free_memory(transfer_device_memory);

  Ok(IndexBufferDetails {
    num_indexes: indexes.len() as u32,
    buffer: vkbs::PreparedBuffer(buffer, device_memory),
  })
}

pub struct TextureImageDetails {
  image: vkbs::PreparedImage,
  image_view: vk::ImageView,
}

pub fn make_texture_image(
  device: &vkl::LDevice,
  command_pool: &vk::CommandPool,
  queue: &vk::Queue,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> vkl::RawResult<TextureImageDetails> {
  let png_bytes = include_bytes!("explosion.png");
  let img_decoder = png::Decoder::new(png_bytes as &[u8]);
  let (img_info, mut img_reader) = img_decoder.read_info().unwrap();
  let mut img_buf: Vec<u8> = vec![0; img_info.buffer_size()];
  img_reader.next_frame(&mut img_buf).unwrap();

  let buffer_size = (img_info.width * img_info.height * 4) as u64;

  let vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory) = try!(vkbs::make_buffer(
    device,
    buffer_size,
    vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
    vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
    memory_properties
  ));

  unsafe {
    try!(device.bind_buffer_memory(&transfer_buffer, &transfer_device_memory));
    try!(device.map_vec_data_to_memory(&transfer_device_memory, &img_buf));
  }

  let image_extent = vk::Extent3D {
    width: img_info.width,
    height: img_info.height,
    depth: 1,
  };
  let vkbs::PreparedImage(image, image_device_memory) = try!(vkbs::make_image(
    device,
    image_extent,
    vk::FORMAT_R8G8B8A8_UNORM,
    vk::IMAGE_TILING_OPTIMAL,
    vk::IMAGE_USAGE_TRANSFER_DST_BIT | vk::IMAGE_USAGE_SAMPLED_BIT,
    vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
    memory_properties
  ));

  unsafe {
    try!(device.bind_image_memory(&image, &image_device_memory, 0));
  }

  try!(vkbs::transition_image_layout(
    &device,
    &command_pool,
    &queue,
    &image,
    vk::FORMAT_R8G8B8A8_UNORM,
    vk::IMAGE_LAYOUT_UNDEFINED,
    vk::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL
  ));

  try!(vkbs::copy_buffer_into_image(
    &device,
    &transfer_buffer,
    &image,
    img_info.width,
    img_info.height,
    &command_pool,
    &queue
  ));

  try!(vkbs::transition_image_layout(
    &device,
    &command_pool,
    &queue,
    &image,
    vk::FORMAT_R8G8B8A8_UNORM,
    vk::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
    vk::IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
  ));

  device.destroy_buffer(transfer_buffer);
  device.free_memory(transfer_device_memory);

  let image_view = do_or_die!(make_texture_image_view(&device, &image /* image */));

  Ok(TextureImageDetails {
    image: vkbs::PreparedImage(image, image_device_memory),
    image_view: image_view,
  })
}

pub fn make_texture_image_view(
  device: &vkl::LDevice,
  image: &vk::Image,
) -> vkl::RawResult<vk::ImageView> {
  vkss::make_image_view(
    device,
    image,
    vk::FORMAT_R8G8B8A8_UNORM,
    vk::IMAGE_ASPECT_COLOR_BIT,
  )
}

pub fn make_texture_sampler(device: &vkl::LDevice) -> vkl::RawResult<vk::Sampler> {
  let sampler_create_info = vk::SamplerCreateInfo {
    sType: vk::STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    magFilter: vk::FILTER_LINEAR,
    minFilter: vk::FILTER_LINEAR,
    mipmapMode: vk::SAMPLER_MIPMAP_MODE_LINEAR,
    addressModeU: vk::SAMPLER_ADDRESS_MODE_REPEAT,
    addressModeV: vk::SAMPLER_ADDRESS_MODE_REPEAT,
    addressModeW: vk::SAMPLER_ADDRESS_MODE_REPEAT,
    mipLodBias: 0.0f32,
    anisotropyEnable: vk::TRUE,
    maxAnisotropy: 16.0f32,
    compareEnable: vk::TRUE,
    compareOp: vk::COMPARE_OP_ALWAYS,
    minLod: 0.0f32,
    maxLod: 0.0f32,
    borderColor: vk::BORDER_COLOR_INT_OPAQUE_BLACK,
    unnormalizedCoordinates: vk::FALSE,
  };

  device.create_sampler(&sampler_create_info)
}

pub struct DepthImageDetails {
  image: vkbs::PreparedImage,
  image_view: vk::ImageView,
}

pub fn select_supported_format(
  instance: &vkl::LInstance,
  device_spec: &vkds::SelectedPhysicalDeviceSpec,
  candidates: Vec<vk::Format>,
  tiling: vk::ImageTiling,
  features: vk::FormatFeatureFlags,
) -> vk::Format {
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

pub fn select_supported_depth_format(
  instance: &vkl::LInstance,
  device_spec: &vkds::SelectedPhysicalDeviceSpec,
) -> vk::Format {
  select_supported_format(
    instance,
    device_spec,
    vec![
      vk::FORMAT_D32_SFLOAT,
      vk::FORMAT_D32_SFLOAT_S8_UINT,
      vk::FORMAT_D24_UNORM_S8_UINT,
    ],
    vk::IMAGE_TILING_OPTIMAL,
    vk::FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT,
  )
}

pub fn make_depth_image(
  device: &vkl::LDevice,
  swapchain: &vkss::LoadedSwapchain,
  depth_format: vk::Format,
  command_pool: &vk::CommandPool,
  queue: &vk::Queue,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> vkl::RawResult<DepthImageDetails> {
  let prepared_image = try!(vkbs::make_image(
    device,
    vk::Extent3D {
      width: swapchain.surface_extent.width,
      height: swapchain.surface_extent.height,
      depth: 1,
    },
    depth_format,
    vk::IMAGE_TILING_OPTIMAL,
    vk::IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
    vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
    memory_properties
  ));
  unsafe {
    try!(device.bind_image_memory(
      &prepared_image.0, /* image */
      &prepared_image.1, /* deviceMemory */
      0
    ));
  }

  let image_view = try!(vkss::make_image_view(
    device,
    &prepared_image.0, /* image */
    depth_format,
    vk::IMAGE_ASPECT_DEPTH_BIT,
  ));

  try!(vkbs::transition_image_layout(
    &device,
    &command_pool,
    &queue,
    &prepared_image.0, /* image */
    depth_format,
    vk::IMAGE_LAYOUT_UNDEFINED,
    vk::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL
  ));

  Ok(DepthImageDetails {
    image: prepared_image,
    image_view: image_view,
  })
}

#[repr(C, packed)]
pub struct MVPUniform {
  view: cgmath::Matrix4<f32>,
  proj: cgmath::Matrix4<f32>,
}

pub struct UniformBufferDetails {
  buffer: vkbs::PreparedBuffer,
}

pub fn make_uniform_buffer(
  device: &vkl::LDevice,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> vkl::RawResult<UniformBufferDetails> {
  let buffer_size = std::mem::size_of::<MVPUniform>();
  let prepared_buffer = try!(vkbs::make_buffer(
    device,
    buffer_size as u64,
    vk::BUFFER_USAGE_UNIFORM_BUFFER_BIT,
    vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
    memory_properties,
  ));
  unsafe {
    try!(device.bind_buffer_memory(
      &prepared_buffer.0, /* buffer */
      &prepared_buffer.1  /* deviceMemory */
    ));
  }
  Ok(UniformBufferDetails {
    buffer: prepared_buffer,
  })
}

#[repr(C)]
struct PushConstant {
  model: cgmath::Matrix4<f32>,
}


pub fn record_command_buffer(
  device: &vkl::LDevice,
  swapchain: &vkss::LoadedSwapchain,
  render_pass: &vk::RenderPass,
  vertex_buffer: &vk::Buffer,
  index_buffer: &vk::Buffer,
  num_indexes: u32,
  graphics_pipeline: &vk::Pipeline,
  pipeline_layout: &vk::PipelineLayout,
  descriptor_sets: &Vec<vk::DescriptorSet>,
  framebuffer: &vk::Framebuffer,
  command_buffer: &vk::CommandBuffer,
  model_matrices: &Vec<cgmath::Matrix4<f32>>,
) {
  device.reset_command_buffer(command_buffer);

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
    let clear_depth = vk::ClearValue {
      depthStencil: vk::ClearDepthStencilValue {
        depth: 1.0f32,
        stencil: 0u32,
      },
    };
    let all_clears = [clear_color, clear_depth];
    let render_pass_begin_info = vk::RenderPassBeginInfo {
      sType: vk::STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
      pNext: ptr::null(),
      renderPass: *render_pass,
      framebuffer: *framebuffer,
      renderArea: vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: vk::Extent2D {
          width: swapchain.surface_extent.width,
          height: swapchain.surface_extent.height,
        },
      },
      clearValueCount: all_clears.len() as u32,
      pClearValues: all_clears.as_ptr(),
    };

    let push_constant_size = std::mem::size_of::<PushConstant>();
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
      device.ptrs().CmdBindDescriptorSets(
        *command_buffer,
        vk::PIPELINE_BIND_POINT_GRAPHICS,
        *pipeline_layout,
        0,                            /* firstSet */
        descriptor_sets.len() as u32, /* descriptorSetCount */
        descriptor_sets.as_ptr(),
        0, /* dynamicOffsetCount */
        ptr::null(),
      );
      device
        .ptrs()
        .CmdBindIndexBuffer(*command_buffer, *index_buffer, 0, vk::INDEX_TYPE_UINT16);

      for model_matrix in model_matrices {
        let push_constant = PushConstant {
          model: model_matrix.clone(),
        };
        device.ptrs().CmdPushConstants(
          *command_buffer,
          *pipeline_layout,
          vk::SHADER_STAGE_VERTEX_BIT,
          0, /* offset */
          push_constant_size as u32,
          &push_constant as *const PushConstant as *const c_void,
        );
        device
          .ptrs()
          .CmdDrawIndexed(*command_buffer, num_indexes, 1, 0, 0, 0);
      }

      device.ptrs().CmdEndRenderPass(*command_buffer);
    }

    do_or_die!(vkl::util::dooy("end command buffer", &|| unsafe {
      device.ptrs().EndCommandBuffer(*command_buffer)
    }))
  }
}


struct VulkanContext {
  first_frame_for_idxs: Vec<bool>,
  instance: vkl::LInstance,
  device: vkl::LDevice,
  debug_report_callback: vk::DebugReportCallbackEXT,
  device_spec: vkds::SelectedPhysicalDeviceSpec,
  swapchain: vkss::LoadedSwapchain,
  image_views: Vec<vk::ImageView>,
  render_pass: vk::RenderPass,
  vert_shader_module: vk::ShaderModule,
  frag_shader_module: vk::ShaderModule,
  uniform_buffer: vkbs::PreparedBuffer,
  vertex_buffer: vkbs::PreparedBuffer,
  index_buffer_details: IndexBufferDetails,
  texture_image: vkbs::PreparedImage,
  texture_image_view: vk::ImageView,
  depth_image: vkbs::PreparedImage,
  depth_image_view: vk::ImageView,
  texture_sampler: vk::Sampler,
  descriptor_pool: vk::DescriptorPool,
  descriptor_set_layouts: Vec<vk::DescriptorSetLayout>,
  descriptor_sets: Vec<vk::DescriptorSet>,
  pipeline_layout: vk::PipelineLayout,
  graphics_pipeline: vk::Pipeline,
  framebuffers: Vec<vk::Framebuffer>,
  command_buffer_fences: Vec<vk::Fence>,
  gfx_command_pool: vk::CommandPool,
  transfer_command_pool_opt: Option<vk::CommandPool>,
  command_buffers: Vec<vk::CommandBuffer>,
  image_available_semaphore: vk::Semaphore,
  render_finished_semaphore: vk::Semaphore,
}

impl VulkanContext {
  fn create<W: vkl::WindowSystemPlugin>(
    vulkan: &vkl::Vulkan,
    window_system_plugin: &mut W,
  ) -> VulkanContext {
    let enabled_extensions = do_or_die!(vulkan.select_extensions(x11::extension_spec()));
    let enabled_layers = do_or_die!(vulkan.select_layers(x11::layer_spec()));

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

      let device_specs =
        vkds::select_best_device_and_queue(devices_details, devices_queues_details);

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

    let depth_format = select_supported_depth_format(&instance, &device_spec);

    let render_pass = do_or_die!(vkps::make_render_pass(&device, depth_format, &swapchain));
    let descriptor_set_layouts = do_or_die!(vkdrs::make_descriptor_set_layouts(&device));
    let pipeline_layout = do_or_die!(vkps::make_pipeline_layout::<PushConstant>(
      &device,
      &descriptor_set_layouts
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
    let (
      vertex_buffer_details,
      index_buffer_details,
      uniform_buffer_details,
      texture_buffer_details,
      depth_buffer_details,
    ) = {
      let copy_command_pool = transfer_command_pool_opt
        .as_ref()
        .unwrap_or(&gfx_command_pool);
      let queue_family_idx = device_spec
        .dedicated_transfer_queue_family_idx_opt
        .unwrap_or(device_spec.gfx_queue_family_idx);
      let queue = device.get_device_queue(queue_family_idx, 0 /* queueIdx */);
      let gfx_queue =
        device.get_device_queue(device_spec.gfx_queue_family_idx, 0 /* queueIdx */);

      let vertex_buffer_details = do_or_die!(make_vertex_buffer(
        &device,
        &copy_command_pool,
        &queue,
        &device_spec.memory_properties
      ));
      let index_buffer_details = do_or_die!(make_index_buffer(
        &device,
        &copy_command_pool,
        &queue,
        &device_spec.memory_properties
      ));

      let uniform_buffer_details =
        do_or_die!(make_uniform_buffer(&device, &device_spec.memory_properties));

      let texture_buffer_details = do_or_die!(make_texture_image(
        &device,
        &gfx_command_pool,
        &gfx_queue,
        &device_spec.memory_properties
      ));

      let depth_buffer_details = do_or_die!(make_depth_image(
        &device,
        &swapchain,
        depth_format,
        &gfx_command_pool,
        &gfx_queue,
        &device_spec.memory_properties
      ));

      (
        vertex_buffer_details,
        index_buffer_details,
        uniform_buffer_details,
        texture_buffer_details,
        depth_buffer_details,
      )
    };

    let texture_sampler = do_or_die!(make_texture_sampler(&device));

    let descriptor_pool = do_or_die!(vkdrs::make_descriptor_pool(&device));

    let descriptor_sets = do_or_die!(vkdrs::make_descriptor_sets(
      &device,
      &descriptor_set_layouts,
      &descriptor_pool
    ));

    vkdrs::write_descriptor::<MVPUniform>(
      &device,
      &uniform_buffer_details.buffer.0, /* buffer */
      descriptor_sets.get(0).unwrap(),
      &texture_buffer_details.image_view,
      &texture_sampler,
    );

    let vert_shader_module = do_or_die!(vkl::builtins::make_shader_module(
      &device,
      include_bytes!("../../../bazel-genfiles/client/renderer/demo/star_vert_shader.spv"),
    ));
    let frag_shader_module = do_or_die!(vkl::builtins::make_shader_module(
      &device,
      include_bytes!("../../../bazel-genfiles/client/renderer/demo/star_frag_shader.spv"),
    ));

    let graphics_pipeline = do_or_die!(vkps::make_graphics_pipeline(
      &device,
      &vert_shader_module,
      &frag_shader_module,
      &vec![
        vertex_buffer_details.vertex_input_props.pos_attr_desc,
        vertex_buffer_details.vertex_input_props.tex_attr_desc,
      ],
      vertex_buffer_details.vertex_input_props.binding_description,
      &render_pass,
      &swapchain,
      &pipeline_layout
    ));

    let framebuffers = do_or_die!(vkbs::make_framebuffers(
      &device,
      &image_views,
      &depth_buffer_details.image_view,
      &swapchain,
      &render_pass
    ));

    let command_buffers = do_or_die!(vkbs::make_command_buffers(
      &device,
      &gfx_command_pool,
      framebuffers.len() as u32
    ));

    let image_available_semaphore = do_or_die!(vkl::builtins::make_semaphore(&device));
    let render_finished_semaphore = do_or_die!(vkl::builtins::make_semaphore(&device));

    let swapchain_image_count = swapchain.swapchain_image_count;
    let mut command_buffer_fences = Vec::new();
    let mut first_frame_for_idxs = Vec::new();
    for _ in 0..swapchain_image_count {
      command_buffer_fences.push(do_or_die!(vkl::builtins::make_fence(&device)));
      first_frame_for_idxs.push(true);
    }

    VulkanContext {
      first_frame_for_idxs: first_frame_for_idxs,
      instance: instance,
      device: device,
      debug_report_callback: debug_report_callback,
      device_spec: device_spec,
      swapchain: swapchain,
      image_views: image_views,
      render_pass: render_pass,
      vert_shader_module: vert_shader_module,
      frag_shader_module: frag_shader_module,
      uniform_buffer: uniform_buffer_details.buffer,
      index_buffer_details: index_buffer_details,
      vertex_buffer: vertex_buffer_details.buffer,
      texture_image: texture_buffer_details.image,
      texture_image_view: texture_buffer_details.image_view,
      depth_image: depth_buffer_details.image,
      depth_image_view: depth_buffer_details.image_view,
      texture_sampler: texture_sampler,
      descriptor_pool: descriptor_pool,
      descriptor_set_layouts: descriptor_set_layouts,
      descriptor_sets: descriptor_sets,
      pipeline_layout: pipeline_layout,
      graphics_pipeline: graphics_pipeline,
      framebuffers: framebuffers,
      gfx_command_pool: gfx_command_pool,
      command_buffer_fences: command_buffer_fences,
      transfer_command_pool_opt: transfer_command_pool_opt,
      command_buffers: command_buffers,
      image_available_semaphore: image_available_semaphore,
      render_finished_semaphore: render_finished_semaphore,
    }
  }

  pub fn update_uniform_buffer(&self) {
    let view = cgmath::Matrix4::<f32>::look_at(
      cgmath::Point3::<f32>::new(100.0f32, 100.0f32, 100.0f32),
      cgmath::Point3::<f32>::new(0.0f32, 0.0f32, 0.0f32),
      cgmath::Vector3::<f32>::new(0.0f32, 0.0f32, 1.0f32),
    );
    let mut proj = cgmath::perspective(
      (cgmath::Rad::<f32>::turn_div_4() / 2.0f32),
      ((self.swapchain.surface_extent.width as f32)
        / (self.swapchain.surface_extent.height as f32)),
      0.1f32,
      10000.0f32,
    );
    proj.y.y = proj.y.y * -1.0f32;

    let new_uniform = MVPUniform {
      view: view,
      proj: proj,
    };

    unsafe {
      do_or_die!(
        self
          .device
          .map_data_to_memory(&self.uniform_buffer.1 /* deviceMemory */, &new_uniform)
      );
    }
  }

  pub fn draw_demo_frame(&mut self, locs: &Vec<[f64; 4]>) {
    unsafe {
      self.update_uniform_buffer();
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
      let command_buffer = self.command_buffers.get(image_index as usize).unwrap();
      let framebuffer = self.framebuffers.get(image_index as usize).unwrap();
      let command_buffer_fence = self
        .command_buffer_fences
        .get(image_index as usize)
        .unwrap();
      let mut model_matrices = Vec::new();
      for loc in locs {
        model_matrices.push(
          cgmath::Matrix4::<f32>::from_translation(cgmath::Vector3::<f32>::new(
            loc[0] as f32,
            loc[1] as f32,
            loc[2] as f32,
          )) * cgmath::Matrix4::<f32>::from_scale((loc[3] / 10.0) as f32),
        );
      }

      {
        let first_frame_for_idx = *self.first_frame_for_idxs.get(image_index as usize).unwrap();
        if !first_frame_for_idx {
          unsafe {
            let all_fences = [*command_buffer_fence];
            do_or_die!(vkl::util::dooy("wait for fences", &|| {
              self.device.ptrs().WaitForFences(
                self.device.logical_device,
                1,
                all_fences.as_ptr(),
                vk::TRUE,  /* wait all */
                100000000, /* ns */
              )
            }));
            do_or_die!(vkl::util::dooy("reset fences", &|| {
              self
                .device
                .ptrs()
                .ResetFences(self.device.logical_device, 1, all_fences.as_ptr())
            }));
          }
        } else {
          let first_frame_for_idx = self
            .first_frame_for_idxs
            .get_mut(image_index as usize)
            .unwrap();
          *first_frame_for_idx = false;
        }
      }


      record_command_buffer(
        &self.device,
        &self.swapchain,
        &self.render_pass,
        &self.vertex_buffer.0,               /* buffer */
        &self.index_buffer_details.buffer.0, /* buffer */
        self.index_buffer_details.num_indexes,
        &self.graphics_pipeline,
        &self.pipeline_layout,
        &self.descriptor_sets,
        &framebuffer,
        &command_buffer,
        &model_matrices,
      );

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
          .QueueSubmit(queue, 1, &submit_info, *command_buffer_fence)
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

impl Drop for VulkanContext {
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
    self.device.destroy_sampler(self.texture_sampler);
    self.device.destroy_image_view(self.texture_image_view);
    self.device.destroy_image_view(self.depth_image_view);
    self
      .device
      .destroy_buffer(self.uniform_buffer.0 /* buffer */);
    self
      .device
      .destroy_buffer(self.index_buffer_details.buffer.0 /* buffer */);
    self
      .device
      .destroy_buffer(self.vertex_buffer.0 /* buffer */);
    self.device.destroy_image(self.texture_image.0 /* image */);
    self.device.destroy_image(self.depth_image.0 /* image */);
    self
      .device
      .free_memory(self.depth_image.1 /* deviceMemory */);
    self
      .device
      .free_memory(self.texture_image.1 /* deviceMemory */);
    self
      .device
      .free_memory(self.uniform_buffer.1 /* deviceMemory */);
    self
      .device
      .free_memory(self.index_buffer_details.buffer.1 /* deviceMemory */);
    self
      .device
      .free_memory(self.vertex_buffer.1 /* deviceMemory */);
    self.device.destroy_descriptor_pool(self.descriptor_pool);
    for descriptor_set_layout in self.descriptor_set_layouts.drain(..) {
      self
        .device
        .destroy_descriptor_set_layout(descriptor_set_layout)
    }
    self.device.destroy_pipeline_layout(self.pipeline_layout);
    self.device.destroy_shader_module(self.vert_shader_module);
    self.device.destroy_shader_module(self.frag_shader_module);
    self.device.destroy_render_pass(self.render_pass);
    for command_buffer_fence in self.command_buffer_fences.drain(..) {
      self.device.destroy_fence(command_buffer_fence);
    }
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

fn main() {
  fern::Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message
        ))
    })
    // Add blanket level filter -
    .level(log::LogLevelFilter::Debug)
    // - and per-module overrides
    .chain(std::io::stdout())
    // Apply globally
    .apply()
    .unwrap();

  const POSITION_CEILING: f64 = 100.0;
  const VELOCITY_CEILING: f64 = 5.0;
  const STAR_MASS: f64 = 50.0f64;
  const PLANET_MASS: f64 = 5.0f64;

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let mut window = video_subsystem
    .window("rust-sdl2 demo", 800, 600)
    .position_centered()
    .vulkan()
    .build()
    .unwrap();
  // TODO(acmcarther): This seems unpleasant and brittle-ish.
  let mut sdl_window_system_plugin = sdl2_vulkan_interop::SdlWindowSystemPlugin::new(&mut window);
  let vulkan = vkl::Vulkan::new("libvulkan.so.1");
  let mut vulkan_context = VulkanContext::create(&vulkan, &mut sdl_window_system_plugin);

  let (state_send, state_recv) = std::sync::mpsc::channel();

  std::thread::spawn(move || {
    let mut grid = cp::CelestialGrid::new();
    let mut rng = rand::thread_rng();
    let cosmic_params = cp::CosmicParams {
      gravitational_constant: 10.0,
    };

    for _ in 0..30 {
      grid.insert_system(
        cp::CelestialVector {
          x: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
          y: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
          z: 0.0f64,
        },
        cp::CelestialVector {
          x: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
          y: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
          z: 0.0f64,
        },
        STAR_MASS,
      );
    }

    for _ in 0..100 {
      grid.insert_system(
        cp::CelestialVector {
          x: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
          y: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
          z: 0.0f64,
        },
        cp::CelestialVector {
          x: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
          y: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
          z: 0.0f64,
        },
        PLANET_MASS,
      );
    }


    for tick in 0..100000000 {
      grid.tick_celestial_grid(&cosmic_params, 900u64);

      let mut snap_ents = Vec::new();
      for id in grid.get_system_ids().iter() {
        if let Some(system_details) = grid.get_system_details(id.clone()) {
          snap_ents.push([
            system_details.coords.x.clone(),
            system_details.coords.y.clone(),
            system_details.coords.z.clone(),
            system_details.mass.clone(),
          ]);
        }
      }
      state_send.send(snap_ents).unwrap();

      if tick % 10 == 0 {
        for id in grid.get_system_ids().iter() {
          let mut remove_system = false;
          let mut system_mass = 0.0;
          if let Some(system_details) = grid.get_system_details(id.clone()) {
            if system_details.coords.x > 2_000.0 || system_details.coords.x < -2_000.0 {
              remove_system = true;
              system_mass = *system_details.mass;
            } else if system_details.coords.y > 2_000.0 || system_details.coords.y < -2_000.0 {
              remove_system = true;
              system_mass = *system_details.mass;
            }
          }
          if remove_system {
            println!("removing system: {}", id);
            grid.remove_system(id.clone());
            grid.insert_system(
              cp::CelestialVector {
                x: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
                y: ((POSITION_CEILING * rng.gen::<f64>()) - (POSITION_CEILING / 2.0)),
                z: 0.0f64,
              },
              cp::CelestialVector {
                x: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
                y: ((VELOCITY_CEILING * rng.gen::<f64>()) - (VELOCITY_CEILING / 2.0)),
                z: 0.0f64,
              },
              system_mass,
            );
          }
        }
      }
    }
  });

  let mut event_pump = sdl_context.event_pump().unwrap();

  let mut last_recv = None;
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. }
        | sdl2::event::Event::KeyDown {
          keycode: Some(sdl2::keyboard::Keycode::Escape),
          ..
        } => break 'running,
        _ => {},
      }
    }

    loop {
      let rcv = state_recv.try_recv();

      let exit_state_poll = match rcv {
        Ok(state) => {
          last_recv = Some(state);
          false
        },
        Err(std::sync::mpsc::TryRecvError::Empty) => true,
        Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("main thread hung up!"),
      };
      if exit_state_poll {
        break;
      }
    }

    if last_recv.is_some() {
      vulkan_context.draw_demo_frame(last_recv.as_ref().unwrap());
    }
  }
}
