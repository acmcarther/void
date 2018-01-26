extern crate vk_sys as vk;
extern crate vk_lite as vkl;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

/** Essentially "try!", but for Vulkan Error Types. Prints things nicely. */
macro_rules! do_or_die {
  ($res:expr) => {
    match $res {
      Err(vkl::RawReturnCode(code, ctx_string)) => panic!("Low level Vulkan error {} with context: {}", vk_result_to_human(code), ctx_string),
      v => v.unwrap(),
    }
  };
}


fn vk_result_to_human(code: i32) -> String {
  match code {
    0 => "VK_SUCCESS".to_owned(),
    1 => "VK_NOT_READY".to_owned(),
    2 => "VK_TIMEOUT".to_owned(),
    3 => "VK_EVENT_SET".to_owned(),
    4 => "VK_EVENT_RESET".to_owned(),
    5 => "VK_INCOMPLETE".to_owned(),
    -1 => "VK_ERROR_OUT_OF_HOST_MEMORY".to_owned(),
    -2 => "VK_ERROR_OUT_OF_DEVICE_MEMORY".to_owned(),
    -3 => "VK_ERROR_INITIALIZATION_FAILED".to_owned(),
    -4 => "VK_ERROR_DEVICE_LOST".to_owned(),
    -5 => "VK_ERROR_MEMORY_MAP_FAILED".to_owned(),
    -6 => "VK_ERROR_LAYER_NOT_PRESENT".to_owned(),
    -7 => "VK_ERROR_EXTENSION_NOT_PRESENT".to_owned(),
    -8 => "VK_ERROR_FEATURE_NOT_PRESENT".to_owned(),
    -9 => "VK_ERROR_INCOMPATIBLE_DRIVER".to_owned(),
    -10 => "VK_ERROR_TOO_MANY_OBJECTS".to_owned(),
    -11 => "VK_ERROR_FORMAT_NOT_SUPPORTED".to_owned(),
    -12 => "VK_ERROR_FRAGMENTED_POOL".to_owned(),
    _ => format!("UNKNOWN VK CODE {}", code)
  }
}

pub fn draw_demo_frame(t: &VulkanTriangle) {
  let mut image_index = 0;
  unsafe {
    let image_index = do_or_die!(vkl::util::loady("next image", &|a| t.device.ptrs().AcquireNextImageKHR(
      t.device.logical_device,
      t.swapchain_khr,
      u64::max_value(),
      t.image_available_semaphore,
      0 /* vk_null_handle */,
      a)));

    let wait_semaphores = [t.image_available_semaphore];
    let wait_stages = [vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];
    let signal_semaphores = [t.render_finished_semaphore];
    let submit_info = vk::SubmitInfo {
      sType: vk::STRUCTURE_TYPE_SUBMIT_INFO,
      pNext: ptr::null(),
      waitSemaphoreCount: 1,
      pWaitSemaphores: wait_semaphores.as_ptr(),
      pWaitDstStageMask: wait_stages.as_ptr(),
      commandBufferCount: 1,
      pCommandBuffers: t.command_buffers.get(image_index as usize).unwrap(),
      signalSemaphoreCount: 1,
      pSignalSemaphores: signal_semaphores.as_ptr(),
    };

    let queue = t.device.get_device_queue(t.capable_physical_device.gfx_supporting_queue_family_index, 0 /* queue_index */);

    do_or_die!(vkl::util::dooy("queue submit", &|| {t.device.ptrs()
      .QueueSubmit(queue, 1, &submit_info, 0 /* vk_null_handle */)}));

    let swapchains = [t.swapchain_khr];
    let present_info_khr = vk::PresentInfoKHR {
      sType: vk::STRUCTURE_TYPE_PRESENT_INFO_KHR,
      pNext: ptr::null(),
      waitSemaphoreCount: 1,
      pWaitSemaphores: signal_semaphores.as_ptr(),
      swapchainCount: 1,
      pSwapchains: swapchains.as_ptr(),
      pImageIndices: &image_index,
      pResults: ptr::null_mut(),
    };

    do_or_die!(vkl::util::dooy("queue present", &|| t.device.ptrs().QueuePresentKHR(
      queue,
      &present_info_khr)));
  }
}

impl Drop for VulkanTriangle {
  fn drop(&mut self) {
    do_or_die!(self.device.device_wait_idle());
    self.device.destroy_semaphore(self.render_finished_semaphore);
    self.device.destroy_semaphore(self.image_available_semaphore);
    self.device.destroy_command_pool(self.command_pool);
    for framebuffer in self.framebuffers.drain(..) {
      self.device.destroy_framebuffer(framebuffer);
    }
    self.device.destroy_pipeline(self.graphics_pipeline);
    self.device.destroy_pipeline_layout(self.pipeline_layout);
    self.device.destroy_shader_module(self.vert_shader_module);
    self.device.destroy_shader_module(self.frag_shader_module);
    self.device.destroy_render_pass(self.render_pass);
    for image_view in self.image_views.drain(..) {
      self.device.destroy_image_view(image_view);
    }

    self.device.destroy_swapchain(self.swapchain_khr);
    self.instance.destroy_debug_callback(self.debug_report_callback)

    // swapchain_params: does not need explicit destruction
    // capable_physical_device: does not need explicit destruction
    // device: Destroyed on drop
    // instance: Destroyed on drop
  }
}

pub struct SwapchainParams {
  format: vk::Format,
  extent: vk::Extent2D,
}


pub struct CapablePhysicalDevice {
  device: vk::PhysicalDevice,
  gfx_supporting_queue_family_index: u32,
  swapchain_capabilities: vk::SurfaceCapabilitiesKHR,
  swapchain_formats: Vec<vk::SurfaceFormatKHR>,
  swapchain_present_modes: Vec<vk::PresentModeKHR>,
}

// TODO(acmcarther): This is a higher level function, inlined here during refactoring
// The intent is that the application will use this api directly
fn make_instance(application_name: &'static str,
                 engine_name: &'static str,
                 extensions: &Vec<[i8; 256]>,
                 layers: &Vec<[i8; 256]>,
                 create_fn: &Fn(&vk::InstanceCreateInfo) -> vkl::RawResult<vkl::LInstance>) -> vkl::LInstance {
  let pApplicationName = CString::new(application_name).unwrap();
  let pEngineName = CString::new(engine_name).unwrap();

  // Set up application
  let vk_application_info = vk::ApplicationInfo {
    sType: vk::STRUCTURE_TYPE_APPLICATION_INFO,
    pNext: ptr::null(),
    pApplicationName: pApplicationName.as_ptr(),
    applicationVersion: 1,
    pEngineName: pEngineName.as_ptr(),
    engineVersion: 1,
    apiVersion: 0 /* 1? */,
  };

  let ppEnabledLayerNames = layers.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
  let ppEnabledExtensionNames = extensions.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
  do_or_die!(create_fn(&vk::InstanceCreateInfo {
    sType: vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
    pApplicationInfo: &vk_application_info as *const _,
    flags: 0,
    pNext: ptr::null(),
    enabledLayerCount: ppEnabledLayerNames.len() as u32,
    ppEnabledLayerNames: ppEnabledLayerNames.as_ptr(),
    enabledExtensionCount: ppEnabledExtensionNames.len() as u32,
    ppEnabledExtensionNames: ppEnabledExtensionNames.as_ptr(),
  }))
}

// This can probably stay baked into the lite lib
fn make_debug_report_callback(cb: vk::PFN_vkDebugReportCallbackEXT, create_fn: &Fn(&vk::DebugReportCallbackCreateInfoEXT) -> vkl::RawResult<vk::DebugReportCallbackEXT>) -> vk::DebugReportCallbackEXT {
  do_or_die!(create_fn(&vk::DebugReportCallbackCreateInfoEXT {
    sType: vk::STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
    flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT,
    pNext: ptr::null(),
    pfnCallback: cb,
    pUserData: ptr::null_mut(),
  }))
}

// TODO(acmcarther): Some of this should be refactored out. Callers will want to customize how they
// select devices
fn find_capable_gfx_device(v_i: &vkl::LInstance, surface: &vk::SurfaceKHR) -> Option<CapablePhysicalDevice> {
  let mut physical_device: vk::PhysicalDevice = 0;
  let mut swapchain_capabilities: vk::SurfaceCapabilitiesKHR = unsafe { std::mem::uninitialized() };
  let mut swapchain_formats: Vec<vk::SurfaceFormatKHR> = Vec::new();
  let mut swapchain_present_modes: Vec<vk::PresentModeKHR> = Vec::new();
  let mut gfx_supporting_queue_family_index: u32 = 0;
  let physical_devices = do_or_die!(v_i.list_physical_devices());
  unsafe {
    for _physical_device in physical_devices.iter() {
      let mut physical_device_properties: vk::PhysicalDeviceProperties = v_i.get_physical_device_properties(*_physical_device);

      let mut physical_device_features: vk::PhysicalDeviceFeatures = v_i.get_physical_device_features(*_physical_device);

      println!("Vulkan Physical Device found: {}", CStr::from_ptr(physical_device_properties.deviceName.as_ptr()).to_str().unwrap());

      let gfx_supporting_queue_family_index_opt = {
        let surface_is_supported_for_queue_idx_fn = |queue_family_idx| {
          let support_is_present = do_or_die!(v_i.get_physical_device_surface_support(*_physical_device, queue_family_idx, surface));

          // N.B.: Output is a vulkan-style 32 bit bool
          support_is_present > 0
        };

        let queue_family_properties_list = v_i.list_queue_family_properties(*_physical_device);
        // TODO(acmcarther): Support independent GRAPHICS and PRESENT queues.
        // The current implementation expects to find a single queue for both, but it isn't
        // unreasonable to have these live in separate queues.
        queue_family_properties_list.iter()
          .enumerate()
          .filter(|&(idx, props)|
                    props.queueCount > 0
                    && (props.queueFlags & vk::QUEUE_GRAPHICS_BIT) > 0
                    && surface_is_supported_for_queue_idx_fn(idx as u32))
          .map(|(idx, props)| idx)
          .next() /* get first */
      };

      // TODO(acmcarther): This is duplicated in logical device instantiation
      let required_extension_names = vec![
        "VK_KHR_swapchain",
      ];

      let required_extensions_supported = {
        let available_extensions = do_or_die!(v_i.list_device_extension_properties(*_physical_device));
        let available_extension_names = available_extensions.iter().map(|e| CStr::from_ptr(e.extensionName.as_ptr()).to_str().unwrap()).collect::<Vec<_>>();
        for available_extension_name in available_extension_names.iter() {
          println!("Vulkan Device Extension found: {}", available_extension_name);
        }
        required_extension_names
          .iter()
          .map(|e| available_extension_names.contains(e))
          .all(|is_contained| is_contained)
      };

      let _swapchain_capabilities = do_or_die!(v_i.get_physical_device_surface_capabilities(*_physical_device, surface));

      let _swapchain_formats = do_or_die!(v_i.list_physical_device_surface_formats(*_physical_device, surface));

      let _swapchain_present_modes = do_or_die!(v_i.list_physical_device_present_modes(*_physical_device, surface));

      let required_swapchain_support_present = {
        !_swapchain_formats.is_empty() && !_swapchain_present_modes.is_empty()
      };

      if physical_device == 0
        && gfx_supporting_queue_family_index_opt.is_some()
        && required_extensions_supported
        && required_swapchain_support_present {
        println!("going with that physical_device, as it supports a gfx and present queue");
        physical_device = *_physical_device;
        swapchain_capabilities = _swapchain_capabilities;
        swapchain_formats = _swapchain_formats;
        swapchain_present_modes = _swapchain_present_modes;
        gfx_supporting_queue_family_index = gfx_supporting_queue_family_index_opt.unwrap() as u32;
      }
    }
  }

  if physical_device == 0 {
    None
  } else {
    Some(CapablePhysicalDevice {
      device: physical_device,
      gfx_supporting_queue_family_index: gfx_supporting_queue_family_index,
      swapchain_capabilities: swapchain_capabilities,
      swapchain_formats: swapchain_formats,
      swapchain_present_modes: swapchain_present_modes,
    })
  }
}

// TODO(acmcarther): This needs to be customizable
fn make_logical_device(capable_physical_device: &CapablePhysicalDevice, enabled_layers: &Vec<[i8; 256]>, create_fn: &Fn(usize, &vk::DeviceCreateInfo) -> vkl::RawResult<vkl::LDevice>) -> vkl::LDevice {
  let queue_priorities = [1.0f32];
  let device_queue_create_info = vk::DeviceQueueCreateInfo {
    sType: vk::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    queueFamilyIndex: capable_physical_device.gfx_supporting_queue_family_index,
    queueCount: 1,
    pQueuePriorities: queue_priorities.as_ptr(),
  };

  let physical_device_features = vk::PhysicalDeviceFeatures {
    robustBufferAccess: vk::FALSE,
    fullDrawIndexUint32: vk::FALSE,
    imageCubeArray: vk::FALSE,
    independentBlend: vk::FALSE,
    geometryShader: vk::FALSE,
    tessellationShader: vk::FALSE,
    sampleRateShading: vk::FALSE,
    dualSrcBlend: vk::FALSE,
    logicOp: vk::FALSE,
    multiDrawIndirect: vk::FALSE,
    drawIndirectFirstInstance: vk::FALSE,
    depthClamp: vk::FALSE,
    depthBiasClamp: vk::FALSE,
    fillModeNonSolid: vk::FALSE,
    depthBounds: vk::FALSE,
    wideLines: vk::FALSE,
    largePoints: vk::FALSE,
    alphaToOne: vk::FALSE,
    multiViewport: vk::FALSE,
    samplerAnisotropy: vk::FALSE,
    textureCompressionETC2: vk::FALSE,
    textureCompressionASTC_LDR: vk::FALSE,
    textureCompressionBC: vk::FALSE,
    occlusionQueryPrecise: vk::FALSE,
    pipelineStatisticsQuery: vk::FALSE,
    vertexPipelineStoresAndAtomics: vk::FALSE,
    fragmentStoresAndAtomics: vk::FALSE,
    shaderTessellationAndGeometryPointSize: vk::FALSE,
    shaderImageGatherExtended: vk::FALSE,
    shaderStorageImageExtendedFormats: vk::FALSE,
    shaderStorageImageMultisample: vk::FALSE,
    shaderStorageImageReadWithoutFormat: vk::FALSE,
    shaderStorageImageWriteWithoutFormat: vk::FALSE,
    shaderUniformBufferArrayDynamicIndexing: vk::FALSE,
    shaderSampledImageArrayDynamicIndexing: vk::FALSE,
    shaderStorageBufferArrayDynamicIndexing: vk::FALSE,
    shaderStorageImageArrayDynamicIndexing: vk::FALSE,
    shaderClipDistance: vk::FALSE,
    shaderCullDistance: vk::FALSE,
    shaderf3264: vk::FALSE,
    shaderInt64: vk::FALSE,
    shaderInt16: vk::FALSE,
    shaderResourceResidency: vk::FALSE,
    shaderResourceMinLod: vk::FALSE,
    sparseBinding: vk::FALSE,
    sparseResidencyBuffer: vk::FALSE,
    sparseResidencyImage2D: vk::FALSE,
    sparseResidencyImage3D: vk::FALSE,
    sparseResidency2Samples: vk::FALSE,
    sparseResidency4Samples: vk::FALSE,
    sparseResidency8Samples: vk::FALSE,
    sparseResidency16Samples: vk::FALSE,
    sparseResidencyAliased: vk::FALSE,
    variableMultisampleRate: vk::FALSE,
    inheritedQueries: vk::FALSE,
  };
  let ppEnabledLayerNames = enabled_layers.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
  let enabled_extension_names = vec![CString::new("VK_KHR_swapchain").unwrap()];
  let ppEnabledExtensionNames = enabled_extension_names.iter().map(|i| i.as_c_str().as_ptr()).collect::<Vec<_>>();
  let device_create_info = vk::DeviceCreateInfo {
    sType: vk::STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    queueCreateInfoCount: 1,
    pQueueCreateInfos: &device_queue_create_info as *const _,
    enabledLayerCount: ppEnabledLayerNames.len() as u32,
    ppEnabledLayerNames: ppEnabledLayerNames.as_ptr(),
    enabledExtensionCount: ppEnabledExtensionNames.len() as u32,
    ppEnabledExtensionNames: ppEnabledExtensionNames.as_ptr(),
    pEnabledFeatures: &physical_device_features as *const _,
  };

  do_or_die!(create_fn(capable_physical_device.device, &device_create_info))
}

pub fn make_swap_chain(capable_physical_device: &CapablePhysicalDevice,
                           surface: &vk::SurfaceKHR,
                           create_fn: &Fn(&vk::SwapchainCreateInfoKHR) -> vkl::RawResult<vk::SwapchainKHR>) -> (SwapchainParams, vk::SwapchainKHR) {
  let swapchain_best_format = {
    let swapchain_formats = &capable_physical_device.swapchain_formats;

    // Device has no preference at all
    if swapchain_formats.len() == 1 && swapchain_formats.get(0).unwrap().format == vk::FORMAT_UNDEFINED {
      vk::SurfaceFormatKHR {
        format: vk::FORMAT_B8G8R8A8_UNORM,
        colorSpace: vk::COLOR_SPACE_SRGB_NONLINEAR_KHR,
      }
    } else {
      // Try to find our favorite format
      let ideal_format_opt = swapchain_formats
        .iter()
        .find(|f| f.format == vk::FORMAT_B8G8R8A8_UNORM && f.colorSpace == vk::COLOR_SPACE_SRGB_NONLINEAR_KHR);
      if ideal_format_opt.is_some() {
        vk::SurfaceFormatKHR {
          format: vk::FORMAT_B8G8R8A8_UNORM,
          colorSpace: vk::COLOR_SPACE_SRGB_NONLINEAR_KHR,
        }
      } else {
        println!("Using a sub-optimal swapchain format");
        // Just use the first available
        let first_available_format = swapchain_formats.get(0).unwrap();
        vk::SurfaceFormatKHR {
          format: first_available_format.format,
          colorSpace: first_available_format.colorSpace,
        }
      }
    }
  };
  println!("Vulkan configuring swapchain format");

  // Choose MAILBOX, IMMEDIATE, FIFO
  let swapchain_best_present_mode /* : vk::"Present Mode", u32 */  = {
    let swapchain_present_modes = &capable_physical_device.swapchain_present_modes;
    let mut mode = vk::PRESENT_MODE_FIFO_KHR /* default */;

    if swapchain_present_modes.contains(&vk::PRESENT_MODE_MAILBOX_KHR) {
      mode = vk::PRESENT_MODE_MAILBOX_KHR
    } else if swapchain_present_modes.contains(&vk::PRESENT_MODE_IMMEDIATE_KHR) {
      mode = vk::PRESENT_MODE_IMMEDIATE_KHR
    }

    mode
  };
  println!("Vulkan configuring swapchain present mode");

  let swapchain_extent: vk::Extent2D = {
    let swapchain_capabilities = &capable_physical_device.swapchain_capabilities;
    let must_use_provided_values = swapchain_capabilities.currentExtent.width != u32::max_value();
    let DEFAULT_SWAP_WIDTH: u32 = 800 /* px */;
    let DEFAULT_SWAP_HEIGHT: u32 = 600 /* px */;
    if must_use_provided_values {
      vk::Extent2D {
        width: swapchain_capabilities.currentExtent.width,
        height: swapchain_capabilities.currentExtent.height,
      }
    } else {
      vk::Extent2D {
        width: swapchain_capabilities.minImageExtent.width
          .max(swapchain_capabilities.maxImageExtent.width
               .min(DEFAULT_SWAP_WIDTH)),
        height: swapchain_capabilities.minImageExtent.height
          .max(swapchain_capabilities.maxImageExtent.height
               .min(DEFAULT_SWAP_HEIGHT)),
      }
    }
  };
  println!("Vulkan configuring swapchain extents");

  let swapchain_image_count: u32 = {
    let swapchain_capabilities = &capable_physical_device.swapchain_capabilities;
    let desired_image_count = swapchain_capabilities.minImageCount + 1;

    // Max images may be bounded, use that if its lower than our desired image count
    if swapchain_capabilities.maxImageCount != 0
      && swapchain_capabilities.maxImageCount < desired_image_count {
      swapchain_capabilities.maxImageCount
    } else {
      desired_image_count
    }
  };

  println!("Vulkan preparing swapchain");

  // TODO(acmcarther): Support multiple queues (ex: separate gfx queue from present queue)
  // This will involve changing imageSharingMode to vk::SHARING_MODE_CONCURRENT, setting
  // queueFamilyIndexCount, and populating the pQueueFamilyIndices
  let swapchain_create_info_khr = {
    let swapchain_capabilities = &capable_physical_device.swapchain_capabilities;
    vk::SwapchainCreateInfoKHR {
      sType: vk::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
      pNext: ptr::null(),
      flags: 0,
      minImageCount: swapchain_image_count,
      imageFormat: swapchain_best_format.format,
      imageColorSpace: swapchain_best_format.colorSpace,
      imageExtent: vk::Extent2D {
        width: swapchain_extent.width,
        height: swapchain_extent.height,
      },
      imageArrayLayers: 1,
      imageUsage: vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
      imageSharingMode: vk::SHARING_MODE_EXCLUSIVE,
      queueFamilyIndexCount: 0 /* omitted under SHARING_MODE_EXCLUSIVE */,
      pQueueFamilyIndices: ptr::null() /* omitted under SHARING_MODE_EXCLUSIVE */,
      preTransform: swapchain_capabilities.currentTransform,
      compositeAlpha: vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
      presentMode: swapchain_best_present_mode,
      clipped: vk::TRUE,
      oldSwapchain: 0 /* null handle */,
      surface: *surface,
    }
  };
  println!("Vulkan creating swap chain");

  let swapchain = do_or_die!(create_fn(&swapchain_create_info_khr));

  (SwapchainParams {
    format: swapchain_best_format.format,
    extent: swapchain_extent,
  }, swapchain)
}

fn make_image_views(swapchain_images: &Vec<vk::Image>,
                        swapchain_params: &SwapchainParams,
                        create_fn: &Fn(&vk::ImageViewCreateInfo) -> vkl::RawResult<vk::ImageView>) -> Vec<vk::ImageView> {
  let mut image_views = Vec::with_capacity(swapchain_images.len());
  println!("Vulkan creating image view for each image.");
  for swapchain_image in swapchain_images.iter() {
    let image_view_create_info = vk::ImageViewCreateInfo {
      sType: vk::STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
      pNext: ptr::null(),
      flags: 0,
      image: *swapchain_image,
      format: swapchain_params.format,
      viewType: vk::IMAGE_VIEW_TYPE_2D,
      components: vk::ComponentMapping {
        r: vk::COMPONENT_SWIZZLE_IDENTITY,
        g: vk::COMPONENT_SWIZZLE_IDENTITY,
        b: vk::COMPONENT_SWIZZLE_IDENTITY,
        a: vk::COMPONENT_SWIZZLE_IDENTITY,
      },
      // N.B.: Under a sterographic 3d application situation, create a swapchain containing
      // multiple layers -- one per view
      subresourceRange: vk::ImageSubresourceRange {
        aspectMask: vk::IMAGE_ASPECT_COLOR_BIT,
        baseMipLevel: 0,
        levelCount: 1,
        baseArrayLayer: 0,
        layerCount: 1,
      },
    };

    image_views.push(do_or_die!(create_fn(&image_view_create_info)));
  }

  image_views
}

pub fn make_render_pass(swapchain_params: &SwapchainParams, create_fn: &Fn(&vk::RenderPassCreateInfo) -> vkl::RawResult<vk::RenderPass>) -> vk::RenderPass {
  let color_attachment_description = vk::AttachmentDescription {
    flags: 0,
    format: swapchain_params.format,
    samples: vk::SAMPLE_COUNT_1_BIT,
    loadOp: vk::ATTACHMENT_LOAD_OP_CLEAR,
    storeOp: vk::ATTACHMENT_STORE_OP_STORE,
    stencilLoadOp: vk::ATTACHMENT_LOAD_OP_DONT_CARE,
    stencilStoreOp: vk::ATTACHMENT_STORE_OP_DONT_CARE,
    initialLayout: vk::IMAGE_LAYOUT_UNDEFINED,
    finalLayout: vk::IMAGE_LAYOUT_PRESENT_SRC_KHR,
  };

  let color_attachment_reference = vk::AttachmentReference {
    attachment: 0,
    layout: vk::IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
  };

  let subpass_description = vk::SubpassDescription {
    flags: 0,
    pipelineBindPoint: vk::PIPELINE_BIND_POINT_GRAPHICS,
    inputAttachmentCount: 0,
    pInputAttachments: ptr::null(),
    colorAttachmentCount: 1,
    pColorAttachments: &color_attachment_reference,
    pResolveAttachments: ptr::null(),
    pDepthStencilAttachment: ptr::null(),
    preserveAttachmentCount: 0,
    pPreserveAttachments: ptr::null(),
  };

  let dependency = vk::SubpassDependency {
    srcSubpass: vk::SUBPASS_EXTERNAL,
    dstSubpass: 0,
    srcStageMask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
    dstStageMask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
    srcAccessMask: 0,
    dstAccessMask: vk::ACCESS_COLOR_ATTACHMENT_READ_BIT | vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
    dependencyFlags: 0,
  };

  println!("Vulkan creating render pass");

  let render_pass_create_info = vk::RenderPassCreateInfo {
    sType: vk::STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    attachmentCount: 1,
    pAttachments: &color_attachment_description,
    subpassCount: 1,
    pSubpasses: &subpass_description,
    dependencyCount: 1,
    pDependencies: &dependency,
  };

  do_or_die!(create_fn(&render_pass_create_info))
}

pub fn make_shader_module(shader_bytes: &[u8], create_fn: &Fn(&vk::ShaderModuleCreateInfo) -> vkl::RawResult<vk::ShaderModule>) -> vk::ShaderModule {
  let shader_module_create_info = vk::ShaderModuleCreateInfo {
    sType: vk::STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    codeSize: shader_bytes.len(),
    pCode: shader_bytes.as_ptr() as *const u32,
  };
  do_or_die!(create_fn(&shader_module_create_info))
}

pub fn make_pipeline_layout(create_fn: &Fn(&vk::PipelineLayoutCreateInfo) -> vkl::RawResult<vk::PipelineLayout>) -> vk::PipelineLayout {
  let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    setLayoutCount: 0,
    pSetLayouts: ptr::null(),
    pushConstantRangeCount: 0,
    pPushConstantRanges: ptr::null(),
  };

  do_or_die!(create_fn(&pipeline_layout_create_info))
}

pub fn make_graphics_pipeline(vert_shader_module: &vk::ShaderModule,
                              frag_shader_module: &vk::ShaderModule,
                              render_pass: &vk::RenderPass,
                              swapchain_params: &SwapchainParams,
                              pipeline_layout: &vk::PipelineLayout,
                              create_fn: &Fn(&[vk::GraphicsPipelineCreateInfo]) -> vkl::RawResult<Vec<vk::Pipeline>>) -> vk::Pipeline {

  let common_shader_pipeline_name = CString::new("main").unwrap();
  let pName = common_shader_pipeline_name.as_c_str().as_ptr();
  let vert_pipeline_shader_stage_create_info = vk::PipelineShaderStageCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    stage: vk::SHADER_STAGE_VERTEX_BIT,
    module: *vert_shader_module,
    pName: pName,
    pSpecializationInfo: ptr::null(),
  };

  let frag_pipeline_shader_stage_create_info = vk::PipelineShaderStageCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    stage: vk::SHADER_STAGE_FRAGMENT_BIT,
    module: *frag_shader_module,
    pName: pName,
    pSpecializationInfo: ptr::null(),
  };

  let pipeline_vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    vertexBindingDescriptionCount: 0,
    pVertexBindingDescriptions: ptr::null(),
    vertexAttributeDescriptionCount: 0,
    pVertexAttributeDescriptions: ptr::null(),
  };

  let pipeline_input_assembly_state_create_info = vk::PipelineInputAssemblyStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    topology: vk::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
    primitiveRestartEnable: vk::FALSE,
  };

  let viewport = vk::Viewport {
     x: 0.0f32,
     y: 0.0f32,
     width: swapchain_params.extent.width as f32,
     height: swapchain_params.extent.height as f32,
     minDepth: 0.0f32,
     maxDepth: 1.0f32,
  };

  // Defines how the image in the viewport is truncated
  let scissor = vk::Rect2D {
    offset: vk::Offset2D {
      x: 0,
      y: 0
    },
    extent: vk::Extent2D {
      width: swapchain_params.extent.width,
      height: swapchain_params.extent.height,
    },
  };

  let pipeline_viewport_state_create_info = vk::PipelineViewportStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    viewportCount: 1,
    pViewports: &viewport,
    scissorCount: 1,
    pScissors: &scissor,
  };

  let pipeline_rasterization_state_create_info = vk::PipelineRasterizationStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    depthClampEnable: vk::FALSE,
    rasterizerDiscardEnable: vk::FALSE,
    polygonMode: vk::POLYGON_MODE_FILL,
    cullMode: vk::CULL_MODE_BACK_BIT,
    frontFace: vk::FRONT_FACE_CLOCKWISE,
    depthBiasEnable: vk::FALSE,
    depthBiasConstantFactor: 0.0f32,
    depthBiasClamp: 0.0f32,
    depthBiasSlopeFactor: 0.0f32,
    lineWidth: 1.0f32,
  };

  // TODO(acmcarther): Examine these options
  // N.B: Enabling this requires a GPU extension.
  let pipeline_multisample_state_create_info = vk::PipelineMultisampleStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    rasterizationSamples: vk::SAMPLE_COUNT_1_BIT,
    sampleShadingEnable: vk::FALSE,
    minSampleShading: 1.0f32,
    pSampleMask: ptr::null(),
    alphaToCoverageEnable: vk::FALSE,
    alphaToOneEnable: vk::FALSE,
  };

  // TODO(acmcarther): Depth and Stencil Testing
  // ...


  // TODO(acmcarther): Examine these options
  let pipeline_color_blend_attachment_state = vk::PipelineColorBlendAttachmentState {
    blendEnable: vk::FALSE,
    colorWriteMask: vk::COLOR_COMPONENT_R_BIT | vk::COLOR_COMPONENT_G_BIT | vk::COLOR_COMPONENT_B_BIT | vk::COLOR_COMPONENT_A_BIT,
    srcColorBlendFactor: vk::BLEND_FACTOR_ONE,
    dstColorBlendFactor: vk::BLEND_FACTOR_ZERO,
    colorBlendOp: vk::BLEND_OP_ADD,
    srcAlphaBlendFactor: vk::BLEND_FACTOR_ONE,
    dstAlphaBlendFactor: vk::BLEND_FACTOR_ZERO,
    alphaBlendOp: vk::BLEND_OP_ADD,
  };

  let pipeline_color_blend_state_create_info = vk::PipelineColorBlendStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    logicOpEnable: vk::FALSE,
    logicOp: vk::LOGIC_OP_COPY,
    attachmentCount: 1,
    pAttachments: &pipeline_color_blend_attachment_state,
    blendConstants: [0f32, 0f32, 0f32, 0f32],
  };

  let dynamic_states = [
    vk::DYNAMIC_STATE_VIEWPORT,
    vk::DYNAMIC_STATE_LINE_WIDTH,
  ];

  let pipeline_dynamic_state_create_info = vk::PipelineDynamicStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    dynamicStateCount: 2,
    pDynamicStates: dynamic_states.as_ptr(),
  };

  let pipeline_shader_stage_infos = vec![
    vert_pipeline_shader_stage_create_info,
    frag_pipeline_shader_stage_create_info
  ];

  let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo {
    sType: vk::STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    stageCount: 2,
    pStages: pipeline_shader_stage_infos.as_ptr(),
    pVertexInputState: &pipeline_vertex_input_state_create_info,
    pInputAssemblyState: &pipeline_input_assembly_state_create_info,
    pTessellationState: ptr::null(),
    pViewportState: &pipeline_viewport_state_create_info,
    pRasterizationState: &pipeline_rasterization_state_create_info,
    pMultisampleState: &pipeline_multisample_state_create_info,
    pDepthStencilState: ptr::null(),
    pColorBlendState: &pipeline_color_blend_state_create_info,
    pDynamicState: ptr::null(),
    layout: *pipeline_layout,
    renderPass: *render_pass,
    subpass: 0,
    basePipelineHandle: 0 /* vk_null_handle */,
    basePipelineIndex: -1,
  };

  do_or_die!(create_fn(&vec![graphics_pipeline_create_info])).remove(0)
}

pub fn make_framebuffers(image_views: &Vec<vk::ImageView>, swapchain_params: &SwapchainParams, render_pass: &vk::RenderPass, create_fn: &Fn(&vk::FramebufferCreateInfo) -> vkl::RawResult<vk::Framebuffer>) -> Vec<vk::Framebuffer> {
  let mut framebuffers = Vec::with_capacity(image_views.len());
  for swapchain_image_view in image_views.iter() {
    let framebuffer_create_info = vk::FramebufferCreateInfo {
      sType: vk::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
      pNext: ptr::null(),
      flags: 0,
      renderPass: *render_pass,
      attachmentCount: 1,
      pAttachments: swapchain_image_view,
      width: swapchain_params.extent.width,
      height: swapchain_params.extent.height,
      layers: 1,
    };

    framebuffers.push(do_or_die!(create_fn(&framebuffer_create_info)));
  }

  framebuffers
}

pub fn make_command_pool(capable_physical_device: &CapablePhysicalDevice, create_fn: &Fn(&vk::CommandPoolCreateInfo) -> vkl::RawResult<vk::CommandPool>) -> vk::CommandPool {
  let command_pool_create_info = vk::CommandPoolCreateInfo {
    sType: vk::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    queueFamilyIndex: capable_physical_device.gfx_supporting_queue_family_index,
  };

  do_or_die!(create_fn(&command_pool_create_info))
}

pub fn make_command_buffers(swapchain_params: &SwapchainParams,
                            framebuffers: &Vec<vk::Framebuffer>,
                            command_pool: &vk::CommandPool,
                            allocate_fn: &Fn(&vk::CommandBufferAllocateInfo, &[vk::Framebuffer]) -> vkl::RawResult<Vec<vk::CommandBuffer>>) -> Vec<vk::CommandBuffer> {
  let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
    sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    pNext: ptr::null(),
    commandPool: *command_pool,
    level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
    commandBufferCount: framebuffers.len() as u32,
  };

  do_or_die!(allocate_fn(&command_buffer_allocate_info, &framebuffers))
}

pub fn record_command_buffers(v_d: &vkl::LDevice, swapchain_params: &SwapchainParams, framebuffers: &Vec<vk::Framebuffer>, render_pass: &vk::RenderPass, graphics_pipeline: &vk::Pipeline, command_buffers: &Vec<vk::CommandBuffer>) {
  for (idx, command_buffer) in command_buffers.iter().enumerate() {
    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
      sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
      pNext: ptr::null(),
      flags: vk::COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
      pInheritanceInfo: ptr::null(),
    };

    do_or_die!(vkl::util::dooy("start command buffer", &|| unsafe {v_d.ptrs().BeginCommandBuffer(*command_buffer, &command_buffer_begin_info)}));

    {
      let clear_color = vk::ClearValue {
        color: vk::ClearColorValue {
          float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32]
        }
      };
      let render_pass_begin_info = vk::RenderPassBeginInfo {
        sType: vk::STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        pNext: ptr::null(),
        renderPass: *render_pass,
        framebuffer: *framebuffers.get(idx).unwrap(),
        renderArea: vk::Rect2D {
          offset: vk::Offset2D {
            x: 0,
            y: 0,
          },
          extent: vk::Extent2D {
            width: swapchain_params.extent.width,
            height: swapchain_params.extent.height,
          },
        },
        clearValueCount: 1,
        pClearValues: &clear_color,
      };

      unsafe {
        v_d.ptrs().CmdBeginRenderPass(*command_buffer, &render_pass_begin_info, vk::SUBPASS_CONTENTS_INLINE);
        v_d.ptrs().CmdBindPipeline(*command_buffer, vk::PIPELINE_BIND_POINT_GRAPHICS, *graphics_pipeline);
        v_d.ptrs().CmdDraw(*command_buffer, 3, 1, 0, 0);
        v_d.ptrs().CmdEndRenderPass(*command_buffer);
      }
    }

    do_or_die!(vkl::util::dooy("end command buffer", &|| unsafe {v_d.ptrs().EndCommandBuffer(*command_buffer)}))
  }
}

pub fn make_semaphore(create_fn: &Fn(&vk::SemaphoreCreateInfo) -> vkl::RawResult<vk::Semaphore>) -> vk::Semaphore {
  let semaphore_create_info = vk::SemaphoreCreateInfo {
    sType: vk::STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
  };
  do_or_die!(create_fn(&semaphore_create_info))
}

pub fn vulkan<'a, W: vkl::WindowSystemPlugin>(vulkan: &'a vkl::Vulkan, window_system_plugin: &mut W, vert_shader_bytes: &[u8], frag_shader_bytes: &[u8]) -> VulkanTriangle {
  let extension_spec = vkl::FeatureSpec { wanted: vec! [
      "VK_EXT_acquire_xlib_display",
      //"VK_EXT_display_surface_counter",
      "VK_KHR_display",
      "VK_KHR_get_physical_device_properties2",
      "VK_KHR_get_surface_capabilities2",
      "VK_KHR_surface",
      //"VK_KHR_xcb_surface",
      "VK_KHR_xlib_surface",
      "VK_KHX_device_group_creation",
    ],
    required: vec! [
      "VK_EXT_debug_report",
    ],
  };
  let enabled_extensions = do_or_die!(vulkan.select_extensions(extension_spec));

  let layer_spec = vkl::FeatureSpec {
    wanted: vec![
      "VK_LAYER_LUNARG_core_validation",
      "VK_LAYER_LUNARG_parameter_validation",
    ],
    required: vec! [
      "VK_LAYER_LUNARG_standard_validation",
    ],
  };
  let enabled_layers = do_or_die!(vulkan.select_layers(layer_spec));

  println!("setting up instance");
  let v_i = make_instance("dummy_application_name",
                                 "dummy_engine_name",
                                 &enabled_extensions,
                                 &enabled_layers,
                                 &|a| vulkan.create_instance(a));

  // Configure debug callback
  let debug_report_callback =
    make_debug_report_callback(vk_debug_report_callback_ext, &|a| v_i.create_debug_callback(a));

  let v_surface = do_or_die!(window_system_plugin.create_surface(&v_i));

  let capable_physical_device = find_capable_gfx_device(&v_i, &v_surface)
    .expect("there was no suitable physical device!");

  let v_d = make_logical_device(&capable_physical_device, &enabled_layers, &|a, b| v_i.create_logical_device(a, b));

  //let queue = v_d.get_device_queue(capable_physical_device.queue_family_idx, 0 /* queueIndex */);

  let (swapchain_params, swapchain_khr) = make_swap_chain(&capable_physical_device, &v_surface, &|a| v_d.create_swapchain(a));

  let swapchain_images = do_or_die!(unsafe {v_d.get_swapchain_images(&swapchain_khr) });

  let image_views = make_image_views(&swapchain_images, &swapchain_params, &|a| v_d.create_image_view(a));

  let render_pass = make_render_pass(&swapchain_params, &|a| v_d.create_render_pass(a));

  let vert_shader_module = make_shader_module(vert_shader_bytes, &|a| v_d.create_shader_module(a));

  let frag_shader_module = make_shader_module(frag_shader_bytes, &|a| v_d.create_shader_module(a));

  let pipeline_layout = make_pipeline_layout(&|a| v_d.create_pipeline_layout(a));

  let graphics_pipeline = make_graphics_pipeline(&vert_shader_module, &frag_shader_module, &render_pass, &swapchain_params, &pipeline_layout, &|a| v_d.create_graphics_pipelines(a));

  let framebuffers = make_framebuffers(&image_views, &swapchain_params, &render_pass, &|a| v_d.create_framebuffer(a));
  let command_pool = make_command_pool(&capable_physical_device, &|a| v_d.create_command_pool(a));

  let command_buffers = make_command_buffers(&swapchain_params, &framebuffers, &command_pool, &|a, b| v_d.allocate_command_buffers(a, b));

  record_command_buffers(&v_d, &swapchain_params, &framebuffers, &render_pass, &graphics_pipeline, &command_buffers);

  let image_available_semaphore = make_semaphore(&|a| v_d.create_semaphore(a));
  let render_finished_semaphore = make_semaphore(&|a| v_d.create_semaphore(a));

  VulkanTriangle {
    instance: v_i,
    device: v_d,
    debug_report_callback: debug_report_callback,
    capable_physical_device: capable_physical_device,
    swapchain_params: swapchain_params,
    swapchain_khr: swapchain_khr,
    image_views: image_views,
    render_pass: render_pass,
    vert_shader_module: vert_shader_module,
    frag_shader_module: frag_shader_module,
    pipeline_layout: pipeline_layout,
    graphics_pipeline: graphics_pipeline,
    framebuffers: framebuffers,
    command_pool: command_pool,
    command_buffers: command_buffers,
    image_available_semaphore: image_available_semaphore,
    render_finished_semaphore: render_finished_semaphore,
  }
}

pub struct VulkanTriangle {
  instance: vkl::LInstance,
  device: vkl::LDevice,
  debug_report_callback: vk::DebugReportCallbackEXT,
  capable_physical_device: CapablePhysicalDevice,
  swapchain_params: SwapchainParams,
  swapchain_khr: vk::SwapchainKHR,
  image_views: Vec<vk::ImageView>,
  render_pass: vk::RenderPass,
  vert_shader_module: vk::ShaderModule,
  frag_shader_module: vk::ShaderModule,
  pipeline_layout: vk::PipelineLayout,
  graphics_pipeline: vk::Pipeline,
  framebuffers: Vec<vk::Framebuffer>,
  command_pool: vk::CommandPool,
  command_buffers: Vec<vk::CommandBuffer>,
  image_available_semaphore: vk::Semaphore,
  render_finished_semaphore: vk::Semaphore,
}

extern "system" fn vk_debug_report_callback_ext(
    flags: vk::DebugReportFlagsEXT,
    object_type: vk::DebugReportObjectTypeEXT,
    obj: u64,
    location: usize,
    code: i32,
    layer_prefix: *const c_char,
    msg: *const c_char,
    user_data: *mut c_void) -> vk::Bool32 {
  unsafe {
    println!("validation layer: {}", CStr::from_ptr(msg).to_str().unwrap());
  }
  vk::FALSE
}
