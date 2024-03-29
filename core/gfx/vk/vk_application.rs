#[macro_use]
extern crate log;
#[macro_use]
extern crate memoffset;
extern crate vk_base_renderer;
extern crate vk_buffer_cache;
extern crate vk_buffer_support as vkbs;
extern crate vk_descriptor_support as vkdrs;
#[macro_use(do_or_die)]
extern crate vk_lite as vkl;
extern crate vk_pipeline_support as vkps;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

use vk_base_renderer::BaseRenderer;
use vk_buffer_cache::MeshCache;
use vk_buffer_cache::VertexBufferDescriptorCache;

/**
 * Defines the application configuration functions required for renderer init.
 *
 * This trait is ad-hoc at best, and was extracted by force from existing applications. It will
 * probably be refactored to be more declarative.
 */
pub trait ApplicationRendererInitializer {
  /** Generates the PushConstantRanges for the app's used push constants. */
  fn make_push_constant_ranges(&mut self) -> Vec<vk::PushConstantRange>;

  /** Populates a mesh cache with meshes. */
  fn init_mesh_cache(&mut self, mesh_cache: &mut MeshCache, base_renderer: &mut BaseRenderer);

  /** Populates a vertex buffer descriptor cache. */
  fn init_vbd_cache(&mut self, vbd_cache: &mut VertexBufferDescriptorCache);

  /**
   * Produces a vertex shader module for the application.
   *
   * This is likely to change in a couple of ways:
   * - It will likely support providing more than one shader as most applications have more than
   * one.
   * - It will likely support hotloading of the shader as this is a useful development mechanism.
   */
  fn make_vert_shader_module(&mut self, device: &vkl::LDevice) -> vkl::RawResult<vk::ShaderModule>;

  /**
   * Produces a vertex shader module for the application.
   *
   * This is likely to change in a couple of ways:
   * - It will likely support providing more than one shader as most applications have more than
   * one.
   * - It will likely support hotloading of the shader as this is a useful development mechanism.
   */
  fn make_frag_shader_module(&mut self, device: &vkl::LDevice) -> vkl::RawResult<vk::ShaderModule>;

  /** Produces the buffer objects and descriptors required for the application. */
  fn make_uniform_buffers(
    &mut self,
    device: &vkl::LDevice,
    memory_properties: &vk::PhysicalDeviceMemoryProperties,
  ) -> vkl::RawResult<(Vec<vkbs::PreparedBuffer>, Vec<vk::DescriptorBufferInfo>)>;

  /**
   * Retrieves the list of vertex input attribute descriptions from the cache.
   *
   * This is expected to be tightly coupled to the vbd cache populating function.
   */
  fn get_all_attr_desc(
    &mut self,
    vbd_cache: &VertexBufferDescriptorCache,
  ) -> Vec<vk::VertexInputAttributeDescription>;

  /**
   * Retrieves the list of vertex input binding descriptions from the cache.
   *
   * This is expected to be tightly coupled to the vbd cache populating function.
   */
  fn get_all_vert_buf_desc(
    &mut self,
    vbd_cache: &VertexBufferDescriptorCache,
  ) -> Vec<vk::VertexInputBindingDescription>;
}

/**
 * The depth image, used for depth testing during rendering.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
pub struct DepthImage {
  pub image: vkbs::PreparedImage,
  pub image_view: vk::ImageView,
}

pub struct Application<'window> {
  base_renderer: BaseRenderer<'window>,
  vert_shader_module: vk::ShaderModule,
  frag_shader_module: vk::ShaderModule,
  mesh_cache: MeshCache,
  vertex_buffer_descriptor_cache: VertexBufferDescriptorCache,
  depth_image: DepthImage,
  uniform_buffers: Vec<vkbs::PreparedBuffer>,
  descriptor_pool: vk::DescriptorPool,
  descriptor_set_layouts: Vec<vk::DescriptorSetLayout>,
  descriptor_sets: Vec<vk::DescriptorSet>,
  pipeline_layout: vk::PipelineLayout,
  graphics_pipeline: vk::Pipeline,
  framebuffers: Vec<vk::Framebuffer>,
  command_buffer_fences: Vec<vk::Fence>,
  command_buffers: Vec<vk::CommandBuffer>,
  image_available_semaphore: vk::Semaphore,
  render_finished_semaphore: vk::Semaphore,
  first_frame_for_idxs: Vec<bool>,
}

impl<'window> Drop for Application<'window> {
  fn drop(&mut self) {
    let device = &mut self.base_renderer.device;

    do_or_die!(device.device_wait_idle());
    device.destroy_semaphore(self.render_finished_semaphore);
    device.destroy_semaphore(self.image_available_semaphore);
    device.destroy_command_pool(self.base_renderer.gfx_command_pool);
    if let Some(command_pool) = self.base_renderer.transfer_command_pool_opt {
      device.destroy_command_pool(command_pool);
    }
    for framebuffer in self.framebuffers.drain(..) {
      device.destroy_framebuffer(framebuffer);
    }
    device.destroy_pipeline(self.graphics_pipeline);

    device.destroy_image_view(self.depth_image.image_view);
    device.destroy_image(self.depth_image.image.0 /* image */);
    device.free_memory(self.depth_image.image.1 /* deviceMemory */);

    for buffer in self.uniform_buffers.drain(..) {
      device.destroy_buffer(buffer.0 /* buffer */);
      device.free_memory(buffer.1 /* deviceMemory */);
    }

    vk_buffer_cache::deallocate_mesh_cache(&device, &mut self.mesh_cache);

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

pub fn init<'window, ARI: ApplicationRendererInitializer>(
  mut base_renderer: BaseRenderer<'window>,
  app_rendr_initr: &mut ARI,
) -> Application<'window> {
  let descriptor_set_layouts =
    do_or_die!(vkdrs::make_descriptor_set_layouts(&base_renderer.device));
  let pipeline_layout = do_or_die!(vkps::make_pipeline_layout(
    &base_renderer.device,
    &descriptor_set_layouts,
    &app_rendr_initr.make_push_constant_ranges(),
  ));

  let descriptor_pool = do_or_die!(vkdrs::make_descriptor_pool(&base_renderer.device));

  let descriptor_sets = do_or_die!(vkdrs::make_descriptor_sets(
    &base_renderer.device,
    &descriptor_set_layouts,
    &descriptor_pool
  ));

  let mut vertex_buffer_descriptor_cache = VertexBufferDescriptorCache::new();
  app_rendr_initr.init_vbd_cache(&mut vertex_buffer_descriptor_cache);
  let mut mesh_cache = MeshCache::new();
  app_rendr_initr.init_mesh_cache(&mut mesh_cache, &mut base_renderer);

  let (uniform_buffers, uniform_buffer_infos) = do_or_die!(app_rendr_initr.make_uniform_buffers(
    &base_renderer.device,
    &base_renderer.device_spec.memory_properties
  ));

  unsafe {
    vkdrs::write_descriptors(
      &base_renderer.device,
      descriptor_sets.get(0).unwrap(),
      0, /* descriptor_binding_id */
      uniform_buffer_infos,
    );
  };

  let vert_shader_module =
    do_or_die!(app_rendr_initr.make_vert_shader_module(&base_renderer.device));
  let frag_shader_module =
    do_or_die!(app_rendr_initr.make_frag_shader_module(&base_renderer.device));

  let graphics_pipeline = {
    do_or_die!(vkps::make_graphics_pipeline(
      &base_renderer.device,
      &vert_shader_module,
      &frag_shader_module,
      &app_rendr_initr.get_all_attr_desc(&vertex_buffer_descriptor_cache),
      &app_rendr_initr.get_all_vert_buf_desc(&vertex_buffer_descriptor_cache),
      &base_renderer.render_pass,
      &base_renderer.swapchain,
      &pipeline_layout
    ))
  };

  let depth_image = do_or_die!(make_depth_image(&base_renderer));

  let framebuffers = do_or_die!(vkbs::make_framebuffers(
    &base_renderer.device,
    &base_renderer.image_views,
    &depth_image.image_view,
    &base_renderer.swapchain,
    &base_renderer.render_pass
  ));

  let command_buffers = do_or_die!(vkbs::make_command_buffers(
    &base_renderer.device,
    &base_renderer.gfx_command_pool,
    framebuffers.len() as u32
  ));

  let image_available_semaphore = do_or_die!(vkl::builtins::make_semaphore(&base_renderer.device));
  let render_finished_semaphore = do_or_die!(vkl::builtins::make_semaphore(&base_renderer.device));

  let swapchain_image_count = base_renderer.swapchain.swapchain_image_count;
  let mut command_buffer_fences = Vec::new();
  let mut first_frame_for_idxs = Vec::new();
  for _ in 0..swapchain_image_count {
    command_buffer_fences.push(do_or_die!(vkl::builtins::make_fence(&base_renderer.device)));
    first_frame_for_idxs.push(true);
  }

  Application {
    base_renderer: base_renderer,
    vert_shader_module: vert_shader_module,
    frag_shader_module: frag_shader_module,
    uniform_buffers: uniform_buffers,
    depth_image: depth_image,
    vertex_buffer_descriptor_cache: vertex_buffer_descriptor_cache,
    mesh_cache: mesh_cache,
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
    first_frame_for_idxs: first_frame_for_idxs,
  }
}

pub fn make_depth_image(base_renderer: &BaseRenderer) -> vkl::RawResult<DepthImage> {
  let device = &base_renderer.device;
  let swapchain = &base_renderer.swapchain;
  let depth_format = base_renderer.depth_format;
  let gfx_command_pool = &base_renderer.gfx_command_pool;
  let memory_properties = &base_renderer.device_spec.memory_properties;
  let gfx_queue = base_renderer.device.get_device_queue(
    base_renderer.device_spec.gfx_queue_family_idx,
    0, /* queueIdx */
  );

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
    &gfx_command_pool,
    &gfx_queue,
    &prepared_image.0, /* image */
    depth_format,
    vk::IMAGE_LAYOUT_UNDEFINED,
    vk::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL
  ));

  Ok(DepthImage {
    image: prepared_image,
    image_view: image_view,
  })
}
