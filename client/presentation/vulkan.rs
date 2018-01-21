extern crate dylib;
extern crate vk_sys as vk;

use std::collections::HashMap;
use std::ffi::CString;
use std::ffi::CStr;
use std::path::PathBuf;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

pub trait WindowSystemPlugin {
  fn create_surface(&mut self, instance: vk::Instance, instance_ptrs: &vk::InstancePointers) -> vk::SurfaceKHR;
}

/** Contains static ptrs, entry ptrs, and dylib */
pub struct VkCtx {
  dylib: dylib::DynamicLibrary,
  pub entry_points: vk::EntryPoints,
  pub static_points: vk::Static,
  debug_report_callbacks: HashMap<vk::Instance, Vec<vk::DebugReportCallbackEXT>>,
  surfaces_map: HashMap<vk::Instance, Vec<vk::SurfaceKHR>>,
  instance_pointers_map: HashMap<vk::Instance, vk::InstancePointers>,
  device_pointers_map: HashMap<vk::Device, vk::DevicePointers>,
  queues_map: HashMap<vk::Device, vk::Queue>,
  swapchains_map: HashMap<vk::Device, vk::SwapchainKHR>,
  swapchain_params_map: HashMap<vk::SwapchainKHR, SwapchainParams>,
  swapchain_images_map: HashMap<vk::SwapchainKHR, Vec<vk::Image>>,
  swapchain_image_views_map: HashMap<vk::SwapchainKHR, Vec<vk::ImageView>>,
}

struct SwapchainParams {
  format: vk::Format,
  extent: vk::Extent2D,
}

impl Drop for VkCtx {
  fn drop(&mut self) {
    let mut debug_report_callbacks: HashMap<vk::Instance, Vec<vk::DebugReportCallbackEXT>> = HashMap::new();
    std::mem::swap(&mut self.debug_report_callbacks, &mut debug_report_callbacks);
    for (instance, debug_report_callback_ext_list) in debug_report_callbacks.into_iter() {
      for debug_report_callback_ext in debug_report_callback_ext_list.into_iter() {
        unsafe {
          self.instance_pointers_map.get(&instance).unwrap()
            .DestroyDebugReportCallbackEXT(instance, debug_report_callback_ext, ptr::null());
        }
      }
    }

    let mut swapchain_image_views_map: HashMap<vk::SwapchainKHR, Vec<vk::ImageView>> = HashMap::new();
    std::mem::swap(&mut self.swapchain_image_views_map, &mut swapchain_image_views_map);
    for (_swapchain_khr, image_views) in swapchain_image_views_map.into_iter() {
      // Find the device that this swapchain belongs to
      let logical_device =
        *self.swapchains_map.iter()
            .find(|&(logical_device, swapchain_khr)| *swapchain_khr == _swapchain_khr)
            .map(|(logical_device, _)| logical_device)
            .unwrap();
      unsafe {
        for image_view in image_views.iter() {
          self.device_ptrs(logical_device).DestroyImageView(logical_device, *image_view, ptr::null());
        }
      }
    }


    let mut swapchains_map: HashMap<vk::Device, vk::SwapchainKHR> = HashMap::new();
    std::mem::swap(&mut self.swapchains_map, &mut swapchains_map);
    for (logical_device, swapchain_khr) in swapchains_map.into_iter() {
      unsafe {
        self.device_ptrs(logical_device).DestroySwapchainKHR(logical_device, swapchain_khr, ptr::null());
      }
    }

    let mut device_pointers_map: HashMap<vk::Device, vk::DevicePointers> = HashMap::new();
    std::mem::swap(&mut self.device_pointers_map, &mut device_pointers_map);
    for (logical_device, device_pointers) in device_pointers_map.into_iter() {
      unsafe {
        device_pointers.DestroyDevice(logical_device, ptr::null());
      }
    }

    let mut surfaces_map: HashMap<vk::Instance, Vec<vk::SurfaceKHR>> = HashMap::new();
    std::mem::swap(&mut self.surfaces_map, &mut surfaces_map);
    for (instance, surfaces) in surfaces_map.into_iter() {
      for surface in surfaces.iter() {
        unsafe {
          self.instance_pointers_map.get(&instance).unwrap()
            .DestroySurfaceKHR(instance, *surface, ptr::null());
        }
      }
    }

    let mut instance_pointers_map: HashMap<vk::Instance, vk::InstancePointers> = HashMap::new();
    std::mem::swap(&mut self.instance_pointers_map, &mut instance_pointers_map);
    for (instance, instance_pointers) in instance_pointers_map.into_iter() {
      unsafe {
        instance_pointers.DestroyInstance(instance, ptr::null());
      }
    }
  }
}

impl VkCtx {
  fn from_dylib(dylib: dylib::DynamicLibrary) -> VkCtx {
    VkCtx {
      entry_points: vk::EntryPoints::load(|symbol_name| unsafe {
        dylib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void
      }),
      static_points: vk::Static::load(|symbol_name| unsafe {
        dylib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void
      }),
      debug_report_callbacks: HashMap::new(),
      instance_pointers_map: HashMap::new(),
      device_pointers_map: HashMap::new(),
      surfaces_map: HashMap::new(),
      queues_map: HashMap::new(),
      swapchains_map: HashMap::new(),
      swapchain_params_map: HashMap::new(),
      swapchain_images_map: HashMap::new(),
      swapchain_image_views_map: HashMap::new(),
      dylib: dylib
    }
  }

  fn load_instance_ptrs(&self, instance: vk::Instance) -> vk::InstancePointers {
    unsafe {
      vk::InstancePointers::load(|symbol_name| {
        self.static_points.GetInstanceProcAddr(instance, symbol_name.as_ptr()) as *const std::os::raw::c_void
      })
    }
  }

  /** visible for refactoring */
  pub fn select_extensions(&self, spec: ExtensionSpec) -> Vec<[i8; 256]> {
    let mut num_extensions = 0;
    let mut extensions = unsafe {std::mem::uninitialized() };
    unsafe {
      let result = self.entry_points.EnumerateInstanceExtensionProperties(
        ptr::null(), &mut num_extensions, ptr::null::<vk::ExtensionProperties>() as *mut _);

      if result != vk::SUCCESS {
        panic!("failed to enumerate instance extension properties instance with {}", vk_result_to_human(result as i32));
      }

      extensions = Vec::with_capacity(num_extensions as usize);

      let result = self.entry_points.EnumerateInstanceExtensionProperties(
        ptr::null(), &mut num_extensions, extensions.as_mut_ptr());

      if result != vk::SUCCESS {
        panic!("failed to enumerate instance extension properties instance with {}", vk_result_to_human(result as i32));
      }

      extensions.set_len(num_extensions as usize);

      let enabled_extensions = extensions.into_iter()
        .filter(|e| {
          let extension_as_str = CStr::from_ptr(e.extensionName.as_ptr()).to_str().unwrap();
          println!("Vulkan Extension found: {}", extension_as_str);
          let should_enable = spec.required.contains(&extension_as_str) || spec.wanted.contains(&extension_as_str);
          if should_enable {
            println!("       Extension enabled: {}", extension_as_str);
          }

          should_enable
        })
        .map(|e| e.extensionName)
        .collect::<Vec<_>>();

      let enabled_extensions_pretty = enabled_extensions.iter()
        .map(|e| CStr::from_ptr(e.as_ptr()).to_str().unwrap())
        .collect::<Vec<_>>();

      let mut missing_required_extensions = Vec::new();
      for required_ext in spec.required {
        if !enabled_extensions_pretty.contains(&required_ext) {
          missing_required_extensions.push(required_ext);
        }
      }

      if !missing_required_extensions.is_empty() {
        panic!("Some extensions were marked required but were not available on this platform: {:?}", missing_required_extensions);
      }

      enabled_extensions
    }
  }

  /** visible for refactoring */
  pub fn select_layers(&self, spec: LayerSpec) -> Vec<[i8; 256]> {
    let mut num_layers = 0;
    let mut layers = unsafe { std::mem::uninitialized() };
    unsafe {
      let result = self.entry_points.EnumerateInstanceLayerProperties(
        &mut num_layers, ptr::null::<vk::LayerProperties>() as *mut _);

      if result != vk::SUCCESS {
        panic!("failed to enumerate instance layer properties instance with {}", vk_result_to_human(result as i32));
      }

      layers = Vec::with_capacity(num_layers as usize);

      let result = self.entry_points.EnumerateInstanceLayerProperties(
        &mut num_layers, layers.as_mut_ptr());

      if result != vk::SUCCESS {
        panic!("failed to enumerate instance layer properties instance with {}", vk_result_to_human(result as i32));
      }

      layers.set_len(num_layers as usize);

      let enabled_layers = layers.into_iter()
        .filter(|l| {
          let layer_as_str = CStr::from_ptr(l.layerName.as_ptr()).to_str().unwrap();
          println!("Vulkan Layer found: {}", layer_as_str);
          let should_enable = spec.required.contains(&layer_as_str) || spec.wanted.contains(&layer_as_str);
          if should_enable {
            println!("       Layer enabled: {}", layer_as_str);
          }

          should_enable
        })
        .map(|l| l.layerName)
        .collect::<Vec<_>>();

      let enabled_layers_pretty = enabled_layers.iter()
        .map(|e| CStr::from_ptr(e.as_ptr()).to_str().unwrap())
        .collect::<Vec<_>>();

      let mut missing_required_layers = Vec::new();
      for required_layer in spec.required {
        if !enabled_layers_pretty.contains(&required_layer) {
          missing_required_layers.push(required_layer);
        }
      }

      if !missing_required_layers.is_empty() {
        panic!("Some layers were marked required but were not available on this platform: {:?}", missing_required_layers);
      }

      enabled_layers
    }
  }

  pub fn make_instance(&mut self, application_name: &'static str, engine_name: &'static str, extensions: &Vec<[i8; 256]>, layers: &Vec<[i8; 256]>) -> vk::Instance {
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
    let vk_instance_create_info = vk::InstanceCreateInfo {
      sType: vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
      pApplicationInfo: &vk_application_info as *const _,
      flags: 0,
      pNext: ptr::null(),
      enabledLayerCount: ppEnabledLayerNames.len() as u32,
      ppEnabledLayerNames: ppEnabledLayerNames.as_ptr(),
      enabledExtensionCount: ppEnabledExtensionNames.len() as u32,
      ppEnabledExtensionNames: ppEnabledExtensionNames.as_ptr(),
    };

    let mut instance = 0;
    unsafe {
      let result = self.entry_points.CreateInstance(
        &vk_instance_create_info, ptr::null(), &mut instance);

      if result != vk::SUCCESS {
        panic!("failed to create vulkan instance with {}", vk_result_to_human(result as i32));
      }
    }

    let instance_pointers = self.load_instance_ptrs(instance);
    self.instance_pointers_map.insert(instance, instance_pointers);

    instance
  }

  pub fn instance_ptrs(&self, instance: vk::Instance) -> &vk::InstancePointers {
    self.instance_pointers_map.get(&instance).unwrap()
  }

  pub fn configure_default_callback(&mut self, instance: vk::Instance, cb: vk::PFN_vkDebugReportCallbackEXT) {
    let debug_report_callback_ext = {
      let mut debug_report_callback_ext = 0;
      let debug_report_callback_create_info_ext = vk::DebugReportCallbackCreateInfoEXT {
        sType: vk::STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
        flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT,
        pNext: ptr::null(),
        pfnCallback: cb,
        pUserData: ptr::null_mut(),
      };
      unsafe {
        let result = self.instance_pointers_map.get(&instance).unwrap().CreateDebugReportCallbackEXT(
          instance,
          &debug_report_callback_create_info_ext as *const _,
          ptr::null(),
          &mut debug_report_callback_ext
        );
        if result != vk::SUCCESS {
          panic!("failed to create vulkan instance with {}", vk_result_to_human(result as i32));
        }
      };
      debug_report_callback_ext
    };

    if self.debug_report_callbacks.contains_key(&instance) {
      self.debug_report_callbacks.get_mut(&instance).unwrap().push(debug_report_callback_ext);
    } else {
      self.debug_report_callbacks.insert(instance, vec![debug_report_callback_ext]);
    }
  }

  pub fn find_capable_gfx_device(&self, instance: vk::Instance, surface: &mut vk::SurfaceKHR) -> Option<CapablePhysicalDevice> {
    let mut physical_device: vk::PhysicalDevice = 0;
    let mut swapchain_capabilities: vk::SurfaceCapabilitiesKHR = unsafe { std::mem::uninitialized() };
    let mut swapchain_formats: Vec<vk::SurfaceFormatKHR> = Vec::new();
    let mut swapchain_present_modes: Vec<vk::PresentModeKHR> = Vec::new();
    let mut gfx_supporting_queue_family_index: u32 = 0;
    unsafe {
      let mut num_physical_devices = 0u32;
      let result = self.instance_ptrs(instance).EnumeratePhysicalDevices(
        instance, &mut num_physical_devices, ptr::null_mut());

      if result != vk::SUCCESS {
        panic!("failed to enumerate physical devices with {}", vk_result_to_human(result as i32));
      }

      println!("found {} physical devices", num_physical_devices);

      let mut physical_devices: Vec<vk::PhysicalDevice> = Vec::with_capacity(num_physical_devices as usize);

      let result = self.instance_ptrs(instance).EnumeratePhysicalDevices(
        instance, &mut num_physical_devices, physical_devices.as_mut_ptr());

      if result != vk::SUCCESS {
        panic!("failed to enumerate instance extension properties instance with {}", vk_result_to_human(result as i32));
      }
      physical_devices.set_len(num_physical_devices as usize);

      for _physical_device in physical_devices.iter() {
        let mut physical_device_properties: vk::PhysicalDeviceProperties = std::mem::uninitialized();
        self.instance_ptrs(instance).GetPhysicalDeviceProperties(*_physical_device, &mut physical_device_properties);

        let mut physical_device_features: vk::PhysicalDeviceFeatures = std::mem::uninitialized();
        self.instance_ptrs(instance).GetPhysicalDeviceFeatures(*_physical_device, &mut physical_device_features);

        println!("Vulkan Physical Device found: {}", CStr::from_ptr(physical_device_properties.deviceName.as_ptr()).to_str().unwrap());

        let mut num_queue_family_properties = 0u32;
        self.instance_ptrs(instance).GetPhysicalDeviceQueueFamilyProperties(
          *_physical_device, &mut num_queue_family_properties, ptr::null_mut());

        println!("Vulkan Physical Queue Family Properties: {} found", num_queue_family_properties);

        let mut queue_family_properties_list: Vec<vk::QueueFamilyProperties> =
          Vec::with_capacity(num_queue_family_properties as usize);

        self.instance_ptrs(instance).GetPhysicalDeviceQueueFamilyProperties(
          *_physical_device, &mut num_queue_family_properties, queue_family_properties_list.as_mut_ptr());

        println!("populated queue family properties list");

        queue_family_properties_list.set_len(num_queue_family_properties as usize);

        let gfx_supporting_queue_family_index_opt = {
          let surface_is_supported_for_queue_idx_fn = |queue_family_idx| {
            let mut support_is_present = 0 /* false */;
            let result = self.instance_ptrs(instance)
              .GetPhysicalDeviceSurfaceSupportKHR(*_physical_device, queue_family_idx, *surface, &mut support_is_present);

            if result != vk::SUCCESS {
              panic!("failed to determine surface-device suitability with {}", vk_result_to_human(result as i32));
            }

            println!("support present in device: {}", support_is_present);

            // N.B.: Output is a vulkan-style 32 bit bool
            support_is_present > 0
          };

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
          let mut num_extensions = 0;
          self.instance_ptrs(instance).EnumerateDeviceExtensionProperties(*_physical_device, ptr::null(), &mut num_extensions, ptr::null_mut());
          let mut available_extensions: Vec<vk::ExtensionProperties> =
            Vec::with_capacity(num_extensions as usize);
          self.instance_ptrs(instance).EnumerateDeviceExtensionProperties(*_physical_device, ptr::null(), &mut num_extensions, available_extensions.as_mut_ptr());
          available_extensions.set_len(num_extensions as usize);

          let available_extension_names = available_extensions.iter().map(|e| CStr::from_ptr(e.extensionName.as_ptr()).to_str().unwrap()).collect::<Vec<_>>();
          for available_extension_name in available_extension_names.iter() {
            println!("Vulkan Device Extension found: {}", available_extension_name);
          }
          required_extension_names
            .iter()
            .map(|e| available_extension_names.contains(e))
            .all(|is_contained| is_contained)
        };

        let _swapchain_capabilities = {
          let mut swapchain_capabilities: vk::SurfaceCapabilitiesKHR = std::mem::uninitialized();
          let result = self.instance_ptrs(instance).GetPhysicalDeviceSurfaceCapabilitiesKHR(*_physical_device, *surface, &mut swapchain_capabilities);

          if result != vk::SUCCESS {
            panic!("failed to extract physical device surface capabilities with {}", vk_result_to_human(result as i32));
          }

          swapchain_capabilities
        };

        let _swapchain_formats = {
          let mut num_formats = 0;
          let result = self.instance_ptrs(instance).GetPhysicalDeviceSurfaceFormatsKHR(
            *_physical_device, *surface, &mut num_formats, ptr::null_mut());

          if result != vk::SUCCESS {
            panic!("failed to enumerate surface formats for device with {}", vk_result_to_human(result as i32));
          }

          let mut swapchain_formats = Vec::with_capacity(num_formats as usize);

          let result = self.instance_ptrs(instance).GetPhysicalDeviceSurfaceFormatsKHR(
            *_physical_device, *surface, &mut num_formats, swapchain_formats.as_mut_ptr());

          if result != vk::SUCCESS {
            panic!("failed to list surface formats for device with {}", vk_result_to_human(result as i32));
          }
          swapchain_formats.set_len(num_formats as usize);
          swapchain_formats
        };

        let _swapchain_present_modes = {
          let mut num_present_modes = 0;
          let result = self.instance_ptrs(instance).GetPhysicalDeviceSurfacePresentModesKHR(
            *_physical_device, *surface, &mut num_present_modes, ptr::null_mut());

          if result != vk::SUCCESS {
            panic!("failed to enumerate surface present modes for device with {}", vk_result_to_human(result as i32));
          }

          let mut swapchain_present_modes = Vec::with_capacity(num_present_modes as usize);

          let result = self.instance_ptrs(instance).GetPhysicalDeviceSurfacePresentModesKHR(
            *_physical_device, *surface, &mut num_present_modes, swapchain_present_modes.as_mut_ptr());

          if result != vk::SUCCESS {
            panic!("failed to list surface present modes for device with {}", vk_result_to_human(result as i32));
          }
          swapchain_present_modes.set_len(num_present_modes as usize);
          swapchain_present_modes
        };

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
        instance: instance,
        device: physical_device,
        gfx_supporting_queue_family_index: gfx_supporting_queue_family_index,
        swapchain_capabilities: swapchain_capabilities,
        swapchain_formats: swapchain_formats,
        swapchain_present_modes: swapchain_present_modes,
      })
    }
  }

  pub fn init_logical_device(&mut self, capable_physical_device: &CapablePhysicalDevice, enabled_layers: &Vec<[i8; 256]>) -> vk::Device {
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
    println!("Constructing logical device");

    let mut logical_device = 0;
    unsafe {
      let result = self.instance_ptrs(capable_physical_device.instance).CreateDevice(
        capable_physical_device.device, &device_create_info, ptr::null(), &mut logical_device);

      if result != vk::SUCCESS {
        panic!("failed to create logical device with {}", vk_result_to_human(result as i32));
      }
    }

    println!("Loading device pointers");

    // Set up the ptrs required for device manipulation
    let device_pointers = unsafe {
      vk::DevicePointers::load(|symbol_name| {
        self.instance_ptrs(capable_physical_device.instance).GetDeviceProcAddr(logical_device, symbol_name.as_ptr()) as *const std::os::raw::c_void
      })
    };

    self.device_pointers_map.insert(logical_device, device_pointers);

    let mut queue: vk::Queue = unsafe {std::mem::uninitialized() } ;
    unsafe {
      self.device_ptrs(logical_device).GetDeviceQueue(
        logical_device,
        capable_physical_device.gfx_supporting_queue_family_index,
        0,
        &mut queue
      );
    }
    self.queues_map.insert(logical_device, queue);

    logical_device
  }

  pub fn device_ptrs(&self, logical_device: vk::Device) -> &vk::DevicePointers {
    self.device_pointers_map.get(&logical_device).unwrap()
  }

  pub fn init_swap_chain(&mut self, instance: vk::Instance, logical_device: vk::Device, capable_physical_device: &CapablePhysicalDevice, surface: &mut vk::SurfaceKHR) -> vk::SwapchainKHR {
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

    let swapchain = {
      let mut swapchain = 0u64;
      unsafe {
        let result = self.device_ptrs(logical_device)
          .CreateSwapchainKHR(logical_device, &swapchain_create_info_khr, ptr::null(), &mut swapchain);

        if result != vk::SUCCESS {
          panic!("failed to create swapchain with {}", vk_result_to_human(result as i32));
        }
        swapchain
      }
    };

    let swapchain_images = {
      let mut num_images = 0;

      unsafe {
        let result = self.device_ptrs(logical_device)
          .GetSwapchainImagesKHR(logical_device, swapchain, &mut num_images, ptr::null_mut());

        if result != vk::SUCCESS {
          panic!("failed to enumerate swapchain images with {}", vk_result_to_human(result as i32));
        }

        let mut swapchain_images = Vec::with_capacity(num_images as usize);

        let result = self.device_ptrs(logical_device).GetSwapchainImagesKHR(
          logical_device, swapchain, &mut num_images, swapchain_images.as_mut_ptr());

        if result != vk::SUCCESS {
          panic!("failed to fetch swapchain images with {}", vk_result_to_human(result as i32));
        }
        swapchain_images.set_len(num_images as usize);
        swapchain_images
      }
    };

    self.swapchains_map.insert(logical_device, swapchain);
    self.swapchain_params_map.insert(swapchain, SwapchainParams {
      format: swapchain_best_format.format,
      extent: swapchain_extent,
    });
    self.swapchain_images_map.insert(swapchain, swapchain_images);

    swapchain
  }

  pub fn init_image_views(&mut self, logical_device: vk::Device, swapchain: vk::SwapchainKHR) {
    let swapchain_images = self.swapchain_images_map.get(&swapchain).unwrap();
    let swapchain_params = self.swapchain_params_map.get(&swapchain).unwrap();
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

      unsafe {
        let mut image_view = std::mem::uninitialized();
        let result = self.device_ptrs(logical_device).CreateImageView(logical_device, &image_view_create_info, ptr::null(), &mut image_view);
        if result != vk::SUCCESS {
          panic!("failed to fetch swapchain image view with {}", vk_result_to_human(result as i32));
        }

        image_views.push(image_view);
      }
    }

    self.swapchain_image_views_map.insert(swapchain, image_views);
  }
}

/** Contains instance, and instance ptrs */
pub struct VkInstanceCtx<'a> {
  pub instance_pointers: &'a vk::InstancePointers,
}

/** Contains device, and device ptrs */
pub struct VkDeviceCtx<'a> {
  pub device_pointers: &'a vk::DevicePointers,
}

pub struct ExtensionSpec {
  pub wanted: Vec<&'static str>,
  pub required: Vec<&'static str>
}

pub struct LayerSpec {
  pub wanted: Vec<&'static str>,
  pub required: Vec<&'static str>
}

pub struct CapablePhysicalDevice {
  instance: vk::Instance,
  device: vk::PhysicalDevice,
  gfx_supporting_queue_family_index: u32,
  swapchain_capabilities: vk::SurfaceCapabilitiesKHR,
  swapchain_formats: Vec<vk::SurfaceFormatKHR>,
  swapchain_present_modes: Vec<vk::PresentModeKHR>,
}

pub fn vulkan<W: WindowSystemPlugin>(window_system_plugin: &mut W) -> VkCtx {
  let dylib_path = PathBuf::from("libvulkan.so.1");
  let dylib = dylib::DynamicLibrary::open(Some(dylib_path.as_path())).unwrap();

  let mut vk_ctx = VkCtx::from_dylib(dylib);

  let extension_spec = ExtensionSpec {
    wanted: vec! [
      "VK_EXT_acquire_xlib_display",
      "VK_EXT_display_surface_counter",
      "VK_KHR_display",
      "VK_KHR_get_physical_device_properties2",
      "VK_KHR_get_surface_capabilities2",
      "VK_KHR_surface",
      "VK_KHR_xcb_surface",
      "VK_KHR_xlib_surface",
      "VK_KHX_device_group_creation",
    ],
    required: vec! [
      "VK_EXT_debug_report",
    ],
  };
  let enabled_extensions = vk_ctx.select_extensions(extension_spec);

  let layer_spec = LayerSpec {
    wanted: Vec::new(),
    required: vec! [
      "VK_LAYER_LUNARG_standard_validation",
    ],
  };
  let enabled_layers = vk_ctx.select_layers(layer_spec);

  println!("setting up instance");
  let instance = vk_ctx.make_instance(
    "dummy_application_name", "dummy_engine_name", &enabled_extensions, &enabled_layers);

  // Configure debug callback
  println!("setting up debug callback");
  vk_ctx.configure_default_callback(instance, vk_debug_report_callback_ext);

  let mut surface_khr = window_system_plugin.create_surface(instance, vk_ctx.instance_ptrs(instance));
  if vk_ctx.surfaces_map.contains_key(&instance) {
    vk_ctx.surfaces_map.get_mut(&instance).unwrap().push(surface_khr);
  } else {
    vk_ctx.surfaces_map.insert(instance, vec![surface_khr]);
  }

  let capable_physical_device = vk_ctx.find_capable_gfx_device(instance, &mut surface_khr)
    .expect("there was no suitable physical device!");

  let logical_device = vk_ctx.init_logical_device(&capable_physical_device, &enabled_layers);

  let swapchain_khr = vk_ctx.init_swap_chain(instance, logical_device, &capable_physical_device, &mut surface_khr);

  vk_ctx.init_image_views(logical_device, swapchain_khr);

  vk_ctx
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
