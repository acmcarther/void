extern crate cgmath;
extern crate gfx_basics;
extern crate procedural;
#[macro_use]
extern crate log;
#[macro_use]
extern crate memoffset;
extern crate vk_sys;
#[macro_use]
extern crate vk;

use cgmath::Angle;
use gfx_basics::Mesh;
use gfx_basics::Vertex;
use vk::base_renderer::BaseRenderer;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::ptr;
use vk::lite as lite;
use vk::buffer_support as vkbs;
use vk::descriptor_support as vkdrs;
use vk::pipeline_support as vkps;
use vk::swapchain_support as vkss;
use vk::pipeline_support::PushConstantRangeGenerator;
use vk::descriptor_support::BufferInfoGenerator;
use vk::buffer_cache::VertexBufferDescriptorId;
use vk::buffer_cache::MeshId;
use vk::buffer_cache::MeshCache;
use vk::buffer_cache::VertexBufferDescriptorCache;
use vk::buffer_cache::VertexBufferDescriptor;
use vk::buffer_cache::VertexBuffer;
use vk::buffer_cache::IndexBuffer;
use vk::buffer_cache::MeshBufferSet;

/** A GalaxyBig-demo specific renderer. */
pub struct GalaxyBigRenderer<'window> {
  base_renderer: BaseRenderer<'window>,
  vert_shader_module: vk_sys::ShaderModule,
  frag_shader_module: vk_sys::ShaderModule,
  mesh_cache: MeshCache,
  vertex_buffer_descriptor_cache: VertexBufferDescriptorCache,
  depth_image: DepthImage,
  uniform_buffer: UniformBuffer,
  descriptor_pool: vk_sys::DescriptorPool,
  descriptor_set_layouts: Vec<vk_sys::DescriptorSetLayout>,
  descriptor_sets: Vec<vk_sys::DescriptorSet>,
  pipeline_layout: vk_sys::PipelineLayout,
  graphics_pipeline: vk_sys::Pipeline,
  framebuffers: Vec<vk_sys::Framebuffer>,
  command_buffer_fences: Vec<vk_sys::Fence>,
  command_buffers: Vec<vk_sys::CommandBuffer>,
  image_available_semaphore: vk_sys::Semaphore,
  render_finished_semaphore: vk_sys::Semaphore,
  first_frame_for_idxs: Vec<bool>,
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
 * An arbitrary uniform buffer
 *
 * Uniform buffers are large blobs of state that are passed to all vertices and fragments during
 * the render process.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
struct UniformBuffer {
  buffer: vkbs::PreparedBuffer,
}

/**
 * The depth image, used for depth testing during rendering.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
struct DepthImage {
  image: vkbs::PreparedImage,
  image_view: vk_sys::ImageView,
}

/** A uniform describing the "view" and "projection": characteristics of the camera/viewer. */
#[repr(C)]
struct MVPUniform {
  view: cgmath::Matrix4<f32>,
  proj: cgmath::Matrix4<f32>,
}

pub struct MeshToRender {
  pub mesh_id: MeshId,
  pub pos: [f32; 3],
  pub scale: f32,
  //pub scale: f32,
}

const STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID: VertexBufferDescriptorId = 1u32;
pub const ICO_0_MESH_ID: MeshId = 0u32;
pub const ICO_1_MESH_ID: MeshId = 1u32;
pub const ICO_2_MESH_ID: MeshId = 2u32;
pub const ICO_3_MESH_ID: MeshId = 3u32;
pub const ICO_4_MESH_ID: MeshId = 4u32;
pub const ICO_5_MESH_ID: MeshId = 5u32;
pub const ICO_6_MESH_ID: MeshId = 6u32;

impl<'window> GalaxyBigRenderer<'window> {
  pub fn new(base_renderer: BaseRenderer<'window>) -> GalaxyBigRenderer<'window> {
    let descriptor_set_layouts =
      do_or_die!(vkdrs::make_descriptor_set_layouts(&base_renderer.device));
    let pipeline_layout = do_or_die!(vkps::make_pipeline_layout(
      &base_renderer.device,
      &descriptor_set_layouts,
      &PushConstantRangeGenerator::new()
        .push::<ModelPushConstant>(vk_sys::SHADER_STAGE_VERTEX_BIT)
        .take_ranges()
    ));

    let descriptor_pool = do_or_die!(vkdrs::make_descriptor_pool(&base_renderer.device));

    let descriptor_sets = do_or_die!(vkdrs::make_descriptor_sets(
      &base_renderer.device,
      &descriptor_set_layouts,
      &descriptor_pool
    ));

    let mut vertex_buffer_descriptor_cache = VertexBufferDescriptorCache::new();
    vertex_buffer_descriptor_cache.insert(
      STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID,
      make_standard_vertex_buffer_descriptor(),
    );

    let mut mesh_cache = MeshCache::new();
    {
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

      let icospheres = vec![
        (ICO_0_MESH_ID, 0u32 /* iterations */),
        (ICO_1_MESH_ID, 1u32 /* iterations */),
        (ICO_2_MESH_ID, 2u32 /* iterations */),
        (ICO_3_MESH_ID, 3u32 /* iterations */),
        (ICO_4_MESH_ID, 4u32 /* iterations */),
        (ICO_5_MESH_ID, 5u32 /* iterations */),
        (ICO_6_MESH_ID, 6u32 /* iterations */),
      ];

      for (mesh_id, num_iterations) in icospheres.into_iter() {
        let mesh = procedural::icosphere(num_iterations);
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

    let uniform_buffer = do_or_die!(make_uniform_buffer(
      &base_renderer.device,
      &base_renderer.device_spec.memory_properties
    ));

    unsafe {
      vkdrs::write_descriptors(
        &base_renderer.device,
        descriptor_sets.get(0).unwrap(),
        0, /* descriptor_binding_id */
        BufferInfoGenerator::new()
          .push::<MVPUniform>(&uniform_buffer.buffer.0 /* buffer */)
          .take_infos(),
      );
    };

    let vert_shader_module = do_or_die!(lite::builtins::make_shader_module(
      &base_renderer.device,
      include_bytes!(
        "../../../bazel-out/k8-fastbuild/genfiles/experimental/demo/galaxy_big/galaxy_big_vert_shader.\
         spv"
      ),
    ));
    let frag_shader_module = do_or_die!(lite::builtins::make_shader_module(
      &base_renderer.device,
      include_bytes!(
        "../../../bazel-out/k8-fastbuild/genfiles/experimental/demo/galaxy_big/galaxy_big_frag_shader.\
         spv"
      ),
    ));

    let graphics_pipeline = {
      let standard_vertex_buffer_descriptor = vertex_buffer_descriptor_cache
        .get(&STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID)
        .unwrap();
      do_or_die!(vkps::make_graphics_pipeline(
        &base_renderer.device,
        &vert_shader_module,
        &frag_shader_module,
        &standard_vertex_buffer_descriptor.attr_descriptions,
        &vec![
          standard_vertex_buffer_descriptor.binding_description_cloned(),
        ],
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
      do_or_die!(make_depth_image(
        &base_renderer.device,
        &base_renderer.swapchain,
        base_renderer.depth_format,
        &base_renderer.gfx_command_pool,
        &gfx_queue,
        &base_renderer.device_spec.memory_properties
      ))
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
      do_or_die!(lite::builtins::make_semaphore(&base_renderer.device));
    let render_finished_semaphore =
      do_or_die!(lite::builtins::make_semaphore(&base_renderer.device));

    let swapchain_image_count = base_renderer.swapchain.swapchain_image_count;
    let mut command_buffer_fences = Vec::new();
    let mut first_frame_for_idxs = Vec::new();
    for _ in 0..swapchain_image_count {
      command_buffer_fences.push(do_or_die!(lite::builtins::make_fence(&base_renderer.device)));
      first_frame_for_idxs.push(true);
    }

    // TODO(acmcarther): Perform initialization specific to galaxy_big demo
    GalaxyBigRenderer {
      base_renderer: base_renderer,
      vert_shader_module: vert_shader_module,
      frag_shader_module: frag_shader_module,
      uniform_buffer: uniform_buffer,
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

  pub fn set_camera_pos(&mut self, pos: [f32; 3], target: [f32; 3]) {
    let view = cgmath::Matrix4::<f32>::look_at(
      cgmath::Point3::<f32>::new(pos[0], pos[1], pos[2]),
      cgmath::Point3::<f32>::new(target[0], target[1], target[2]),
      cgmath::Vector3::<f32>::new(0.0f32, 0.0f32, 1.0f32),
    );
    let mut proj = cgmath::perspective(
      cgmath::Rad::<f32>::turn_div_4() / 2.0f32,
      (self.base_renderer.swapchain.surface_extent.width as f32)
        / (self.base_renderer.swapchain.surface_extent.height as f32),
      0.1f32,     /* near clip plane */
      10000.0f32, /* far clip plane */
    );
    proj.y.y = proj.y.y * -1.0f32;

    let new_uniform = MVPUniform {
      view: view,
      proj: proj,
    };

    unsafe {
      do_or_die!(self.base_renderer.device.map_data_to_memory(
        &self.uniform_buffer.buffer.1, /* deviceMemory */
        &new_uniform
      ));
    }
  }

  pub fn draw_demo_frame(&mut self, meshes_to_render: &Vec<MeshToRender>) {
    let image_index = do_or_die!(lite::util::loady("next image", &|a| unsafe {
      self.base_renderer.device.ptrs().AcquireNextImageKHR(
        self.base_renderer.device.logical_device,
        self.base_renderer.swapchain.swapchain,
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

    // Do a bunch of jank stuff for one-time init of swapchain image fences
    {
      let first_frame_for_idx = *self.first_frame_for_idxs.get(image_index as usize).unwrap();
      if !first_frame_for_idx {
        unsafe {
          let all_fences = [*command_buffer_fence];
          do_or_die!(lite::util::dooy("wait for fences", &|| {
            self.base_renderer.device.ptrs().WaitForFences(
              self.base_renderer.device.logical_device,
              1,
              all_fences.as_ptr(),
              vk_sys::TRUE,    /* wait all */
              10000000000, /* ns */
            )
          }));
          do_or_die!(lite::util::dooy("reset fences", &|| self
            .base_renderer
            .device
            .ptrs()
            .ResetFences(
              self.base_renderer.device.logical_device,
              1,
              all_fences.as_ptr(),
            )));
        }
      } else {
        let first_frame_for_idx = self
          .first_frame_for_idxs
          .get_mut(image_index as usize)
          .unwrap();
        *first_frame_for_idx = false;
      }
    }

    // Group meshes to render by their actual mesh (to allow us to perform instanced rendering)
    let mut meshes_to_render_by_mesh_id: HashMap<MeshId, Vec<&MeshToRender>> = HashMap::new();
    for mesh_to_render in meshes_to_render.iter() {
      let mesh_id = &mesh_to_render.mesh_id;
      if meshes_to_render_by_mesh_id.contains_key(mesh_id) {
        meshes_to_render_by_mesh_id
          .get_mut(mesh_id)
          .unwrap()
          .push(mesh_to_render);
      } else {
        meshes_to_render_by_mesh_id.insert(*mesh_id, vec![mesh_to_render]);
      }
    }

    self
      .base_renderer
      .device
      .reset_command_buffer(command_buffer);
    let command_buffer_begin_info = vk_sys::CommandBufferBeginInfo {
      sType: vk_sys::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
      pNext: ptr::null(),
      flags: vk_sys::COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
      pInheritanceInfo: ptr::null(),
    };

    do_or_die!(lite::util::dooy("start command buffer", &|| unsafe {
      self
        .base_renderer
        .device
        .ptrs()
        .BeginCommandBuffer(*command_buffer, &command_buffer_begin_info)
    }));

    let clear_color = vk_sys::ClearValue {
      color: vk_sys::ClearColorValue {
        float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32],
      },
    };
    let clear_depth = vk_sys::ClearValue {
      depthStencil: vk_sys::ClearDepthStencilValue {
        depth: 1.0f32,
        stencil: 0u32,
      },
    };
    let all_clears = [clear_color, clear_depth];
    let render_pass_begin_info = vk_sys::RenderPassBeginInfo {
      sType: vk_sys::STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
      pNext: ptr::null(),
      renderPass: self.base_renderer.render_pass,
      framebuffer: *framebuffer,
      renderArea: vk_sys::Rect2D {
        offset: vk_sys::Offset2D { x: 0, y: 0 },
        extent: vk_sys::Extent2D {
          width: self.base_renderer.swapchain.surface_extent.width,
          height: self.base_renderer.swapchain.surface_extent.height,
        },
      },
      clearValueCount: all_clears.len() as u32,
      pClearValues: all_clears.as_ptr(),
    };

    let push_constant_size = std::mem::size_of::<ModelPushConstant>();

    // Literally a bunch of draw calls, very unsafe!
    unsafe {
      self.base_renderer.device.ptrs().CmdBeginRenderPass(
        *command_buffer,
        &render_pass_begin_info,
        vk_sys::SUBPASS_CONTENTS_INLINE,
      );
      self.base_renderer.device.ptrs().CmdBindPipeline(
        *command_buffer,
        vk_sys::PIPELINE_BIND_POINT_GRAPHICS,
        self.graphics_pipeline,
      );

      self.base_renderer.device.ptrs().CmdBindDescriptorSets(
        *command_buffer,
        vk_sys::PIPELINE_BIND_POINT_GRAPHICS,
        self.pipeline_layout,
        0,                                 /* firstSet */
        self.descriptor_sets.len() as u32, /* descriptorSetCount */
        self.descriptor_sets.as_ptr(),
        0, /* dynamicOffsetCount */
        ptr::null(),
      );

      for (mesh_id, meshes_to_render_by_mesh_id) in meshes_to_render_by_mesh_id.into_iter() {
        let mesh = self.mesh_cache.get(&mesh_id).unwrap();
        let all_vertex_buffers = [mesh.vertex_buffer.buffer.0];
        let all_buffer_offsets = [0];

        self.base_renderer.device.ptrs().CmdBindVertexBuffers(
          *command_buffer,
          0,
          1,
          all_vertex_buffers.as_ptr(),
          all_buffer_offsets.as_ptr(),
        );
        self.base_renderer.device.ptrs().CmdBindIndexBuffer(
          *command_buffer,
          mesh.index_buffer.buffer.0,
          0,
          vk_sys::INDEX_TYPE_UINT16,
        );

        for mesh_to_render in meshes_to_render_by_mesh_id {
          let push_constant = ModelPushConstant {
            model: cgmath::Matrix4::<f32>::from_translation(cgmath::Vector3::<f32>::new(
              mesh_to_render.pos[0],
              mesh_to_render.pos[1],
              mesh_to_render.pos[2],
            )) * cgmath::Matrix4::<f32>::from_scale(mesh_to_render.scale),
          };
          self.base_renderer.device.ptrs().CmdPushConstants(
            *command_buffer,
            self.pipeline_layout,
            vk_sys::SHADER_STAGE_VERTEX_BIT,
            0, /* offset */
            push_constant_size as u32,
            &push_constant as *const ModelPushConstant as *const c_void,
          );
          self.base_renderer.device.ptrs().CmdDrawIndexed(
            *command_buffer,
            mesh.index_buffer.num_indexes,
            1,
            0,
            0,
            0,
          );
        }
      }

      self
        .base_renderer
        .device
        .ptrs()
        .CmdEndRenderPass(*command_buffer);
      do_or_die!(lite::util::dooy("end command buffer", &|| unsafe {
        self
          .base_renderer
          .device
          .ptrs()
          .EndCommandBuffer(*command_buffer)
      }))
    }

    let wait_semaphores = [self.image_available_semaphore];
    let wait_stages = [vk_sys::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];
    let signal_semaphores = [self.render_finished_semaphore];
    let submit_info = vk_sys::SubmitInfo {
      sType: vk_sys::STRUCTURE_TYPE_SUBMIT_INFO,
      pNext: ptr::null(),
      waitSemaphoreCount: 1,
      pWaitSemaphores: wait_semaphores.as_ptr(),
      pWaitDstStageMask: wait_stages.as_ptr(),
      commandBufferCount: 1,
      pCommandBuffers: self.command_buffers.get(image_index as usize).unwrap(),
      signalSemaphoreCount: 1,
      pSignalSemaphores: signal_semaphores.as_ptr(),
    };

    let queue = self.base_renderer.device.get_device_queue(
      self.base_renderer.device_spec.gfx_queue_family_idx,
      0, /* queue_index */
    );

    do_or_die!(lite::util::dooy("queue submit", &|| unsafe {
      self
        .base_renderer
        .device
        .ptrs()
        .QueueSubmit(queue, 1, &submit_info, *command_buffer_fence)
    }));

    let swapchains = [self.base_renderer.swapchain.swapchain];
    let present_info_khr = vk_sys::PresentInfoKHR {
      sType: vk_sys::STRUCTURE_TYPE_PRESENT_INFO_KHR,
      pNext: ptr::null(),
      waitSemaphoreCount: 1,
      pWaitSemaphores: signal_semaphores.as_ptr(),
      swapchainCount: 1,
      pSwapchains: swapchains.as_ptr(),
      pImageIndices: &image_index,
      pResults: ptr::null_mut(),
    };

    do_or_die!(lite::util::dooy("queue present", &|| unsafe {
      self
        .base_renderer
        .device
        .ptrs()
        .QueuePresentKHR(queue, &present_info_khr)
    }));
  }
}

impl<'window> Drop for GalaxyBigRenderer<'window> {
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
    device.destroy_buffer(self.uniform_buffer.buffer.0 /* buffer */);
    device.destroy_image(self.depth_image.image.0 /* image */);
    device.free_memory(self.uniform_buffer.buffer.1 /* deviceMemory */);
    device.free_memory(self.depth_image.image.1 /* deviceMemory */);

    vk::buffer_cache::deallocate_mesh_cache(&device, &mut self.mesh_cache);

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

fn make_standard_vertex_buffer_descriptor() -> VertexBufferDescriptor {
  let pos_attr_desc = vk_sys::VertexInputAttributeDescription {
    binding: 0,
    location: 0,
    format: vk_sys::FORMAT_R32G32B32_SFLOAT,
    offset: offset_of!(Vertex, pos) as u32,
  };

  let norm_attr_desc = vk_sys::VertexInputAttributeDescription {
    binding: 0,
    location: 1,
    format: vk_sys::FORMAT_R32G32B32_SFLOAT,
    offset: offset_of!(Vertex, norm) as u32,
  };

  let binding_description = vk_sys::VertexInputBindingDescription {
    binding: 0,
    stride: std::mem::size_of::<Vertex>() as u32,
    inputRate: vk_sys::VERTEX_INPUT_RATE_VERTEX, /* advance per vertex (instead of per instance) */
  };

  VertexBufferDescriptor {
    attr_descriptions: vec![pos_attr_desc, norm_attr_desc],
    binding_description: binding_description,
  }
}

fn make_mesh_buffers(
  device: &lite::LDevice,
  command_pool: &vk_sys::CommandPool,
  queue: &vk_sys::Queue,
  memory_properties: &vk_sys::PhysicalDeviceMemoryProperties,
  mesh: &Mesh,
) -> lite::RawResult<MeshBufferSet> {
  let vertex_buffer = {
    let vertices = &mesh.vertices;
    let buffer_size = (std::mem::size_of::<Vertex>() * vertices.len()) as u64;

    let vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory) =
      try!(vkbs::make_bound_buffer(
        device,
        buffer_size,
        vk_sys::BUFFER_USAGE_TRANSFER_SRC_BIT,
        vk_sys::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk_sys::MEMORY_PROPERTY_HOST_COHERENT_BIT,
        memory_properties
      ));

    unsafe {
      try!(device.map_vec_data_to_memory(&transfer_device_memory, vertices));
    }

    let vkbs::PreparedBuffer(buffer, device_memory) = try!(vkbs::make_bound_buffer(
      device,
      buffer_size,
      vk_sys::BUFFER_USAGE_TRANSFER_DST_BIT | vk_sys::BUFFER_USAGE_VERTEX_BUFFER_BIT,
      vk_sys::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
      memory_properties
    ));

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

    let buffer_size = (std::mem::size_of::<u16>() * indexes.len()) as u64;

    let vkbs::PreparedBuffer(transfer_buffer, transfer_device_memory) =
      try!(vkbs::make_bound_buffer(
        device,
        buffer_size,
        vk_sys::BUFFER_USAGE_TRANSFER_SRC_BIT,
        vk_sys::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk_sys::MEMORY_PROPERTY_HOST_COHERENT_BIT,
        memory_properties
      ));

    unsafe {
      try!(device.map_vec_data_to_memory(&transfer_device_memory, indexes));
    }

    let vkbs::PreparedBuffer(buffer, device_memory) = try!(vkbs::make_bound_buffer(
      device,
      buffer_size,
      vk_sys::BUFFER_USAGE_TRANSFER_DST_BIT | vk_sys::BUFFER_USAGE_INDEX_BUFFER_BIT,
      vk_sys::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
      memory_properties
    ));

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

  Ok(MeshBufferSet {
    index_buffer: index_buffer,
    vertex_buffer: vertex_buffer,
  })
}

fn make_uniform_buffer(
  device: &lite::LDevice,
  memory_properties: &vk_sys::PhysicalDeviceMemoryProperties,
) -> lite::RawResult<UniformBuffer> {
  let buffer_size = std::mem::size_of::<MVPUniform>();
  let prepared_buffer = try!(vkbs::make_bound_buffer(
    device,
    buffer_size as u64,
    vk_sys::BUFFER_USAGE_UNIFORM_BUFFER_BIT,
    vk_sys::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk_sys::MEMORY_PROPERTY_HOST_COHERENT_BIT,
    memory_properties,
  ));
  Ok(UniformBuffer {
    buffer: prepared_buffer,
  })
}

fn make_depth_image(
  device: &lite::LDevice,
  swapchain: &vkss::LoadedSwapchain,
  depth_format: vk_sys::Format,
  command_pool: &vk_sys::CommandPool,
  queue: &vk_sys::Queue,
  memory_properties: &vk_sys::PhysicalDeviceMemoryProperties,
) -> lite::RawResult<DepthImage> {
  let prepared_image = try!(vkbs::make_image(
    device,
    vk_sys::Extent3D {
      width: swapchain.surface_extent.width,
      height: swapchain.surface_extent.height,
      depth: 1,
    },
    depth_format,
    vk_sys::IMAGE_TILING_OPTIMAL,
    vk_sys::IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
    vk_sys::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
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
    vk_sys::IMAGE_ASPECT_DEPTH_BIT,
  ));

  try!(vkbs::transition_image_layout(
    &device,
    &command_pool,
    &queue,
    &prepared_image.0, /* image */
    depth_format,
    vk_sys::IMAGE_LAYOUT_UNDEFINED,
    vk_sys::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL
  ));

  Ok(DepthImage {
    image: prepared_image,
    image_view: image_view,
  })
}
