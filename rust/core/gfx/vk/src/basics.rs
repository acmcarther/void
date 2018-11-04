use lite;
use vk_sys;
use gfx_basics::Mesh;
use gfx_basics::Vertex;
use std::mem;
use buffer_support as bs;
use buffer_cache::VertexBufferDescriptor;
use buffer_cache::VertexBufferDescriptorId;
use buffer_cache::IndexBuffer;
use buffer_cache::MeshBufferSet;
use buffer_cache::VertexBuffer;

/** Produces a VertexBufferDescriptor for the Vertex type. */
pub fn vbd_for_vertex() -> VertexBufferDescriptor {
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
    stride: mem::size_of::<Vertex>() as u32,
    inputRate: vk_sys::VERTEX_INPUT_RATE_VERTEX, /* advance per vertex (instead of per instance) */
  };

  VertexBufferDescriptor {
    attr_descriptions: vec![pos_attr_desc, norm_attr_desc],
    binding_description: binding_description,
  }
}

/** Produces a list of MeshBufferSet for a set of provided meshes. */
pub fn make_mesh_buffers(
  device: &lite::LDevice,
  command_pool: &vk_sys::CommandPool,
  queue: &vk_sys::Queue,
  memory_properties: &vk_sys::PhysicalDeviceMemoryProperties,
  // N.B.: Internals need to change if Mesh type changes (see VertexBuffer definition)
  meshes: &Vec<&Mesh>,
  vertex_vbd_id: VertexBufferDescriptorId,
) -> lite::RawResult<Vec<MeshBufferSet>> {
  // UNWRAP: Safe, guaranteed to be an element from `max`
  let maximal_vbuffer_size = (mem::size_of::<Vertex>()
    * meshes.iter().map(|mesh| mesh.vertices.len()).max().unwrap())
    as u64;
  let maximal_ibuffer_size =
    (mem::size_of::<u16>() * meshes.iter().map(|mesh| mesh.indices.len()).max().unwrap()) as u64;

  let bs::PreparedBuffer(vertex_transfer_buffer, vertex_transfer_device_memory) =
    try!(bs::make_bound_buffer(
      device,
      maximal_vbuffer_size,
      vk_sys::BUFFER_USAGE_TRANSFER_SRC_BIT,
      vk_sys::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk_sys::MEMORY_PROPERTY_HOST_COHERENT_BIT,
      memory_properties
    ));
  let bs::PreparedBuffer(index_transfer_buffer, index_transfer_device_memory) =
    try!(bs::make_bound_buffer(
      device,
      maximal_ibuffer_size,
      vk_sys::BUFFER_USAGE_TRANSFER_SRC_BIT,
      vk_sys::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk_sys::MEMORY_PROPERTY_HOST_COHERENT_BIT,
      memory_properties
    ));

  let mut mesh_buffer_sets = Vec::new();
  for mesh in meshes {
    unsafe {
      try!(device.map_vec_data_to_memory(&vertex_transfer_device_memory, &mesh.vertices));
      try!(device.map_vec_data_to_memory(&index_transfer_device_memory, &mesh.indices));
    };

    let vertex_buffer_size = (mem::size_of::<Vertex>() * mesh.vertices.len()) as u64;
    let index_buffer_size = (mem::size_of::<u16>() * mesh.indices.len()) as u64;
    let prepared_vertex_buffer = try!(bs::make_bound_buffer(
      device,
      vertex_buffer_size,
      vk_sys::BUFFER_USAGE_TRANSFER_DST_BIT | vk_sys::BUFFER_USAGE_VERTEX_BUFFER_BIT,
      vk_sys::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
      memory_properties
    ));
    let prepared_index_buffer = try!(bs::make_bound_buffer(
      device,
      index_buffer_size,
      vk_sys::BUFFER_USAGE_TRANSFER_DST_BIT | vk_sys::BUFFER_USAGE_INDEX_BUFFER_BIT,
      vk_sys::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
      memory_properties
    ));

    // Perform device copy in either transfer queue, or graphics queue (if we must)
    {
      do_or_die!(bs::copy_buffer(
        &device,
        command_pool,
        &vertex_transfer_buffer,
        &prepared_vertex_buffer.0, /* buffer */
        vertex_buffer_size,
        queue
      ));
      do_or_die!(bs::copy_buffer(
        &device,
        command_pool,
        &index_transfer_buffer,
        &prepared_index_buffer.0, /* buffer */
        index_buffer_size,
        queue
      ));
    }

    let vertex_buffer = VertexBuffer {
      buffer: prepared_vertex_buffer,
      // N.B.: This is coupled to the Mesh type. If type changes, change this.
      vertex_buffer_descriptor_id: vertex_vbd_id,
    };
    let index_buffer = IndexBuffer {
      buffer: prepared_index_buffer,
      num_indexes: mesh.indices.len() as u32,
    };

    mesh_buffer_sets.push(MeshBufferSet {
      index_buffer: index_buffer,
      vertex_buffer: vertex_buffer,
    });
  }

  device.destroy_buffer(vertex_transfer_buffer);
  device.free_memory(vertex_transfer_device_memory);
  device.destroy_buffer(index_transfer_buffer);
  device.free_memory(index_transfer_device_memory);

  Ok(mesh_buffer_sets)
}
