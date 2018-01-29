#[macro_use]
extern crate log;
extern crate vk_device_support as vkds;
extern crate vk_lite as vkl;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

use std::ptr;

fn find_suitable_memory_idx(
  memory_requirements: &vk::MemoryRequirements,
  memory_property_flags: vk::MemoryPropertyFlags,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> u32 {
  // TODO(acmcarther): This is potentially in the hot path but it looks inefficient.
  // Consider profiling and optimizing if necessary.
  (0..memory_properties.memoryTypeCount)
    .filter(|idx| memory_requirements.memoryTypeBits & (1u32 << idx) > 0 /* type suitable */)
    .filter(|idx| {
      let prop_flags = memory_properties.memoryTypes[*idx as usize].propertyFlags;
      prop_flags & memory_property_flags > 0 /* props suitable */
    })
    .next()
    .expect("Vulkan: couldn't find physical device memory suitable for buffer or image")
}


pub struct PreparedBuffer(pub vk::Buffer, pub vk::DeviceMemory);
pub struct PreparedImage(pub vk::Image, pub vk::DeviceMemory);

pub fn make_buffer(
  device: &vkl::LDevice,
  buffer_size: vk::DeviceSize,
  buffer_usage: vk::BufferUsageFlags,
  memory_property_flags: vk::MemoryPropertyFlags,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> vkl::RawResult<PreparedBuffer> {
  // (vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT)
  let buffer_create_info = vk::BufferCreateInfo {
    sType: vk::STRUCTURE_TYPE_BUFFER_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    size: buffer_size,
    usage: buffer_usage,
    // TODO(acmcarther): This can be EXCLUSIVE when I start using memory gating
    sharingMode: vk::SHARING_MODE_EXCLUSIVE,
    queueFamilyIndexCount: 0,
    pQueueFamilyIndices: ptr::null(),
  };

  let buffer = try!(device.create_buffer(&buffer_create_info));

  let memory_requirements = device.get_buffer_memory_requirements(&buffer);

  let suitable_mem_idx = find_suitable_memory_idx(
    &memory_requirements,
    memory_property_flags,
    memory_properties,
  );

  let memory_allocate_info = vk::MemoryAllocateInfo {
    sType: vk::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
    pNext: ptr::null(),
    allocationSize: memory_requirements.size,
    memoryTypeIndex: suitable_mem_idx,
  };

  let device_memory = try!(device.allocate_memory(&memory_allocate_info));

  Ok(PreparedBuffer(buffer, device_memory))
}

pub fn make_image(
  device: &vkl::LDevice,
  image_extent: vk::Extent3D,
  image_usage: vk::BufferUsageFlags,
  memory_property_flags: vk::MemoryPropertyFlags,
  memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> vkl::RawResult<PreparedImage> {
  let image_create_info = vk::ImageCreateInfo {
    sType: vk::STRUCTURE_TYPE_IMAGE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    imageType: vk::IMAGE_TYPE_2D,
    format: vk::FORMAT_R8G8B8A8_UNORM,
    extent: image_extent,
    mipLevels: 1,
    arrayLayers: 1,
    samples: vk::SAMPLE_COUNT_1_BIT,
    tiling: vk::IMAGE_TILING_OPTIMAL,
    usage: vk::IMAGE_USAGE_TRANSFER_DST_BIT | vk::IMAGE_USAGE_SAMPLED_BIT,
    sharingMode: vk::SHARING_MODE_EXCLUSIVE,
    queueFamilyIndexCount: 0,
    pQueueFamilyIndices: ptr::null(),
    initialLayout: vk::IMAGE_LAYOUT_UNDEFINED,
  };

  let image = try!(device.create_image(&image_create_info));
  let memory_requirements = device.get_image_memory_requirements(&image);

  let suitable_mem_idx = find_suitable_memory_idx(
    &memory_requirements,
    memory_property_flags,
    memory_properties,
  );
  let memory_allocate_info = vk::MemoryAllocateInfo {
    sType: vk::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
    pNext: ptr::null(),
    allocationSize: memory_requirements.size,
    memoryTypeIndex: suitable_mem_idx,
  };

  let device_memory = try!(device.allocate_memory(&memory_allocate_info));

  Ok(PreparedImage(image, device_memory))
}


fn begin_one_time_command(
  device: &vkl::LDevice,
  transfer_command_pool: &vk::CommandPool,
) -> vkl::RawResult<vk::CommandBuffer> {
  // TODO(acmcarther): use a different, transient, command pool for this
  let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
    sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    pNext: ptr::null(),
    commandPool: *transfer_command_pool,
    level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
    commandBufferCount: 1,
  };

  let mut command_buffers = try!(device.allocate_command_buffers(&command_buffer_allocate_info));

  let command_buffer_begin_info = vk::CommandBufferBeginInfo {
    sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
    pNext: ptr::null(),
    flags: vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
    pInheritanceInfo: ptr::null(),
  };

  unsafe {
    let device_ptrs = device.ptrs();

    let command_buffer = command_buffers.remove(0);

    try!(vkl::util::dooy("start one time command buffer", &|| {
      device_ptrs.BeginCommandBuffer(command_buffer, &command_buffer_begin_info)
    }));

    Ok(command_buffer)
  }
}

fn end_one_time_command(
  device: &vkl::LDevice,
  command_buffer: vk::CommandBuffer,
  transfer_command_pool: &vk::CommandPool,
  transfer_queue: &vk::Queue,
) -> vkl::RawResult<()> {
  unsafe {
    let device_ptrs = device.ptrs();
    try!(vkl::util::dooy("end one time command buffer", &|| {
      device_ptrs.EndCommandBuffer(command_buffer)
    }));

    let submit_info = vk::SubmitInfo {
      sType: vk::STRUCTURE_TYPE_SUBMIT_INFO,
      pNext: ptr::null(),
      waitSemaphoreCount: 0,
      pWaitSemaphores: ptr::null(),
      pWaitDstStageMask: ptr::null(),
      commandBufferCount: 1,
      pCommandBuffers: &command_buffer,
      signalSemaphoreCount: 0,
      pSignalSemaphores: ptr::null(),
    };

    let submit_infos = vec![submit_info];
    try!(vkl::util::dooy("queue submit copy buffer", &|| {
      device.ptrs().QueueSubmit(
        *transfer_queue,
        1, /* submitCount */
        submit_infos.as_ptr(),
        0, /* fence */
      )
    }));

    try!(device.queue_wait_idle(transfer_queue));
    device.free_command_buffers(transfer_command_pool, vec![command_buffer]);
  }

  Ok(())
}

pub fn copy_buffer(
  device: &vkl::LDevice,
  transfer_command_pool: &vk::CommandPool,
  src_buffer: &vk::Buffer,
  dst_buffer: &vk::Buffer,
  size: u64,
  transfer_queue: &vk::Queue,
) -> vkl::RawResult<()> {
  let command_buffer = try!(begin_one_time_command(device, transfer_command_pool));

  unsafe {
    let device_ptrs = device.ptrs();

    let copy_region = vk::BufferCopy {
      srcOffset: 0,
      dstOffset: 0,
      size: size,
    };

    device_ptrs.CmdCopyBuffer(
      command_buffer,
      *src_buffer,
      *dst_buffer,
      1, /* regionCount */
      &copy_region,
    );
  }

  end_one_time_command(
    device,
    command_buffer,
    transfer_command_pool,
    transfer_queue,
  )
}

pub fn make_framebuffers(
  device: &vkl::LDevice,
  image_views: &Vec<vk::ImageView>,
  swapchain: &vkss::LoadedSwapchain,
  render_pass: &vk::RenderPass,
) -> vkl::RawResult<Vec<vk::Framebuffer>> {
  let mut framebuffers = Vec::with_capacity(image_views.len());
  for swapchain_image_view in image_views.iter() {
    let framebuffer_create_info = vk::FramebufferCreateInfo {
      sType: vk::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
      pNext: ptr::null(),
      flags: 0,
      renderPass: *render_pass,
      attachmentCount: 1,
      pAttachments: swapchain_image_view,
      width: swapchain.surface_extent.width,
      height: swapchain.surface_extent.height,
      layers: 1,
    };

    framebuffers.push(try!(device.create_framebuffer(&framebuffer_create_info)));
  }

  Ok(framebuffers)
}

pub fn make_command_buffers(
  device: &vkl::LDevice,
  command_pool: &vk::CommandPool,
  buffer_count: u32,
) -> vkl::RawResult<Vec<vk::CommandBuffer>> {
  let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
    sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    pNext: ptr::null(),
    commandPool: *command_pool,
    level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
    commandBufferCount: buffer_count,
  };

  device.allocate_command_buffers(&command_buffer_allocate_info)
}
