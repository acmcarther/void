use swapchain_support;
use std::ptr;
use lite;
use vk_sys;

pub struct PreparedBuffer(pub vk_sys::Buffer, pub vk_sys::DeviceMemory);
pub struct PreparedImage(pub vk_sys::Image, pub vk_sys::DeviceMemory);

pub fn make_bound_buffer(
  device: &lite::LDevice,
  buffer_size: vk_sys::DeviceSize,
  buffer_usage: vk_sys::BufferUsageFlags,
  memory_property_flags: vk_sys::MemoryPropertyFlags,
  memory_properties: &vk_sys::PhysicalDeviceMemoryProperties,
) -> lite::RawResult<PreparedBuffer> {
  // (vk_sys::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk_sys::MEMORY_PROPERTY_HOST_COHERENT_BIT)
  let buffer_create_info = vk_sys::BufferCreateInfo {
    sType: vk_sys::STRUCTURE_TYPE_BUFFER_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    size: buffer_size,
    usage: buffer_usage,
    // TODO(acmcarther): This can be EXCLUSIVE when I start using memory gating
    sharingMode: vk_sys::SHARING_MODE_EXCLUSIVE,
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

  let memory_allocate_info = vk_sys::MemoryAllocateInfo {
    sType: vk_sys::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
    pNext: ptr::null(),
    allocationSize: memory_requirements.size,
    memoryTypeIndex: suitable_mem_idx,
  };

  let device_memory = try!(device.allocate_memory(&memory_allocate_info));

  unsafe {
    try!(device.bind_buffer_memory(&buffer, &device_memory));
  }
  Ok(PreparedBuffer(buffer, device_memory))
}

pub fn make_image(
  device: &lite::LDevice,
  image_extent: vk_sys::Extent3D,
  format: vk_sys::Format,
  tiling: vk_sys::ImageTiling,
  image_usage: vk_sys::BufferUsageFlags,
  memory_property_flags: vk_sys::MemoryPropertyFlags,
  memory_properties: &vk_sys::PhysicalDeviceMemoryProperties,
) -> lite::RawResult<PreparedImage> {
  let image_create_info = vk_sys::ImageCreateInfo {
    sType: vk_sys::STRUCTURE_TYPE_IMAGE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    imageType: vk_sys::IMAGE_TYPE_2D,
    format: format,
    extent: image_extent,
    mipLevels: 1,
    arrayLayers: 1,
    samples: vk_sys::SAMPLE_COUNT_1_BIT,
    tiling: tiling,
    usage: image_usage,
    sharingMode: vk_sys::SHARING_MODE_EXCLUSIVE,
    queueFamilyIndexCount: 0,
    pQueueFamilyIndices: ptr::null(),
    initialLayout: vk_sys::IMAGE_LAYOUT_UNDEFINED,
  };

  let image = try!(device.create_image(&image_create_info));
  let memory_requirements = device.get_image_memory_requirements(&image);

  let suitable_mem_idx = find_suitable_memory_idx(
    &memory_requirements,
    memory_property_flags,
    memory_properties,
  );
  let memory_allocate_info = vk_sys::MemoryAllocateInfo {
    sType: vk_sys::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
    pNext: ptr::null(),
    allocationSize: memory_requirements.size,
    memoryTypeIndex: suitable_mem_idx,
  };

  let device_memory = try!(device.allocate_memory(&memory_allocate_info));

  Ok(PreparedImage(image, device_memory))
}

fn begin_one_time_command(
  device: &lite::LDevice,
  transfer_command_pool: &vk_sys::CommandPool,
) -> lite::RawResult<vk_sys::CommandBuffer> {
  // TODO(acmcarther): use a different, transient, command pool for this
  let command_buffer_allocate_info = vk_sys::CommandBufferAllocateInfo {
    sType: vk_sys::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    pNext: ptr::null(),
    commandPool: *transfer_command_pool,
    level: vk_sys::COMMAND_BUFFER_LEVEL_PRIMARY,
    commandBufferCount: 1,
  };

  let mut command_buffers = try!(device.allocate_command_buffers(&command_buffer_allocate_info));

  let command_buffer_begin_info = vk_sys::CommandBufferBeginInfo {
    sType: vk_sys::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
    pNext: ptr::null(),
    flags: vk_sys::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
    pInheritanceInfo: ptr::null(),
  };

  unsafe {
    let device_ptrs = device.ptrs();

    let command_buffer = command_buffers.remove(0);

    try!(lite::util::dooy("start one time command buffer", &|| {
      device_ptrs.BeginCommandBuffer(command_buffer, &command_buffer_begin_info)
    }));

    Ok(command_buffer)
  }
}

fn end_one_time_command(
  device: &lite::LDevice,
  command_buffer: vk_sys::CommandBuffer,
  transfer_command_pool: &vk_sys::CommandPool,
  transfer_queue: &vk_sys::Queue,
) -> lite::RawResult<()> {
  unsafe {
    let device_ptrs = device.ptrs();
    try!(lite::util::dooy("end one time command buffer", &|| {
      device_ptrs.EndCommandBuffer(command_buffer)
    }));

    let submit_info = vk_sys::SubmitInfo {
      sType: vk_sys::STRUCTURE_TYPE_SUBMIT_INFO,
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
    try!(lite::util::dooy("queue submit copy buffer", &|| {
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

pub fn format_includes_stencil(format: vk_sys::Format) -> bool {
  format == vk_sys::FORMAT_D32_SFLOAT_S8_UINT || format == vk_sys::FORMAT_D24_UNORM_S8_UINT
}

#[allow(non_snake_case)]
pub fn transition_image_layout(
  device: &lite::LDevice,
  transfer_command_pool: &vk_sys::CommandPool,
  transfer_queue: &vk_sys::Queue,
  image: &vk_sys::Image,
  format: vk_sys::Format,
  old_image_layout: vk_sys::ImageLayout,
  new_image_layout: vk_sys::ImageLayout,
) -> lite::RawResult<()> {
  let command_buffer = try!(begin_one_time_command(device, transfer_command_pool));

  let (srcAccessMask, dstAccessMask, srcStageMask, dstStageMask, aspectMask) = match (old_image_layout, new_image_layout) {
      (vk_sys::IMAGE_LAYOUT_UNDEFINED, vk_sys::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL) => (
        0,
        vk_sys::ACCESS_TRANSFER_WRITE_BIT,
        vk_sys::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
        vk_sys::PIPELINE_STAGE_TRANSFER_BIT,
        vk_sys::IMAGE_ASPECT_COLOR_BIT,
      ),
      (vk_sys::IMAGE_LAYOUT_UNDEFINED, vk_sys::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL) if format_includes_stencil(format) => (
        0,
        vk_sys::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT | vk_sys::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
        vk_sys::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
        vk_sys::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
        vk_sys::IMAGE_ASPECT_DEPTH_BIT | vk_sys::IMAGE_ASPECT_STENCIL_BIT,
      ),
      (vk_sys::IMAGE_LAYOUT_UNDEFINED, vk_sys::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL) /* no stencil */ => (
        0,
        vk_sys::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT | vk_sys::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
        vk_sys::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
        vk_sys::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
        vk_sys::IMAGE_ASPECT_DEPTH_BIT,
      ),
      (vk_sys::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, vk_sys::IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL) => (
        vk_sys::ACCESS_TRANSFER_WRITE_BIT,
        vk_sys::ACCESS_SHADER_READ_BIT,
        vk_sys::PIPELINE_STAGE_TRANSFER_BIT,
        vk_sys::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
        vk_sys::IMAGE_ASPECT_COLOR_BIT,
      ),
      _ => {
        panic!(
          "Vulkan Unsupported image transition {} -> {}",
          old_image_layout,
          new_image_layout
        );
      },
    };

  let image_memory_barrier = vk_sys::ImageMemoryBarrier {
    sType: vk_sys::STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
    pNext: ptr::null(),
    srcAccessMask: srcAccessMask,
    dstAccessMask: dstAccessMask,
    oldLayout: old_image_layout,
    newLayout: new_image_layout,
    srcQueueFamilyIndex: vk_sys::QUEUE_FAMILY_IGNORED,
    dstQueueFamilyIndex: vk_sys::QUEUE_FAMILY_IGNORED,
    image: *image,
    subresourceRange: vk_sys::ImageSubresourceRange {
      aspectMask: aspectMask,
      baseMipLevel: 0,
      levelCount: 1,
      baseArrayLayer: 0,
      layerCount: 1,
    },
  };

  unsafe {
    let device_ptrs = device.ptrs();

    device_ptrs.CmdPipelineBarrier(
      command_buffer,
      srcStageMask, /* srcStageMask */
      dstStageMask, /* dstStageMask */
      0,            /* dependencyFlags */
      0,            /* memoryBarrierCount */
      ptr::null(),
      0, /* bufferMemoryBarrierCount */
      ptr::null(),
      1, /* imageBarrierCount */
      &image_memory_barrier,
    );
  }

  end_one_time_command(
    device,
    command_buffer,
    transfer_command_pool,
    transfer_queue,
  )
}

pub fn copy_buffer_into_image(
  device: &lite::LDevice,
  buffer: &vk_sys::Buffer,
  image: &vk_sys::Image,
  width: u32,
  height: u32,
  transfer_command_pool: &vk_sys::CommandPool,
  transfer_queue: &vk_sys::Queue,
) -> lite::RawResult<()> {
  let command_buffer = try!(begin_one_time_command(device, transfer_command_pool));

  let region = vk_sys::BufferImageCopy {
    bufferOffset: 0,
    bufferRowLength: 0,
    bufferImageHeight: 0,
    imageSubresource: vk_sys::ImageSubresourceLayers {
      aspectMask: vk_sys::IMAGE_ASPECT_COLOR_BIT,
      mipLevel: 0,
      baseArrayLayer: 0,
      layerCount: 1,
    },
    imageOffset: vk_sys::Offset3D { x: 0, y: 0, z: 0 },
    imageExtent: vk_sys::Extent3D {
      width: width,
      height: height,
      depth: 1,
    },
  };

  unsafe {
    let device_ptrs = device.ptrs();

    device_ptrs.CmdCopyBufferToImage(
      command_buffer,
      *buffer,
      *image,
      vk_sys::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
      1, /* regionCount */
      &region,
    );
  }

  end_one_time_command(
    device,
    command_buffer,
    transfer_command_pool,
    transfer_queue,
  )
}

pub fn copy_buffer(
  device: &lite::LDevice,
  transfer_command_pool: &vk_sys::CommandPool,
  src_buffer: &vk_sys::Buffer,
  dst_buffer: &vk_sys::Buffer,
  size: u64,
  transfer_queue: &vk_sys::Queue,
) -> lite::RawResult<()> {
  let command_buffer = try!(begin_one_time_command(device, transfer_command_pool));

  unsafe {
    let device_ptrs = device.ptrs();

    let copy_region = vk_sys::BufferCopy {
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
  device: &lite::LDevice,
  image_views: &Vec<vk_sys::ImageView>,
  depth_image_view: &vk_sys::ImageView,
  swapchain: &swapchain_support::LoadedSwapchain,
  render_pass: &vk_sys::RenderPass,
) -> lite::RawResult<Vec<vk_sys::Framebuffer>> {
  let mut framebuffers = Vec::with_capacity(image_views.len());
  for swapchain_image_view in image_views.iter() {
    let all_attachments = [*swapchain_image_view, *depth_image_view];
    let framebuffer_create_info = vk_sys::FramebufferCreateInfo {
      sType: vk_sys::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
      pNext: ptr::null(),
      flags: 0,
      renderPass: *render_pass,
      attachmentCount: all_attachments.len() as u32,
      pAttachments: all_attachments.as_ptr(),
      width: swapchain.surface_extent.width,
      height: swapchain.surface_extent.height,
      layers: 1,
    };

    framebuffers.push(try!(device.create_framebuffer(&framebuffer_create_info)));
  }

  Ok(framebuffers)
}

pub fn make_command_buffers(
  device: &lite::LDevice,
  command_pool: &vk_sys::CommandPool,
  buffer_count: u32,
) -> lite::RawResult<Vec<vk_sys::CommandBuffer>> {
  let command_buffer_allocate_info = vk_sys::CommandBufferAllocateInfo {
    sType: vk_sys::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    pNext: ptr::null(),
    commandPool: *command_pool,
    level: vk_sys::COMMAND_BUFFER_LEVEL_PRIMARY,
    commandBufferCount: buffer_count,
  };

  device.allocate_command_buffers(&command_buffer_allocate_info)
}

fn find_suitable_memory_idx(
  memory_requirements: &vk_sys::MemoryRequirements,
  memory_property_flags: vk_sys::MemoryPropertyFlags,
  memory_properties: &vk_sys::PhysicalDeviceMemoryProperties,
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
