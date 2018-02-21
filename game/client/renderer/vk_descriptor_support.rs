extern crate vk_lite as vkl;
extern crate vk_sys as vk;

use std::ptr;

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

pub fn write_descriptor<T>(
  device: &vkl::LDevice,
  uniform_buffer: &vk::Buffer,
  descriptor_set: &vk::DescriptorSet,
  texture_image_view: &vk::ImageView,
  texture_sampler: &vk::Sampler,
) {
  let ubo_descriptor_buffer_info = vk::DescriptorBufferInfo {
    buffer: *uniform_buffer,
    offset: 0,
    range: std::mem::size_of::<T>() as u64,
  };

  let ubo_descriptor_set_write = vk::WriteDescriptorSet {
    sType: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
    pNext: ptr::null(),
    dstSet: *descriptor_set,
    dstBinding: 0,
    dstArrayElement: 0,
    descriptorCount: 1,
    descriptorType: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
    pImageInfo: ptr::null(),
    pBufferInfo: &ubo_descriptor_buffer_info,
    pTexelBufferView: ptr::null(),
  };

  let sampler_descriptor_image_info = vk::DescriptorImageInfo {
    imageLayout: vk::IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
    imageView: *texture_image_view,
    sampler: *texture_sampler,
  };

  let sampler_descriptor_set_write = vk::WriteDescriptorSet {
    sType: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
    pNext: ptr::null(),
    dstSet: *descriptor_set,
    dstBinding: 1,
    dstArrayElement: 0,
    descriptorCount: 1,
    descriptorType: vk::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
    pImageInfo: &sampler_descriptor_image_info,
    pBufferInfo: ptr::null(),
    pTexelBufferView: ptr::null(),
  };

  let all_descriptor_set_writes = vec![ubo_descriptor_set_write, sampler_descriptor_set_write];
  device.update_descriptor_sets(
    &all_descriptor_set_writes,
    &Vec::new(), /* descriptor_set_copies */
  );
}
