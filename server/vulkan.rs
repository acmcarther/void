extern crate dylib;
extern crate vk_sys as vk;

use std::time::Duration;
use std::ffi::CString;
use std::ffi::CStr;
use std::path::Path;
use std::path::PathBuf;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

pub fn vulkan() {
  let lib_path = PathBuf::from("libvulkan.so.1");
  let lib = dylib::DynamicLibrary::open(Some(lib_path.as_path())).unwrap();

  // Set up bootstrap vulkan ptrs
  let entry_points = unsafe {
    vk::EntryPoints::load(|symbol_name| lib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void)
  };

  // Fetch supported vulkan extension information
  let mut num_extensions = 20u32;
  let mut extensions = Vec::with_capacity(num_extensions as usize);
  unsafe {
    let result = entry_points.EnumerateInstanceExtensionProperties(
      ptr::null(), &mut num_extensions, ptr::null::<vk::ExtensionProperties>() as *mut _);

    if result != vk::SUCCESS {
      panic!("failed to enumerate instance extension properties instance with {}", vk_result_to_human(result as i32));
    }

    let result = entry_points.EnumerateInstanceExtensionProperties(
      ptr::null(), &mut num_extensions, extensions.as_mut_ptr());

    if result != vk::SUCCESS {
      panic!("failed to enumerate instance extension properties instance with {}", vk_result_to_human(result as i32));
    }

    extensions.set_len(num_extensions as usize);
  }

  // Print extensions for debug
  for extension in extensions.iter() {
    unsafe {
      println!("extension {} available", CStr::from_ptr(extension.extensionName.as_ptr()).to_str().unwrap())
    }
  }

  // Retain debug related extensions
  let enabled_extensions = unsafe {
    extensions.into_iter()
      .filter(|e| {
        let layer_as_str = CStr::from_ptr(e.extensionName.as_ptr()).to_str().unwrap();
        match layer_as_str {
          "VK_EXT_acquire_xlib_display" => false,
          "VK_EXT_debug_report" => true,
          "VK_EXT_direct_mode_display" => false,
          "VK_EXT_display_surface_counter" => true,
          "VK_KHR_display" => true,
          "VK_KHR_external_fence_capabilities" => false,
          "VK_KHR_external_memory_capabilities" => false,
          "VK_KHR_external_semaphore_capabilities" => false,
          "VK_KHR_get_physical_device_properties2" => true,
          "VK_KHR_get_surface_capabilities2" => true,
          "VK_KHR_surface" => true,
          "VK_KHR_xcb_surface" => true,
          "VK_KHR_xlib_surface" => true,
          "VK_KHX_device_group_creation" => true,
          _ => false,
        }
      })
      .map(|e| e.extensionName)
      .collect::<Vec<_>>()
  };
  unsafe {
    for extension in enabled_extensions.iter() {
      println!("extension enabled: {}", CStr::from_ptr(extension.as_ptr()).to_str().unwrap())
    }
  }


  // Fetch supported vulkan layer information
  let mut num_layers = 20u32;
  let mut layers: Vec<vk::LayerProperties> = Vec::with_capacity(num_layers as usize);
  unsafe {
    let result = entry_points.EnumerateInstanceLayerProperties(
      &mut num_layers, ptr::null_mut());

    if result != vk::SUCCESS {
      panic!("failed to enumerate instance extension properties instance with {}", vk_result_to_human(result as i32));
    }

    let result = entry_points.EnumerateInstanceLayerProperties(
      &mut num_layers, layers.as_mut_ptr());

    if result != vk::SUCCESS {
      panic!("failed to enumerate instance extension properties instance with {}", vk_result_to_human(result as i32));
    }
    layers.set_len(num_layers as usize);
  }

  // Print layers for debug
  for layer in layers.iter() {
    unsafe {
      println!("layer {} available", CStr::from_ptr(layer.layerName.as_ptr()).to_str().unwrap())
    }
  }


  // Retain layers that are validation related
  let enabled_layers = unsafe {
    layers.into_iter()
      .filter(|l| CStr::from_ptr(l.layerName.as_ptr()).to_str().unwrap().contains("standard_validation"))
      .map(|l| l.layerName)
      .collect::<Vec<_>>()
  };
  unsafe {
    for layer in enabled_layers.iter() {
      println!("layer enabled: {}", CStr::from_ptr(layer.as_ptr()).to_str().unwrap())
    }
  }

  println!("setting up instance");
  let pApplicationName = CString::new("sdl2_toy").unwrap();
  let pEngineName = CString::new("No Engine").unwrap();

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

  let ppEnabledLayerNames = enabled_layers.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
  let ppEnabledExtensionNames = enabled_extensions.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
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
    let result = entry_points.CreateInstance(
      &vk_instance_create_info, ptr::null(), &mut instance);

    if result != vk::SUCCESS {
      panic!("failed to create vulkan instance with {}", vk_result_to_human(result as i32));
    }
  }

  println!("Fetching static ptrs");

  // Set up _more_ bootstrap vulkan ptrs
  let static_points = unsafe {
    vk::Static::load(|symbol_name| lib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void)
  };


  println!("fetching instance pointers");

  // Set up the ptrs required for instance manipulation
  let instance_pointers = unsafe {
    vk::InstancePointers::load(|symbol_name| {
      static_points.GetInstanceProcAddr(instance, symbol_name.as_ptr()) as *const std::os::raw::c_void
    })
  };

  println!("setting up debug callback");

  // Configure debug callback
  let mut debug_report_callback_ext = {
    let mut debug_report_callback_ext = 0;
    let debug_report_callback_create_info_ext = vk::DebugReportCallbackCreateInfoEXT {
      sType: vk::STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT,
      flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT,
      pNext: ptr::null(),
      pfnCallback: vk_debug_report_callback_ext,
      pUserData: ptr::null_mut(),
    };
    unsafe {
      let result = instance_pointers.CreateDebugReportCallbackEXT(
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

  let mut physical_device: vk::PhysicalDevice = 0;
  let mut gfx_supporting_queue_family_index: u32 = 0;
  unsafe {
    let mut num_devices = 0u32;
    let result = instance_pointers.EnumeratePhysicalDevices(
      instance, &mut num_devices, ptr::null_mut());

    if result != vk::SUCCESS {
      panic!("failed to enumerate devices with {}", vk_result_to_human(result as i32));
    }

    println!("found {} devices", num_devices);

    let mut devices: Vec<vk::PhysicalDevice> = Vec::with_capacity(num_devices as usize);

    let result = instance_pointers.EnumeratePhysicalDevices(
      instance, &mut num_devices, devices.as_mut_ptr());

    if result != vk::SUCCESS {
      panic!("failed to enumerate instance extension properties instance with {}", vk_result_to_human(result as i32));
    }
    devices.set_len(num_devices as usize);

    for device in devices.iter() {
      let mut physical_device_properties: vk::PhysicalDeviceProperties = std::mem::uninitialized();
      instance_pointers.GetPhysicalDeviceProperties(*device, &mut physical_device_properties);

      let mut physical_device_features: vk::PhysicalDeviceFeatures = std::mem::uninitialized();
      instance_pointers.GetPhysicalDeviceFeatures(*device, &mut physical_device_features);

      println!("got a device {}", CStr::from_ptr(physical_device_properties.deviceName.as_ptr()).to_str().unwrap());

      let mut num_queue_family_properties = 0u32;
      let mut queue_family_properties_list: Vec<vk::QueueFamilyProperties> = std::mem::uninitialized();
      instance_pointers.GetPhysicalDeviceQueueFamilyProperties(
        *device, &mut num_queue_family_properties, ptr::null_mut());

      println!("found {} queue family properties", num_queue_family_properties);

      let mut queue_family_properties_list: Vec<vk::QueueFamilyProperties> =
        Vec::with_capacity(num_queue_family_properties as usize);

      instance_pointers.GetPhysicalDeviceQueueFamilyProperties(
        *device, &mut num_queue_family_properties, queue_family_properties_list.as_mut_ptr());

      println!("populated queue family properties list");

      queue_family_properties_list.set_len(num_queue_family_properties as usize);

      let gfx_supporting_queue_family_index_opt =
        queue_family_properties_list.iter()
          .position(|props| props.queueCount > 0 && (props.queueFlags & vk::QUEUE_GRAPHICS_BIT > 0));

      if gfx_supporting_queue_family_index_opt.is_some() && physical_device == 0 {
        println!("going with that device, as it supports a gfx queue");
        physical_device = *device;
        gfx_supporting_queue_family_index = gfx_supporting_queue_family_index_opt.unwrap() as u32;
      }
    }
  }

  if physical_device == 0 {
    panic!("there was no suitable device available!");
  }

  let queue_priorities = [1.0f32];
  let device_queue_create_info = vk::DeviceQueueCreateInfo {
    sType: vk::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    queueFamilyIndex: gfx_supporting_queue_family_index,
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

  let device_create_info = vk::DeviceCreateInfo {
    sType: vk::STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    queueCreateInfoCount: 1,
    pQueueCreateInfos: &device_queue_create_info as *const _,
    enabledLayerCount: ppEnabledLayerNames.len() as u32,
    ppEnabledLayerNames: ppEnabledLayerNames.as_ptr(),
    enabledExtensionCount: 0,
    ppEnabledExtensionNames: ptr::null(),
    pEnabledFeatures: &physical_device_features as *const _,
  };
  println!("Constructing logical device");

  let mut logical_device = 0;
  unsafe {
    let result = instance_pointers.CreateDevice(
      physical_device, &device_create_info, ptr::null(), &mut logical_device);

    if result != vk::SUCCESS {
      panic!("failed to create logical device with {}", vk_result_to_human(result as i32));
    }
  }

  println!("Loading device pointers");

  // Set up the ptrs required for device manipulation
  let device_pointers = unsafe {
    vk::DevicePointers::load(|symbol_name| {
      instance_pointers.GetDeviceProcAddr(logical_device, symbol_name.as_ptr()) as *const std::os::raw::c_void
    })
  };

  println!("Retrieving gfx capable queue");
  // Get the gfx-capable device queue
  let mut queue: vk::Queue = unsafe {std::mem::uninitialized() } ;
  unsafe {
    device_pointers.GetDeviceQueue(
      logical_device,
      gfx_supporting_queue_family_index,
      0,
      &mut queue
    );
  }

  // Destroy all created stuff
  unsafe {
    device_pointers.DestroyDevice(logical_device, ptr::null());
    instance_pointers.DestroyDebugReportCallbackEXT(instance, debug_report_callback_ext, ptr::null());
    instance_pointers.DestroyInstance(instance, ptr::null());
  }
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
