extern crate cgmath;
extern crate geometry;
extern crate icosphere;
#[macro_use]
extern crate log;
#[macro_use]
extern crate memoffset;
extern crate vk_buffer_cache;
extern crate vk_buffer_support as vkbs;
extern crate vk_descriptor_support as vkdrs;
extern crate vk_device_support as vkds;
extern crate vk_instance_support as vkis;
#[macro_use(do_or_die)]
extern crate vk_lite as vkl;
extern crate vk_pipeline_support as vkps;
extern crate vk_renderer;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

mod app_cfg {
  use cgmath::Matrix4;
  use geometry::Mesh;
  use geometry::Vertex;
  use icosphere;
  use std::mem;
  use vk;
  use vk_buffer_cache::IndexBuffer;
  use vk_buffer_cache::MeshBuffers;
  use vk_buffer_cache::MeshCache;
  use vk_buffer_cache::VertexBuffer;
  use vk_buffer_cache::VertexBufferDescriptor;
  use vk_buffer_cache::VertexBufferDescriptorCache;
  use vk_buffer_cache::VertexBufferDescriptorId;
  use vk_renderer::BaseRenderer;
  use vkbs;
  use vkdrs::BufferInfoGenerator;
  use vkl;
  use vkps::PushConstantRangeGenerator;

  pub const STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID: VertexBufferDescriptorId = 1u32;

  pub type UniformBuffer = vkbs::PreparedBuffer;

  /**
   * A "push constant" for supplying a model matrix.
   *
   * Push constants are small bits of state that can be efficiently bound during command buffer
   * construction. They're useful for a form of "instanced rendering" where the same model is
   * rerendered many times with only a small difference
   */
  #[repr(C)]
  pub struct ModelPushConstant {
    model: Matrix4<f32>,
  }

  /** A uniform describing the "view" and "projection": characteristics of the camera/viewer. */
  #[repr(C)]
  pub struct MVPUniform {
    view: Matrix4<f32>,
    proj: Matrix4<f32>,
  }

  /**
   * Generates the single vbd for the application.
   *
   * This is highly application-specific and not really generalizable.
   */
  pub fn make_standard_vertex_buffer_descriptor() -> VertexBufferDescriptor {
    let pos_attr_desc = vk::VertexInputAttributeDescription {
      binding: 0,
      location: 0,
      format: vk::FORMAT_R32G32B32_SFLOAT,
      offset: offset_of!(Vertex, pos) as u32,
    };

    let norm_attr_desc = vk::VertexInputAttributeDescription {
      binding: 0,
      location: 1,
      format: vk::FORMAT_R32G32B32_SFLOAT,
      offset: offset_of!(Vertex, norm) as u32,
    };

    let binding_description = vk::VertexInputBindingDescription {
      binding: 0,
      stride: mem::size_of::<Vertex>() as u32,
      inputRate: vk::VERTEX_INPUT_RATE_VERTEX, /* advance per vertex (instead of per instance) */
    };

    VertexBufferDescriptor {
      attr_descriptions: vec![pos_attr_desc, norm_attr_desc],
      binding_description: binding_description,
    }
  }

  /**
   * Generates the mesh buffers for a standard vertex
   *
   * This is specific to the pipeline configation and the Vertex type.
   */
  pub fn make_mesh_buffers(
    device: &vkl::LDevice,
    command_pool: &vk::CommandPool,
    queue: &vk::Queue,
    memory_properties: &vk::PhysicalDeviceMemoryProperties,
    mesh: &Mesh,
  ) -> vkl::RawResult<MeshBuffers> {
    let vertex_buffer = {
      let vertices = &mesh.vertices;
      let buffer_size = (mem::size_of::<Vertex>() * vertices.len()) as u64;

      let vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory) = try!(vkbs::make_buffer(
        device,
        buffer_size,
        vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
        vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
        memory_properties
      ));

      unsafe {
        try!(device.bind_buffer_memory(&transfer_buffer, &transfer_device_memory));
        try!(device.map_vec_data_to_memory(&transfer_device_memory, vertices));
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

      VertexBuffer {
        buffer: vkbs::PreparedBuffer(buffer, device_memory),
        // N.B.: This is coupled to the "mesh" argument. If type changes change this.
        vertex_buffer_descriptor_id: STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID,
      }
    };

    let index_buffer = {
      // AHHHH!!
      let indexes = &mesh.indices;

      let buffer_size = (mem::size_of::<u16>() * indexes.len()) as u64;

      let vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory) = try!(vkbs::make_buffer(
        device,
        buffer_size,
        vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
        vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
        memory_properties
      ));

      unsafe {
        try!(device.bind_buffer_memory(&transfer_buffer, &transfer_device_memory));
        try!(device.map_vec_data_to_memory(&transfer_device_memory, indexes));
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

      IndexBuffer {
        num_indexes: indexes.len() as u32,
        buffer: vkbs::PreparedBuffer(buffer, device_memory),
      }
    };

    Ok(MeshBuffers {
      index_buffer: index_buffer,
      vertex_buffer: vertex_buffer,
    })
  }

  pub fn make_push_constant_ranges() -> Vec<vk::PushConstantRange> {
    PushConstantRangeGenerator::new()
      .push::<ModelPushConstant>(vk::SHADER_STAGE_VERTEX_BIT)
      .take_ranges()
  }

  pub fn init_mesh_cache(mesh_cache: &mut MeshCache, base_renderer: &mut BaseRenderer) {
    let copy_command_pool = base_renderer
      .transfer_command_pool_opt
      .as_ref()
      .unwrap_or(&base_renderer.gfx_command_pool);
    let queue_family_idx = base_renderer
      .device_spec
      .dedicated_transfer_queue_family_idx_opt
      .unwrap_or(base_renderer.device_spec.gfx_queue_family_idx);
    let queue = base_renderer
      .device
      .get_device_queue(queue_family_idx, 0 /* queueIdx */);

    let icospheres = vec![(0, 0u32 /* iterations */)];

    for (mesh_id, num_iterations) in icospheres.into_iter() {
      let mesh = icosphere::icosphere(num_iterations);
      let mesh_buffers = do_or_die!(make_mesh_buffers(
        &base_renderer.device,
        &copy_command_pool,
        &queue,
        &base_renderer.device_spec.memory_properties,
        &mesh,
      ));
      debug!("Ico: {} has {} verts", mesh_id, mesh.vertices.len());

      mesh_cache.insert(mesh_id, mesh_buffers);
    }
  }

  pub fn init_vbd_cache(vbd_cache: &mut VertexBufferDescriptorCache) {
    vbd_cache.insert(
      STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID,
      make_standard_vertex_buffer_descriptor(),
    );
  }

  pub fn make_uniform_buffers(
    device: &vkl::LDevice,
    memory_properties: &vk::PhysicalDeviceMemoryProperties,
  ) -> vkl::RawResult<(Vec<UniformBuffer>, Vec<vk::DescriptorBufferInfo>)> {
    let buffer_size = mem::size_of::<MVPUniform>();
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
    let uniform_buffer_infos = unsafe {
      BufferInfoGenerator::new()
        .push::<MVPUniform>(&prepared_buffer.0 /* buffer */)
        .take_infos()
    };
    Ok((vec![prepared_buffer], uniform_buffer_infos))
  }

  pub fn make_vertex_shader_module(device: &vkl::LDevice) -> vkl::RawResult<vk::ShaderModule> {
    vkl::builtins::make_shader_module(
      &device,
      include_bytes!(
        "../../../bazel-out/k8-fastbuild/genfiles/experimental/demos/geo_coor/vert_shader.spv"
      ),
    )
  }

  pub fn make_frag_shader_module(device: &vkl::LDevice) -> vkl::RawResult<vk::ShaderModule> {
    vkl::builtins::make_shader_module(
      &device,
      include_bytes!(
        "../../../bazel-out/k8-fastbuild/genfiles/experimental/demos/geo_coor/frag_shader.spv"
      ),
    )
  }

  pub fn get_all_attr_desc(
    vbd_cache: &VertexBufferDescriptorCache,
  ) -> Vec<vk::VertexInputAttributeDescription> {
    let standard_vertex_buffer_descriptor = vbd_cache
      .get(&STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID)
      .unwrap();
    standard_vertex_buffer_descriptor
      .attr_descriptions
      .iter()
      .map(vkl::builtins::clone_viad)
      .collect()
  }

  pub fn get_all_vert_buf_desc(
    vbd_cache: &VertexBufferDescriptorCache,
  ) -> Vec<vk::VertexInputBindingDescription> {
    let standard_vertex_buffer_descriptor = vbd_cache
      .get(&STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID)
      .unwrap();
    vec![
      standard_vertex_buffer_descriptor.binding_description_cloned(),
    ]
  }
}

mod init {
  use app_cfg;
  use vkbs;
  use vkl;
  use vkdrs;
  use vk;
  use vkps;
  use vkss;
  use vk_renderer::BaseRenderer;
  use vk_buffer_cache::VertexBufferDescriptorCache;
  use vk_buffer_cache::MeshCache;
  use vk_buffer_cache;

  /**
   * The depth image, used for depth testing during rendering.
   *
   * UNSAFE: Requires manual deallocation by the device that instantiated it.
   */
  pub struct DepthImage {
    pub image: vkbs::PreparedImage,
    pub image_view: vk::ImageView,
  }

  pub struct GenericRenderer<'window> {
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

  impl<'window> Drop for GenericRenderer<'window> {
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

  pub fn init<'window>(mut base_renderer: BaseRenderer<'window>) -> GenericRenderer {
    let descriptor_set_layouts =
      do_or_die!(vkdrs::make_descriptor_set_layouts(&base_renderer.device));
    let pipeline_layout = do_or_die!(vkps::make_pipeline_layout(
      &base_renderer.device,
      &descriptor_set_layouts,
      &app_cfg::make_push_constant_ranges(),
    ));

    let descriptor_pool = do_or_die!(vkdrs::make_descriptor_pool(&base_renderer.device));

    let descriptor_sets = do_or_die!(vkdrs::make_descriptor_sets(
      &base_renderer.device,
      &descriptor_set_layouts,
      &descriptor_pool
    ));

    let mut vertex_buffer_descriptor_cache = VertexBufferDescriptorCache::new();
    app_cfg::init_vbd_cache(&mut vertex_buffer_descriptor_cache);
    let mut mesh_cache = MeshCache::new();
    app_cfg::init_mesh_cache(&mut mesh_cache, &mut base_renderer);

    let (uniform_buffers, uniform_buffer_infos) = do_or_die!(app_cfg::make_uniform_buffers(
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

    let vert_shader_module = do_or_die!(app_cfg::make_vertex_shader_module(&base_renderer.device));
    let frag_shader_module = do_or_die!(app_cfg::make_frag_shader_module(&base_renderer.device));

    let graphics_pipeline = {
      do_or_die!(vkps::make_graphics_pipeline(
        &base_renderer.device,
        &vert_shader_module,
        &frag_shader_module,
        &app_cfg::get_all_attr_desc(&vertex_buffer_descriptor_cache),
        &app_cfg::get_all_vert_buf_desc(&vertex_buffer_descriptor_cache),
        &base_renderer.render_pass,
        &base_renderer.swapchain,
        &pipeline_layout
      ))
    };

    let depth_image = {
      let gfx_queue = base_renderer.device.get_device_queue(
        base_renderer.device_spec.gfx_queue_family_idx,
        0, /* queueIdx */
      );
      do_or_die!(make_depth_image(&base_renderer))
    };

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

    let image_available_semaphore =
      do_or_die!(vkl::builtins::make_semaphore(&base_renderer.device));
    let render_finished_semaphore =
      do_or_die!(vkl::builtins::make_semaphore(&base_renderer.device));

    let swapchain_image_count = base_renderer.swapchain.swapchain_image_count;
    let mut command_buffer_fences = Vec::new();
    let mut first_frame_for_idxs = Vec::new();
    for _ in 0..swapchain_image_count {
      command_buffer_fences.push(do_or_die!(vkl::builtins::make_fence(&base_renderer.device)));
      first_frame_for_idxs.push(true);
    }

    // TODO(acmcarther): Perform initialization specific to galaxy_big demo
    GenericRenderer {
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
}
