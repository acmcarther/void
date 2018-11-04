use device_support;
use std::ptr;
use lite;
use vk_sys;

/** Picks the best available surface format from the device supported options.  */
fn select_best_surface_format(
  available_formats: &Vec<vk_sys::SurfaceFormatKHR>,
) -> vk_sys::SurfaceFormatKHR {
  // Device has no preference at all
  if available_formats.len() == 1
    && available_formats.get(0).unwrap().format == vk_sys::FORMAT_UNDEFINED
  {
    vk_sys::SurfaceFormatKHR {
      format: vk_sys::FORMAT_B8G8R8A8_UNORM,
      colorSpace: vk_sys::COLOR_SPACE_SRGB_NONLINEAR_KHR,
    }
  } else {
    // Try to find our favorite format
    let ideal_format_opt = available_formats.iter().find(|f| {
      f.format == vk_sys::FORMAT_B8G8R8A8_UNORM && f.colorSpace == vk_sys::COLOR_SPACE_SRGB_NONLINEAR_KHR
    });
    if ideal_format_opt.is_some() {
      vk_sys::SurfaceFormatKHR {
        format: vk_sys::FORMAT_B8G8R8A8_UNORM,
        colorSpace: vk_sys::COLOR_SPACE_SRGB_NONLINEAR_KHR,
      }
    } else {
      warn!("Vulkan: Using a sub-optimal swapchain surface format");
      // Just use the first available
      let first_available_format = available_formats.get(0).unwrap();
      vk_sys::SurfaceFormatKHR {
        format: first_available_format.format,
        colorSpace: first_available_format.colorSpace,
      }
    }
  }
}

/**
 * Picks the best available swapchain presentation mode.
 *
 * Order of preference is:
 * - MAILBOX
 * - IMMEDIATE
 * - FIFO
 */
fn select_best_swapchain_present_mode(
  available_modes: &Vec<vk_sys::PresentModeKHR>,
) -> vk_sys::PresentModeKHR {
  // Choose MAILBOX, IMMEDIATE, FIFO
  let mut mode = vk_sys::PRESENT_MODE_FIFO_KHR /* default */;

  if available_modes.contains(&vk_sys::PRESENT_MODE_MAILBOX_KHR) {
    mode = vk_sys::PRESENT_MODE_MAILBOX_KHR
  } else if available_modes.contains(&vk_sys::PRESENT_MODE_IMMEDIATE_KHR) {
    mode = vk_sys::PRESENT_MODE_IMMEDIATE_KHR
  }

  mode
}

/**
 * Picks the best available surface size.
 *
 * Selects 800x600 if unbounded, else, constrains to window dimensions
 */
#[allow(non_snake_case)]
fn select_surface_extent(surface_capabilities: &vk_sys::SurfaceCapabilitiesKHR) -> vk_sys::Extent2D {
  let DEFAULT_SWAP_WIDTH: u32 = 800 /* px */;
  let DEFAULT_SWAP_HEIGHT: u32 = 600 /* px */;

  let must_use_provided_values = surface_capabilities.currentExtent.width != u32::max_value();
  if must_use_provided_values {
    vk_sys::Extent2D {
      width: surface_capabilities.currentExtent.width,
      height: surface_capabilities.currentExtent.height,
    }
  } else {
    vk_sys::Extent2D {
      width: surface_capabilities.minImageExtent.width.max(
        surface_capabilities
          .maxImageExtent
          .width
          .min(DEFAULT_SWAP_WIDTH),
      ),
      height: surface_capabilities.minImageExtent.height.max(
        surface_capabilities
          .maxImageExtent
          .height
          .min(DEFAULT_SWAP_HEIGHT),
      ),
    }
  }
}

/** Picks number of images in the swapchain, preferring double buffering if available. */
fn select_swapchain_image_count(surface_capabilities: &vk_sys::SurfaceCapabilitiesKHR) -> u32 {
  let desired_image_count = surface_capabilities.minImageCount + 1;

  // Max images may be bounded, use that if its lower than our desired image count
  if surface_capabilities.maxImageCount != 0
    && surface_capabilities.maxImageCount < desired_image_count
  {
    surface_capabilities.maxImageCount
  } else {
    desired_image_count
  }
}

pub struct LoadedSwapchain {
  pub surface_format: vk_sys::SurfaceFormatKHR,
  pub surface_extent: vk_sys::Extent2D,
  pub swapchain_present_mode: vk_sys::PresentModeKHR,
  pub swapchain_image_count: u32,
  pub swapchain: vk_sys::SwapchainKHR,
}

pub fn make_swapchain(
  device: &lite::LDevice,
  physical_device_spec: &device_support::SelectedPhysicalDeviceSpec,
  surface: &vk_sys::SurfaceKHR,
) -> lite::RawResult<LoadedSwapchain> {
  let surface_capabilities = &physical_device_spec.surface_capabilities;
  let surface_format = select_best_surface_format(&physical_device_spec.surface_formats);
  let swapchain_present_mode =
    select_best_swapchain_present_mode(&physical_device_spec.swapchain_present_modes);
  let surface_extent = select_surface_extent(surface_capabilities);
  let swapchain_image_count = select_swapchain_image_count(surface_capabilities);

  let mut queue_family_indices = vec![physical_device_spec.gfx_queue_family_idx];

  let mut image_sharing_mode = vk_sys::SHARING_MODE_EXCLUSIVE;
  if let Some(qf_idx) = physical_device_spec.dedicated_transfer_queue_family_idx_opt {
    queue_family_indices.push(qf_idx);
    image_sharing_mode = vk_sys::SHARING_MODE_CONCURRENT;
  }
  let swapchain_create_info_khr = {
    vk_sys::SwapchainCreateInfoKHR {
      sType: vk_sys::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
      pNext: ptr::null(),
      flags: 0,
      minImageCount: swapchain_image_count,
      imageFormat: surface_format.format,
      imageColorSpace: surface_format.colorSpace,
      imageExtent: vk_sys::Extent2D {
        width: surface_extent.width,
        height: surface_extent.height,
      },
      imageArrayLayers: 1,
      imageUsage: vk_sys::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
      imageSharingMode: image_sharing_mode,
      queueFamilyIndexCount: queue_family_indices.len() as u32,
      pQueueFamilyIndices: queue_family_indices.as_ptr(),
      preTransform: surface_capabilities.currentTransform,
      compositeAlpha: vk_sys::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
      presentMode: swapchain_present_mode,
      clipped: vk_sys::TRUE,
      oldSwapchain: 0, /* null handle */
      surface: *surface,
    }
  };
  let swapchain = try!(device.create_swapchain(&swapchain_create_info_khr));

  Ok(LoadedSwapchain {
    surface_format: surface_format,
    surface_extent: surface_extent,
    swapchain_present_mode: swapchain_present_mode,
    swapchain_image_count: swapchain_image_count,
    swapchain: swapchain,
  })
}

pub fn make_image_view(
  device: &lite::LDevice,
  image: &vk_sys::Image,
  format: vk_sys::Format,
  aspect_mask: vk_sys::ImageAspectFlags,
) -> lite::RawResult<vk_sys::ImageView> {
  let image_create_info = vk_sys::ImageViewCreateInfo {
    sType: vk_sys::STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    image: *image,
    viewType: vk_sys::IMAGE_VIEW_TYPE_2D,
    format: format,
    components: vk_sys::ComponentMapping {
      r: vk_sys::COMPONENT_SWIZZLE_IDENTITY,
      g: vk_sys::COMPONENT_SWIZZLE_IDENTITY,
      b: vk_sys::COMPONENT_SWIZZLE_IDENTITY,
      a: vk_sys::COMPONENT_SWIZZLE_IDENTITY,
    },
    subresourceRange: vk_sys::ImageSubresourceRange {
      aspectMask: aspect_mask,
      baseMipLevel: 0,
      levelCount: 1,
      baseArrayLayer: 0,
      layerCount: 1,
    },
  };

  device.create_image_view(&image_create_info)
}

pub fn make_image_views(
  device: &lite::LDevice,
  swapchain_images: &Vec<vk_sys::Image>,
  swapchain: &LoadedSwapchain,
) -> lite::RawResult<Vec<vk_sys::ImageView>> {
  let mut image_views = Vec::with_capacity(swapchain_images.len());
  debug!("Vulkan creating image view for each image.");
  for swapchain_image in swapchain_images.iter() {
    image_views.push(try!(make_image_view(
      device,
      swapchain_image,
      swapchain.surface_format.format,
      vk_sys::IMAGE_ASPECT_COLOR_BIT
    )));
  }

  Ok(image_views)
}
