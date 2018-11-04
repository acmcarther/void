use lite;
use vk_sys;
use buffer_support;
use std::collections::HashMap;

/** A unique identifier for a vertex buffer descriptor. */
pub type VertexBufferDescriptorId = u32;

/** A unique identifier for a mesh. */
pub type MeshId = u32;

/** A collection of mesh buffers keyed on MeshIds */
pub type MeshCache = HashMap<MeshId, MeshBufferSet>;

/** A mapping from vertex buffer descriptor id to vertex buffer descriptor. */
pub type VertexBufferDescriptorCache = HashMap<VertexBufferDescriptorId, VertexBufferDescriptor>;

/** The vertex describing information required to construct a graphics pipeline for a mesh. */
pub struct VertexBufferDescriptor {
  pub attr_descriptions: Vec<vk_sys::VertexInputAttributeDescription>,
  pub binding_description: vk_sys::VertexInputBindingDescription,
}

/**
 * A vertex buffer.
 *
 * Contains arbitrary vertices, along with a id to the descriptor that describes their form.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
pub struct VertexBuffer {
  pub vertex_buffer_descriptor_id: VertexBufferDescriptorId,
  pub buffer: buffer_support::PreparedBuffer,
}

/**
 * An index buffer
 *
 * Contains indexes into an arbitrary vertex buffer indicating the triangles that form a mesh's
 * shape.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
pub struct IndexBuffer {
  pub buffer: buffer_support::PreparedBuffer,
  pub num_indexes: u32,
}

/**
 * Allocated buffers for a particular mesh.
 *
 * UNSAFE: Requires manual deallocation by the device that instantiated it.
 */
pub struct MeshBufferSet {
  pub vertex_buffer: VertexBuffer,
  pub index_buffer: IndexBuffer,
}

impl VertexBufferDescriptor {
  pub fn binding_description_cloned(&self) -> vk_sys::VertexInputBindingDescription {
    vk_sys::VertexInputBindingDescription {
      binding: self.binding_description.binding,
      stride: self.binding_description.stride,
      inputRate: self.binding_description.inputRate,
    }
  }
}

pub fn deallocate_mesh_cache(device: &lite::LDevice, mesh_cache: &mut MeshCache) {
  for (_, mesh) in mesh_cache.drain() {
    device.destroy_buffer(mesh.index_buffer.buffer.0 /* inner buffer */);
    device.destroy_buffer(mesh.vertex_buffer.buffer.0 /* inner buffer */);
    device.free_memory(mesh.index_buffer.buffer.1 /* deviceMemory */);
    device.free_memory(mesh.vertex_buffer.buffer.1 /* deviceMemory */);
  }
}
