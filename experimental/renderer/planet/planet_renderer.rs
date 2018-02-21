extern crate renderer;
extern crate vk_device_support as vkds;
extern crate vk_instance_support as vkis;
#[macro_use(do_or_die)]
extern crate vk_lite as vkl;
extern crate vk_pipeline_support as vkps;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

use std::collections::HashMap;
use renderer::BaseRenderer;
use std::marker::PhantomData;

/** A Planet-demo specific renderer. */
pub struct PlanetRenderer<'window> {
  base_renderer: BaseRenderer<'window>,
}

/**
 * A "push constant" for supplying a model matrix.
 *
 * Push constants are small bits of state that can be efficiently bound during command buffer
 * construction. They're useful for a form of "instanced rendering" where the same model is
 * rerendered many times with only a small difference
 */
#[repr(C)]
struct ModelPushConstant {
  model: cgmath::Matrix4<f32>,
}

/**
 * The set of buffers containing persistent render state.
 *
 * Currently contains the uniform buffer (not expected to change frequently) and the depth buffer.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
unsafe struct PersistentBuffers {
  uniform_buffer_details: UniformBufferDetails<MVPUniform>,
  depth_buffer_details: DepthImageDetails,
}


/**
 * An arbitrary uniform buffer
 *
 * Uniform buffers are large blobs of state that are passed to all vertices and fragments during
 * the render process.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
unsafe struct UniformBufferDetails<C> {
  buffer: vkbs::PreparedBuffer,
  _content_type: PhantomData<C>,
}

/**
 * The depth image, used for depth testing during rendering.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
unsafe struct DepthImageDetails {
  image: vkbs::PreparedImage,
  image_view: vk::ImageView,
}

/**
 * Allocated buffers for a particular mesh.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
unsafe struct MeshBuffers {
  vertex_buffer: VertexBuffer,
  index_buffer: IndexBuffer,
}

/**
 * A vertex buffer.
 *
 * Contains arbitrary vertices, along with a id to the descriptor that describes their form.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
unsafe struct VertexBuffer {
  vertex_buffer_descriptor_id: VertexBufferDescriptorId,
  buffer: vkbs::PreparedBuffer,
}

/**
 * An index buffer
 *
 * Contains indexes into an arbitrary vertex buffer indicating the triangles that form a mesh's
 * shape.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
struct IndexBufferDetails {
  buffer: vkbs::PreparedBuffer,
  num_indexes: u32,
}

/** A mapping from vertex buffer descriptor id to vertex buffer descriptor. */
type VertexBufferDescriptorCache = HashMap<VertexBufferDescriptorId, VertexBufferDescriptor>

/** A unique identifier for a vertex buffer descriptor. */
type VertexBufferDescriptorId = u32

/** The vertex describing information required to construct a graphics pipeline for a mesh. */
struct VertexBufferDescriptor {
  attr_descriptions: Vec<vk::VertexInputAttributeDescription>,
  binding_description: vk::VertexInputBindingDescription,
}

/** A uniform describing the "view" and "projection": characteristics of the camera/viewer. */
#[repr(C)]
struct MVPUniform {
  view: cgmath::Matrix4<f32>,
  proj: cgmath::Matrix4<f32>,
}

impl<'window> PlanetRenderer<'window> {
  pub fn new(base_renderer: mut BaseRenderer<'window>) -> PlanetRenderer<'window> {
    let buffer_details_bundle = make_buffer_details_bundle(&mut base_renderer);
    let descriptor_set_layouts = do_or_die!(vkdrs::make_descriptor_set_layouts(&device));
    let pipeline_layout =
      do_or_die!(vkps::make_pipeline_layout::<ModelPushConstant>(&device, &descriptor_set_layouts));

    let descriptor_pool = do_or_die!(vkdrs::make_descriptor_pool(&device));

    let descriptor_sets =
      do_or_die!(vkdrs::make_descriptor_sets(&device, &descriptor_set_layouts, &descriptor_pool));

    vkdrs::write_descriptor::<MVPUniform>(
      &device,
      &uniform_buffer_details.buffer.0, /* buffer */
      descriptor_sets.get(0).unwrap(),
      &texture_buffer_details.image_view,
      &texture_sampler,
    );

    let vert_shader_module = do_or_die!(vkl::builtins::make_shader_module(
      &device,
      include_bytes!(
        "../../../bazel-out/k8-fastbuild/genfiles/experimental/renderer/planet/planet_vert_shader.\
         spv"
      ),
    ));
    let frag_shader_module = do_or_die!(vkl::builtins::make_shader_module(
      &device,
      include_bytes!(
        "../../../bazel-out/k8-fastbuild/genfiles/experimental/renderer/planet/planet_frag_shader.\
         spv"
      ),
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

    let command_buffers =
      do_or_die!(vkbs::make_command_buffers(&device, &gfx_command_pool, framebuffers.len() as u32));

    let image_available_semaphore = do_or_die!(vkl::builtins::make_semaphore(&device));
    let render_finished_semaphore = do_or_die!(vkl::builtins::make_semaphore(&device));

    let swapchain_image_count = swapchain.swapchain_image_count;
    let mut command_buffer_fences = Vec::new();
    let mut first_frame_for_idxs = Vec::new();
    for _ in 0..swapchain_image_count {
      command_buffer_fences.push(do_or_die!(vkl::builtins::make_fence(&device)));
      first_frame_for_idxs.push(true);
    }

    // TODO(acmcarther): Perform initialization specific to planet demo
    PlanetRenderer {
      base_renderer: base_renderer,
      vert_shader_module: vert_shader_module,
      frag_shader_module: frag_shader_module,
      uniform_buffer: uniform_buffer_details.buffer,
      index_buffer_details: index_buffer_details,
      vertex_buffer: vertex_buffer_details.buffer,
      depth_image: depth_buffer_details.image,
      depth_image_view: depth_buffer_details.image_view,
      descriptor_pool: descriptor_pool,
      descriptor_set_layouts: descriptor_set_layouts,
      descriptor_sets: descriptor_sets,
      pipeline_layout: pipeline_layout,
      graphics_pipeline: graphics_pipeline,
      framebuffers: framebuffers,
      command_buffer_fences: command_buffer_fences,
      command_buffers: command_buffers,
      image_available_semaphore: image_available_semaphore,
      render_finished_semaphore: render_finished_semaphore,
    }
  }
}

impl<'window> Drop for PlanetRenderer<'window> {
  fn drop(&mut self) {
    let device = &mut self.base_renderer.device;

    do_or_die!(device.device_wait_idle());
    device.destroy_semaphore(self.render_finished_semaphore);
    device.destroy_semaphore(self.image_available_semaphore);
    device.destroy_command_pool(self.gfx_command_pool);
    if let Some(command_pool) = self.transfer_command_pool_opt {
      device.destroy_command_pool(command_pool);
    }
    for framebuffer in self.framebuffers.drain(..) {
      device.destroy_framebuffer(framebuffer);
    }
    device.destroy_pipeline(self.graphics_pipeline);
    device.destroy_sampler(self.texture_sampler);
    device.destroy_image_view(self.depth_image_view);
    device.destroy_buffer(self.uniform_buffer.0 /* buffer */);
    device.destroy_buffer(self.index_buffer_details.buffer.0 /* buffer */);
    device.destroy_buffer(self.vertex_buffer.0 /* buffer */);
    device.destroy_image(self.depth_image.0 /* image */);
    device.free_memory(self.depth_image.1 /* deviceMemory */);
    device.free_memory(self.uniform_buffer.1 /* deviceMemory */);
    device.free_memory(self.index_buffer_details.buffer.1 /* deviceMemory */);
    device.free_memory(self.vertex_buffer.1 /* deviceMemory */);
    device.destroy_descriptor_pool(self.descriptor_pool);
    for descriptor_set_layout in self.descriptor_set_layouts.drain(..) {
      device.destroy_descriptor_set_layout(descriptor_set_layout)
    }
    device.destroy_pipeline_layout(self.pipeline_layout);
    device.destroy_shader_module(self.vert_shader_module);
    device.destroy_shader_module(self.frag_shader_module);
    for command_buffer_fence in self.command_buffer_fences.drain(..) {
      device.destroy_fence(command_buffer_fence);
    }
  }
}

fn make_buffer_details_bundle(base_renderer: &mut BaseRenderer) -> BufferDetailBundle {
  let copy_command_pool = transfer_command_pool_opt.as_ref().unwrap_or(&gfx_command_pool);
  let queue_family_idx =
    device_spec.dedicated_transfer_queue_family_idx_opt.unwrap_or(device_spec.gfx_queue_family_idx);
  let queue = device.get_device_queue(queue_family_idx, 0 /* queueIdx */);
  let gfx_queue = device.get_device_queue(device_spec.gfx_queue_family_idx, 0 /* queueIdx */);

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

  let depth_buffer_details = do_or_die!(make_depth_image(
    &device,
    &swapchain,
    depth_format,
    &gfx_command_pool,
    &gfx_queue,
    &device_spec.memory_properties
  ));

  BufferDetailBundle {
    vertex_buffer_details: vertex_buffer_details,
    index_buffer_details: index_buffer_details,
    uniform_buffer_details: uniform_buffer_details,
    depth_buffer_details: depth_buffer_details,
  }
}

fn make_vertex_buffer(
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
      binding_description: binding_description,
    },
  })
}

fn make_index_buffer(
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

fn make_uniform_buffer(
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
    _content_type: PhantomData<MVPUniform>,
  })
}

fn make_depth_image(
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
