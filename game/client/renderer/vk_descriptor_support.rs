extern crate vk_lite as vkl;
extern crate vk_sys as vk;

use std::ptr;
use std::mem;

pub type DescriptorBindingId = u32;

pub struct BufferInfoGenerator {
  buffer_infos: Vec<vk::DescriptorBufferInfo>,
  existing_byte_size: u64,
}

impl BufferInfoGenerator {
  pub fn new() -> BufferInfoGenerator {
    BufferInfoGenerator {
      buffer_infos: Vec::new(),
      existing_byte_size: 0,
    }
  }

  pub unsafe fn push<'selff, T: Sized>(
    &'selff mut self,
    buffer: &vk::Buffer,
  ) -> &'selff mut BufferInfoGenerator {
    let size = std::mem::size_of::<T>() as u64;
    self.buffer_infos.push(vk::DescriptorBufferInfo {
      buffer: *buffer,
      offset: self.existing_byte_size,
      range: size,
    });
    self.existing_byte_size = self.existing_byte_size + size;

    self
  }

  pub fn take_infos(&mut self) -> Vec<vk::DescriptorBufferInfo> {
    let mut swap_vec = Vec::new();
    mem::swap(&mut self.buffer_infos, &mut swap_vec);
    swap_vec
  }
}

pub fn make_descriptor_pool(device: &vkl::LDevice) -> vkl::RawResult<vk::DescriptorPool> {
  let ubo_pool_size = vk::DescriptorPoolSize {
    ty: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
    descriptorCount: 1,
  };
  let sampler_pool_size = vk::DescriptorPoolSize {
    ty: vk::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
    descriptorCount: 1,
  };

  let all_pool_sizes = [ubo_pool_size, sampler_pool_size];
  let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
    sType: vk::STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    maxSets: 1,
    poolSizeCount: all_pool_sizes.len() as u32,
    pPoolSizes: all_pool_sizes.as_ptr(),
  };

  device.create_descriptor_pool(&descriptor_pool_create_info)
}

pub fn make_descriptor_set_layouts(
  device: &vkl::LDevice,
) -> vkl::RawResult<Vec<vk::DescriptorSetLayout>> {
  let ubo_layout_binding = vk::DescriptorSetLayoutBinding {
    binding: 0,
    descriptorType: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
    descriptorCount: 1,
    stageFlags: vk::SHADER_STAGE_VERTEX_BIT,
    pImmutableSamplers: ptr::null(),
  };

  let sampler_layout_binding = vk::DescriptorSetLayoutBinding {
    binding: 1,
    descriptorType: vk::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
    descriptorCount: 1,
    stageFlags: vk::SHADER_STAGE_FRAGMENT_BIT,
    pImmutableSamplers: ptr::null(),
  };

  let all_bindings = [ubo_layout_binding, sampler_layout_binding];

  let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
    sType: vk::STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    bindingCount: all_bindings.len() as u32,
    pBindings: all_bindings.as_ptr(),
  };

  device
    .create_descriptor_set_layout(&descriptor_set_layout_create_info)
    .map(|l| vec![l])
}

pub fn make_descriptor_sets(
  device: &vkl::LDevice,
  descriptor_set_layouts: &Vec<vk::DescriptorSetLayout>,
  descriptor_pool: &vk::DescriptorPool,
) -> vkl::RawResult<Vec<vk::DescriptorSet>> {
  let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo {
    sType: vk::STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
    pNext: ptr::null(),
    descriptorPool: *descriptor_pool,
    descriptorSetCount: 1,
    pSetLayouts: descriptor_set_layouts.as_ptr(),
  };

  device.allocate_descriptor_sets(&descriptor_set_allocate_info)
}

pub fn write_descriptors(
  device: &vkl::LDevice,
  descriptor_set: &vk::DescriptorSet,
  descriptor_binding_id: DescriptorBindingId,
  descriptor_set_infos: Vec<vk::DescriptorBufferInfo>,
) {
  let descriptor_set_write = vk::WriteDescriptorSet {
    sType: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
    pNext: ptr::null(),
    dstSet: *descriptor_set,
    dstBinding: descriptor_binding_id,
    dstArrayElement: 0,
    descriptorCount: descriptor_set_infos.len() as u32,
    descriptorType: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
    pImageInfo: ptr::null(),
    pBufferInfo: descriptor_set_infos.as_ptr(),
    pTexelBufferView: ptr::null(),
  };

  let all_descriptor_set_writes = vec![descriptor_set_write];
  device.update_descriptor_sets(
    &all_descriptor_set_writes,
    &Vec::new(), /* descriptor_set_copies */
  );
}

pub fn write_texture_image_descriptor(
  device: &vkl::LDevice,
  texture_image_view: &vk::ImageView,
  texture_sampler: &vk::Sampler,
  descriptor_set: &vk::DescriptorSet,
  descriptor_binding_id: DescriptorBindingId,
) {
  let sampler_descriptor_image_info = vk::DescriptorImageInfo {
    imageLayout: vk::IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
    imageView: *texture_image_view,
    sampler: *texture_sampler,
  };

  let sampler_descriptor_set_write = vk::WriteDescriptorSet {
    sType: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
    pNext: ptr::null(),
    dstSet: *descriptor_set,
    dstBinding: descriptor_binding_id,
    dstArrayElement: 0,
    descriptorCount: 1,
    descriptorType: vk::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
    pImageInfo: &sampler_descriptor_image_info,
    pBufferInfo: ptr::null(),
    pTexelBufferView: ptr::null(),
  };

  let all_descriptor_set_writes = vec![sampler_descriptor_set_write];
  device.update_descriptor_sets(
    &all_descriptor_set_writes,
    &Vec::new(), /* descriptor_set_copies */
  );
}
