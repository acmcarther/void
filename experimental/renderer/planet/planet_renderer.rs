extern crate cgmath;
extern crate gfx_basics;
extern crate icosphere;
#[macro_use]
extern crate log;
#[macro_use]
extern crate memoffset;
extern crate vk_application;
extern crate vk_base_renderer;
extern crate vk_basics;
extern crate vk_buffer_cache;
extern crate vk_buffer_support as vkbs;
extern crate vk_descriptor_support as vkdrs;
extern crate vk_device_support as vkds;
extern crate vk_instance_support as vkis;
#[macro_use(do_or_die)]
extern crate vk_lite as vkl;
extern crate vk_pipeline_support as vkps;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

use cgmath::Matrix4;
use gfx_basics::Mesh;
use gfx_basics::Vertex;
use std::mem;
use vk_application::Application as VkApplication;
use vk_application::ApplicationRendererInitializer;
use vk_base_renderer::BaseRenderer;
use vk_buffer_cache::IndexBuffer;
use vk_buffer_cache::MeshBufferSet;
use vk_buffer_cache::MeshCache;
use vk_buffer_cache::VertexBuffer;
use vk_buffer_cache::VertexBufferDescriptor;
use vk_buffer_cache::VertexBufferDescriptorCache;
use vk_buffer_cache::VertexBufferDescriptorId;
use vkdrs::BufferInfoGenerator;
use vkps::PushConstantRangeGenerator;

pub const STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID: VertexBufferDescriptorId = 1u32;

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

pub struct PlanetARI;

impl ApplicationRendererInitializer for PlanetARI {
  fn make_push_constant_ranges(&mut self) -> Vec<vk::PushConstantRange> {
    PushConstantRangeGenerator::new()
      .push::<ModelPushConstant>(vk::SHADER_STAGE_VERTEX_BIT)
      .take_ranges()
  }

  fn init_mesh_cache(&mut self, mesh_cache: &mut MeshCache, base_renderer: &mut BaseRenderer) {
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

    {
      let mesh = icosphere::icosphere(0u32 /* iterations */);
      // UNRAP: pop guaranteed to be present (by make_mesh_buffers api)
      let mesh_buffer_set = do_or_die!(vk_basics::make_mesh_buffers(
        &base_renderer.device,
        &copy_command_pool,
        &queue,
        &base_renderer.device_spec.memory_properties,
        &vec![&mesh],
        STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID,
      )).pop()
        .unwrap();

      mesh_cache.insert(0 /* mesh_id */, mesh_buffer_set);
    }
  }

  fn init_vbd_cache(&mut self, vbd_cache: &mut VertexBufferDescriptorCache) {
    vbd_cache.insert(
      STANDARD_VERTEX_BUFFER_DESCRIPTOR_ID,
      vk_basics::vbd_for_vertex(),
    );
  }

  fn make_uniform_buffers(
    &mut self,
    device: &vkl::LDevice,
    memory_properties: &vk::PhysicalDeviceMemoryProperties,
  ) -> vkl::RawResult<(Vec<vkbs::PreparedBuffer>, Vec<vk::DescriptorBufferInfo>)> {
    let buffer_size = mem::size_of::<MVPUniform>();
    let prepared_buffer = try!(vkbs::make_bound_buffer(
      device,
      buffer_size as u64,
      vk::BUFFER_USAGE_UNIFORM_BUFFER_BIT,
      vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT,
      memory_properties,
    ));
    let uniform_buffer_infos = unsafe {
      BufferInfoGenerator::new()
        .push::<MVPUniform>(&prepared_buffer.0 /* buffer */)
        .take_infos()
    };
    Ok((vec![prepared_buffer], uniform_buffer_infos))
  }

  fn make_vert_shader_module(&mut self, device: &vkl::LDevice) -> vkl::RawResult<vk::ShaderModule> {
    vkl::builtins::make_shader_module(
      &device,
      include_bytes!(
        "../../../bazel-out/k8-fastbuild/genfiles/experimental/renderer/planet/planet_vert_shader.spv"
      ),
    )
  }

  fn make_frag_shader_module(&mut self, device: &vkl::LDevice) -> vkl::RawResult<vk::ShaderModule> {
    vkl::builtins::make_shader_module(
      &device,
      include_bytes!(
        "../../../bazel-out/k8-fastbuild/genfiles/experimental/renderer/planet/planet_frag_shader.spv"
      ),
    )
  }

  fn get_all_attr_desc(
    &mut self,
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

  fn get_all_vert_buf_desc(
    &mut self,
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
