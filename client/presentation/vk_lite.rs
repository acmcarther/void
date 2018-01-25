extern crate dylib;
extern crate vk_sys as vk;

use std::collections::HashMap;
use std::ffi::CString;
use std::ffi::CStr;
use std::path::PathBuf;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

/** Performs a no-result Vulkan action, yielding a Rust-idiomatic result type. */
fn dooy(msg: &str, f: &Fn() -> u32) -> VkRawResult<()> {
  let result = f();

  if result != vk::SUCCESS {
    return Err(VkRawReturnCode(result as i32, format!("while doing {}", msg)));
  }

  Ok(())
}

/** Loads a Vulkan value, yielding a Rust-idiomatic result type. */
fn loady<T>(msg: &str, f: &Fn(*mut T) -> u32) -> VkRawResult<T> {
  unsafe {
    let mut item = std::mem::uninitialized();
    let result = f(&mut item);

    if result != vk::SUCCESS {
      return Err(VkRawReturnCode(result as i32, format!("while getting {}", msg)));
    }

    Ok(item)
  }
}

/** Fetches a list of vulkan values, yielding a Rust-idiomatic result type. */
fn loady_listy<T>(msg: &str, f: &Fn(&mut u32, *mut T) -> u32) -> VkRawResult<Vec<T>> {
  unsafe {
    let mut num_items = 0;
    let result = unsafe { f(&mut num_items, ptr::null::<T>() as *mut _) };
    if result != vk::SUCCESS {
      return Err(VkRawReturnCode(result as i32, format!("while enumerating {}", msg)))
    }

    let mut items = Vec::with_capacity(num_items as usize);

    let result = unsafe { f(&mut num_items, items.as_mut_ptr()) };
    if result != vk::SUCCESS {
      return Err(VkRawReturnCode(result as i32, format!("while fetching list of {}", msg)))
    }

    unsafe { items.set_len(num_items as usize); }

    Ok(items)
  }
}

pub trait WindowSystemPlugin {
  /** Creates a VkSurface object using foreign window system internals */
  fn create_surface(&mut self, instance: &'a VkInstance) -> VkRawResult<VkSurface<'a>> {
}

/** The wrapped vulkan return code, plus call context */
#[derive(Debug)]
pub struct VkRawReturnCode(i32, String);

pub type VkRawResult<T> = Result<T, VkRawReturnCode>;

pub struct Vk {
  dylib: dylib::DynamicLibrary,
  entry_points: vk::EntryPoints,
  static_points: vk::Static,
}

impl Vk {
  pub fn new(dylib_path: &'static str) -> Vk {
    let dylib_path = PathBuf::from(dylib_path);
    let dylib = dylib::DynamicLibrary::open(Some(dylib_path.as_path())).unwrap();

    let entry_points = vk::EntryPoints::load(|symbol_name| unsafe {
      dylib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void
    }),
    let static_points = vk::Static::load(|symbol_name| unsafe {
      dylib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void
    }),

    Vk {
      dylib: dylib,
      entry_points: entry_points,
      static_points: static_points,
    }
  }

  pub fn select_extensions(&self, spec: FeatureSpec) -> Vec<[i8; 256]> {
    let extensions = do_or_die!(self.list_instance_extensions());
    unsafe {
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

  pub fn select_layers(&self, spec: FeatureSpec) -> Vec<[i8; 256]> {
    let layers = do_or_die!(self.list_instance_layers());
    unsafe {
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


  pub fn create_instance(&'a self, instance_create_info: &vk::InstanceCreateInfo) -> VkRawResult<VkInstance> {
    loady("instance", &|a| unsafe {
      self.entry_points.CreateInstance(instance_create_info, ptr::null(), a)
    }).map(|instance| {
      let instance_ptrs = unsafe {
        vk::InstancePointers::load(|symbol_name| {
          self.static_points.GetInstanceProcAddr(instance, symbol_name.as_ptr()) as *const std::os::raw::c_void
        })
      };
      VkInstance {
        _p: PhantomData<&'a>,
        instance: instance,
        instance_ptrs: instance_ptrs
      }
    })
  }

  pub fn list_instance_extensions(&self) -> VkRawResult<Vec<vk::ExtensionProperties>>  {
    loady_listy("instance extensions", &|a, b| unsafe {
      self.entry_points.EnumerateInstanceExtensionProperties(ptr::null() /* pLayerName */, a, b)
    })
  }

  pub fn list_instance_layers(&self) -> VkRawResult<Vec<vk::LayerProperties>> {
    loady_listy("instance layers", &|a, b| unsafe {
      self.entry_points.EnumerateInstanceLayerProperties(a, b)
    })
  }
}

pub struct VkInstance<'a> {
  _p: PhantomData<'a>,
  instance: vk::Instance,
  instance_ptrs: vk::InstancePointers,
}

impl Drop for VkInstance {
  fn drop(&mut self) {
    self.instance_ptrs.DestroyInstance(self.instance, ptr::null());
  }
}

impl VkInstance<'a> {
  // TODO(acmcarther): Create a "dies on drop" object for debug report callback.
  pub fn create_debug_callback(&self, debug_report_callback_create_info_ext: &vk::DebugReportCallbackCreateInfoEXT) -> VkRawResult<VkDebugReportCallback> {
    loady("register debug callback", &|a| unsafe {
      self.instance_ptrs
        .CreateDebugReportCallbackEXT(self.instance, debug_report_callback_create_info_ext, ptr::null(), a)
    }).map(|debug_report_callback_ext| VkDebugReportCallback {
      logical_device: || self.instance_ptrs.DestroyDebugReportCallbackEXT(self.instance, debug_report_callback_ext, ptr::null()),
      debug_report_callback_ext: debug_report_callback_ext
    })
  }

  pub fn list_physical_devices(&self) -> VkRawResult<Vec<usize>, VkRawReturnCode> {
    loady_listy("physical devices", &|a, b| unsafe {
      self.instance_ptrs.EnumeratePhysicalDevices(self.instance, a, b)
    })
  }

  pub fn list_queue_family_properties(&self, physical_device: usize) -> Vec<vk::QueueFamilyProperties> {
    // N.B.: Not generic because FFI method does not return a VkRawResult
    unsafe {
      let mut num_queue_family_properties = 0u32;
      self.instance_ptrs(self.instance).GetPhysicalDeviceQueueFamilyProperties(
        physical_device, &mut num_queue_family_properties, ptr::null_mut());

      println!("Vulkan Physical Queue Family Properties: {} found", num_queue_family_properties);

      let mut queue_family_properties_list: Vec<vk::QueueFamilyProperties> =
        Vec::with_capacity(num_queue_family_properties as usize);

      self.instance_ptrs.GetPhysicalDeviceQueueFamilyProperties(
        physical_device, &mut num_queue_family_properties, queue_family_properties_list.as_mut_ptr());

      println!("populated queue family properties list");

      queue_family_properties_list.set_len(num_queue_family_properties as usize);

      queue_family_properties_list
    }
  }

  pub fn list_device_extension_properties(&self, physical_device: usize) -> VkRawResult<Vec<vk::ExtensionProperties>> {
    loady_listy("physical device extension properties", &|a, b| unsafe {
      self.instance_ptrs.EnumerateDeviceExtensionProperties(physical_device, ptr::null() /* pLayerName */, a, b)
    })
  }

  pub fn get_physical_device_surface_capabilities(&self, physical_device: usize, surface: vk::SurfaceKHR) -> VkRawResult<vk::SurfaceCapabilitiesKHR {
    loady("physical device surface properties", &|a| unsafe {
      self.instance_ptrs.GetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, *surface, a)
    })
  }

  pub fn list_physical_device_surface_formats(&self, physical_device: usize, surface: vk::SurfaceKHR) -> VkRawResult<Vec<vk::SurfaceFormatKHR> {
    loady_listy("physical device surface formats", &|a, b| unsafe {
      self.instance_ptrs.GetPhysicalDeviceSurfaceFormatsKHR(physical_device, *surface, a, b)
    })
  }

  pub fn list_physical_device_present_modes(&self, physical_device: usize, surface: vk::SurfaceKHR) -> VkRawResult<Vec<vk::PresentModeKHR> {
    loady_listy("physical device present modes", &|a, b| unsafe {
      self.instance_ptrs.GetPhysicalDeviceSurfacePresentModesKHR(physical_device, *surface, a, b)
    })
  }

  pub fn create_logical_device(&'a self, physical_device: usize, device_create_info: &vk::DeviceCreateInfo) -> VkRawResult<VkDevice> {
    loady("logical device", &|a| unsafe {
      self.instance_ptrs.CreateDevice(physical_device, device_create_info, ptr::null(), a)
    }).map(|logical_device| {
      let device_pointers = unsafe {
        vk::DevicePointers::load(|symbol_name| {
          self.instance_ptrs(capable_physical_device.instance).GetDeviceProcAddr(logical_device, symbol_name.as_ptr()) as *const std::os::raw::c_void
        })
      };

      VkDevice {
        _p: PhantomData<&'a>,
        logical_device: logical_device,
        device_ptrs: device_ptrs
      }
    })
  }

  // TODO(acmcarther): Different semantics for "get" vs "fetch" or something
  pub fn get_physical_device_properties(&self, physical_device: u32) -> vk::PhysicalDeviceProperties {
    unsafe {
      let mut physical_device_properties: vk::PhysicalDeviceProperties = std::mem::uninitialized();
      self.instance_ptrs.GetPhysicalDeviceProperties(*_physical_device, &mut physical_device_properties);
      physical_device_properties
    }
  }

  pub fn get_physical_device_features(&self, physical_device: u32) -> vk::PhysicalDeviceProperties {
    unsafe {
      let mut physical_device_features: vk::PhysicalDeviceFeatures = std::mem::uninitialized();
      self.instance_ptrs.GetPhysicalDeviceFeatures(*physical_device, &mut physical_device_features);
      physical_device_features
    }
  }


  pub fn get_physical_device_surface_support(&self, physical_device: u32, queue_family_idx: u32, surface: &vk::SurfaceKHR) -> VkRawResult<vk::Bool32> {
    loady("surface-device suitability", &|a| unsafe {
      self.instance_ptrs(self.instance)
        .GetPhysicalDeviceSurfaceSupportKHR(*physical_device, queue_family_idx, *surface, a)
    });
  }

  pub fn create_xlib_surface_khr(&self, xlib_surface_create_info_khr: &vk::XlibSurfaceCreateInfoKHR) -> VkRawResult<VkSurface> {
    loady("xlib surface", &|a| unsafe {
      self.instance_ptrs(self.instance)
        .CreateXlibSurfaceKHR(self.instance, xlib_surface_create_info_khr, std::ptr::null(), a)
    }).map(|surface| {
      VkSurface {
        parent_instance: self,
        surface: surface
      }
    })
  }
}

pub struct VkSurface<'a> {
  parent_instance: &VkInstance,
  // TODO(acmcarther): Raw access to this is not particularly safe (as the underlying data is copy)
  // Its not clear what the best way to protect is is.
  pub surface: vk::SurfaceKHR,
}

impl Drop for VkSurface {
  fn drop(&mut self) {
    parent_instance.instance_ptr(parent_instance.instance)
      .DestroySurfaceKHR(parent_instance.instance, self.surface, ptr::null());
  }
}

impl VkSurface {
  fn create(parent_instance: &VkInstance, surface: vk::SurfaceKHR) -> VkSurface {
     VkSurface {
      parent_instance: instance
      // TODO(acmcarther): Raw access to this is not particularly safe (as the underlying data is copy)
      // Its not clear what the best way to protect is is.
      ppub surface: surface,
    }
  }
}


pub struct VkDebugReportCallback<'a> {
  destroy: &'a Fn(vk::DebugReportCallbackEXT),
  debug_report_callback: vk::DebugReportCallbackEXT,
}

pub struct VkDevice<'a> {
  _p: PhantomData<'a>,
  instance: vk::Instance,
  logical_device: vk::Device,
  device_ptrs: vk::DevicePointers,
}

impl Drop for VkInstance {
  fn drop(&mut self) {
    self.device_ptrs.DestroyDevice(self.logical_device, ptr::null());
  }
}

impl VkDevice {
  pub fn create_swapchain(&self, swapchain_create_info_khr: &vk::SwapchainCreateInfoKHR) -> VkRawResult<vk::SwapchainKHR> {
    loady("swapchain", &|a| unsafe {
      self.device_ptrs.CreateSwapchainKHR(self.logical_device, swapchain_create_info_khr, ptr::null(), a)
    })
  }

  // Use for recording command buffers
  pub unsafe fn ptrs(&self) -> &vk::DevicePointers {
    &self.device_ptrs
  }

  // Unsafe because vk::Images aren't properly tied to their parent swapchain
  pub unsafe fn get_swapchain_images(&self, swapchain: &VkSwapchainKHR) -> VkRawResult<Vec<vk::Image>> {
    loady_listy("swapchain images", &|a, b| unsafe {
      self.device_ptrs.GetSwapchainImagesKHR(self.logical_device, *swapchain.swapchain, a, b)
    })
  }

  pub fn create_image_view(&self, image_view_create_info: &vk::ImageViewCreateInfo) -> VkRawResult<vk::ImageView> {
    loady("swapchain image view", &|a| unsafe {
      self.device_ptrs.CreateImageView(self.logical_device, image_view_create_info, ptr::null(), a)
    })
  }

  pub fn create_render_pass(&self, render_pass_create_info: &vk::RenderPassCreateInfo) -> VkRawResult<vk::RenderPass> {
    loady("render pass", &|a| unsafe {
      self.device_ptrs.CreateRenderPass(self.logical_device, render_pass_create_info, ptr::null(), a)
    })
  }

  pub fn create_pipeline_layout(&self, pipeline_layout_create_info: &vk::PipelineLayoutCreateInfo) -> VkRawResult<vk::PipelineLayout> {
    loady("pipeline layout", &|a| unsafe {
      self.device_ptrs.CreatePipelineLayout(self.logical_device, pipeline_layout_create_info, ptr::null(), a)
    })
  }

  pub fn create_graphics_pipelines(&self, graphics_pipeline_layout_create_infos: &[vk::GraphicsPipelineCreateInfo]) -> VkRawResult<Vec<vk::Pipeline>> {
    // N.B.: Not generic because the number of pipelines is known in advance
    unsafe {
      let num_items = graphics_pipeline_layout_create_infos.len();
      let mut graphics_pipelines = Vec::with_capacity(num_items);
      let result = self.device_ptrs.CreateGraphicsPipelines(
        self.logical_device,
        0 /* pipelineCache */,
        num_items as u32,
        graphics_pipeline_layout_create_infos.as_ptr(),
        ptr::null(),
        graphics_pipelines.as_mut_ptr());

      if result != vk::SUCCESS {
        return Err(VkRawReturnCode(result as i32, format!("while fetching list of graphics pipelines")))
      }
      graphics_pipelines.set_len(num_items);
      Ok(graphics_pipelines)
    }
  }

  pub fn create_shader_module(&self, shader_module_create_info: &vk::ShaderModuleCreateInfo) -> VkRawResult<vk::ShaderModule> {
    loady("shader module", &|a| unsafe {
      self.device_ptrs.CreateShaderModule(self.logical_device, shader_module_create_info, ptr::null(), a)
    })
  }

  pub fn create_framebuffer(&self, framebuffer_create_info: &vk::FramebufferCreateInfo) -> VkRawResult<vk::Framebuffer> {
    loady("framebuffer", &|a| unsafe {
      self.device_ptrs.CreateFramebuffer(self.logical_device, framebuffer_create_info, ptr::null(), a)
    })
  }

  pub fn create_command_pool(&self, command_pool_create_info: &vk::CommandPoolCreateInfo) -> VkRawResult<vk::CommandPool> {
    loady("command pool", &|a| unsafe {
      self.device_ptrs.CreateCommandPool(self.logical_device, command_pool_create_info, ptr::null(), a)
    })
  }

  pub fn allocate_command_buffers(&self, command_buffer_allocate_info: &vk::CommandBufferAllocateInfo, framebuffers: &[VkFramebuffer]) -> VkRawResult<Vec<vk::CommandBuffer>> {
    // N.B.: Not generic because the number of command buffers is known in advance
    unsafe {
      let mut command_buffers = Vec::with_capacity(framebuffers.len());
      let result = self.device_ptrs.AllocateCommandBuffers(
        self.logical_device, command_buffer_allocate_info, command_buffers.as_mut_ptr());

      if result != vk::SUCCESS {
        return Err(VkRawReturnCode(result as i32, format!("while allocating list of command buffers")))
      }

      command_buffers.set_len(framebuffers.len() as usize);
      Ok(command_buffers)
    }
  }

  pub fn create_semaphore(&self, semaphore_create_info: &vk::SemaphoreCreateInfo) -> VkRawResult<vk::Semaphore> {
    loady("semaphore", &|a| unsafe {
      self.device_ptrs.CreateSemaphore(self.logical_device, semaphore_create_info, ptr::null(), a)
    })
  }

  pub fn get_device_queue(&self, queue_family_idx: u32, queue_index: u32) -> vk::Queue {
    loady("queue", &|a| unsafe {
      self.device_ptrs.GetDeviceQueue(self.logical_device, queue_family_idx, queue_index, a)
    })
  }
}

struct VkSemaphore;
struct VkFramebuffer;
struct VkShaderModule;
struct VkPipelineLayout;
struct VkRenderPass;
struct VkCommandBuffer;
struct VkImageView;
struct VkPipeline;
struct VkCommandPool;
struct VkSwapchainKHR;

pub struct VkRenderSession {
  logical_device: vk::Device,
  swapchain_khr: vk::SwapchainKHR
}

struct SwapchainParams {
  format: vk::Format,
  extent: vk::Extent2D,
}

/*
  pub fn draw_demo_frame(&mut self) {
    let &VkRenderSession {logical_device, swapchain_khr} = vk_render_session;
    let mut image_index = 0;
    unsafe {
      let image_index = do_or_die!(loady("next image", &|a| self.device_ptrs(logical_device).AcquireNextImageKHR(
        logical_device,
        swapchain_khr,
        u64::max_value(),
        *self.device_image_available_semaphore_map.get(&logical_device).unwrap(),
        0 /* vk_null_handle */,
        a)));

      let wait_semaphores = [*self.device_image_available_semaphore_map.get(&logical_device).unwrap()];
      let wait_stages = [vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];
      let signal_semaphores = [*self.device_render_finished_semaphore_map.get(&logical_device).unwrap()];
      let submit_info = vk::SubmitInfo {
        sType: vk::STRUCTURE_TYPE_SUBMIT_INFO,
        pNext: ptr::null(),
        waitSemaphoreCount: 1,
        pWaitSemaphores: wait_semaphores.as_ptr(),
        pWaitDstStageMask: wait_stages.as_ptr(),
        commandBufferCount: 1,
        pCommandBuffers: self.device_command_buffers_map.get(&logical_device).unwrap().get(image_index as usize).unwrap(),
        signalSemaphoreCount: 1,
        pSignalSemaphores: signal_semaphores.as_ptr(),
      };

      do_or_die!(dooy("queue submit", &|| {self.device_ptrs(logical_device)
        .QueueSubmit(*self.queues_map.get(&logical_device).unwrap(), 1, &submit_info, 0 /* vk_null_handle */)}));

      let swapchains = [swapchain_khr];
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

      do_or_die!(dooy("queue present", &|| self.device_ptrs(logical_device).QueuePresentKHR(
        *self.queues_map.get(&logical_device).unwrap(),
        &present_info_khr)));
    }
  }
*/

pub struct FeatureSpec {
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

// TODO(acmcarther): This is a higher level function, inlined here during refactoring
// The intent is that the application will use this api directly
fn make_instance(application_name: &'static str, engine_name: &'static str, extensions: &Vec<[i8; 256]>, layers: &Vec<[i8; 256]>, create_fn: Fn(&vk::InstanceCreateInfo) -> VkRawResult<VkInstance>) -> VkInstance {
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
fn make_debug_report_callback(cb: vk::PFN_vkDebugReportCallbackEXT, create_fn: &Fn(&vk::DebugReportCallbackCreateInfoEXT) -> VkRawResult<vk::DebugReportCallbackCreateInfoEXT> -> vk::DebugReportCallbackEXT {
  do_or_die!(create_fn(&vk::DebugReportCallbackCreateInfoEXT {
    sType: vk::STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
    flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT,
    pNext: ptr::null(),
    pfnCallback: cb,
    pUserData: ptr::null_mut(),
  }));
}

// TODO(acmcarther): Some of this should be refactored out. Callers will want to customize how they
// select devices
fn find_capable_gfx_device(v_i: &VkInstance, surface: &VkSurface) -> Option<CapablePhysicalDevice> {
  let mut physical_device: vk::PhysicalDevice = 0;
  let mut swapchain_capabilities: vk::SurfaceCapabilitiesKHR = unsafe { std::mem::uninitialized() };
  let mut swapchain_formats: Vec<vk::SurfaceFormatKHR> = Vec::new();
  let mut swapchain_present_modes: Vec<vk::PresentModeKHR> = Vec::new();
  let mut gfx_supporting_queue_family_index: u32 = 0;
  let physical_devices = do_or_die!(v_i.list_physical_devices());
  unsafe {
    for _physical_device in physical_devices.iter() {
      let mut physical_device_properties: vk::PhysicalDeviceProperties = v_i.get_physical_device_properties();

      let mut physical_device_features: vk::PhysicalDeviceFeatures = v_i.get_physical_device_features();

      println!("Vulkan Physical Device found: {}", CStr::from_ptr(physical_device_properties.deviceName.as_ptr()).to_str().unwrap());

      let gfx_supporting_queue_family_index_opt = {
        let surface_is_supported_for_queue_idx_fn = |queue_family_idx| {
          let mut support_is_present = do_or_die!(v_i.get_physical_device_surface_support(*_physical_device, queue_family_idx, *surface.surface));

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

      let _swapchain_capabilities = do_or_die!(v_i.get_physical_device_surface_capabilities(*_physical_device, *surface.surface));

      let _swapchain_formats = do_or_die!(v_i.list_physical_device_surface_formats(*_physical_device, *surface.surface));

      let _swapchain_present_modes = do_or_die!(v_i.list_physical_device_present_modes(*_physical_device, *surface.surface));

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

// TODO(acmcarther): This needs to be customizable
fn make_logical_device(capable_physical_device: &CapablePhysicalDevice, enabled_layers: &Vec<[i8; 256]>, create_fn: &Fn(usize, &vk::DeviceCreateInfo) -> VkRawResult<VkDevice>) -> VkDevice {
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

pub fn make_swap_chain(logical_device: &VkDevice, capable_physical_device: &CapablePhysicalDevice, surface: &VkSurface, create_fn: &Fn(&vk::SwapchainCreateInfoKHR) -> VkRawResult<VkSwapchainKHR>) -> (SwapchainParams, VkSwapchainKHR) {
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
      surface: *surface.surface,
    }
  };
  println!("Vulkan creating swap chain");

  let swapchain = do_or_die!(create_fn(&swapchain_create_info_khr));

  (SwapchainParams {
    format: swapchain_best_format.format,
    extent: swapchain_extent,
  }), swapchain)
}

fn make_image_views(swapchain_images: &Vec<vk::Image>, params: &SwapchainParams, create_fn: &Fn(&vk::ImageViewCreateInfo) -> VkRawResult<VkImageView>) -> Vec<VkImageView> {
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

    image_views.push(do_or_die!(self.create_image_view(logical_device, &image_view_create_info)));
  }

  image_views
}

pub fn make_render_pass(&mut self, swapchain_params: &SwapchainParams, create_fn: &Fn(&vk::RenderPassCreateInfo) -> VkRawResult<VkRenderPass>) -> VkRenderPass {
  let swapchain_params = self.swapchain_params_map.get(&swapchain_khr).unwrap();
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

pub fn make_shader_module(shader_bytes: &[u8], create_fn: &Fn(&vk::ShaderModuleCreateInfo) -> VkRawResult<VkShaderModule>) -> VkShaderModule {
  let shader_module_create_info = vk::ShaderModuleCreateInfo {
    sType: vk::STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    codeSize: vert_shader_bytes.len(),
    pCode: vert_shader_bytes.as_ptr() as *const u32,
  };
  do_or_die!(create_fn(&shader_module_create_info));
}

pub fn make_pipeline_layout(create_fn: &Fn(&vk::PipelineLayoutCreateInfo) -> VkRawResult<VkPipelineLayout>) -> VkPipelineLayout {
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

pub fn make_graphics_pipeline(vert_shader_module: &VkShaderModule, frag_shader_module: &VkShaderModule, swapchain_params: &SwapchainParams, pipeline_layout: &VkPipelineLayout, create_fn: &Fn(&[vk::GraphicsPipelineCreateInfo]) -> VkRawResult<Vec<VkPipeline>>) -> VkPipeline {

  let common_shader_pipeline_name = CString::new("main").unwrap();
  let pName = common_shader_pipeline_name.as_c_str().as_ptr();
  let vert_pipeline_shader_stage_create_info = vk::PipelineShaderStageCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    stage: vk::SHADER_STAGE_VERTEX_BIT,
    module: *vert_shader_module.module,
    pName: pName,
    pSpecializationInfo: ptr::null(),
  };

  let frag_pipeline_shader_stage_create_info = vk::PipelineShaderStageCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    stage: vk::SHADER_STAGE_FRAGMENT_BIT,
    module: *frag_shader_module.module,
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

  let swapchain_params = self.swapchain_params_map.get(&swapchain_khr).unwrap();
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
    layout: *pipeline_layout.layout,
    renderPass: render_pass,
    subpass: 0,
    basePipelineHandle: 0 /* vk_null_handle */,
    basePipelineIndex: -1,
  };

  do_or_die!(create_fn(&vec![graphics_pipeline_create_info])).get(0).unwrap()
}

pub fn make_framebuffers(image_views: &Vec<VkImageView>, swapchain_params: &SwapchainParams, render_pass: &VkRenderPass, create_fn: &Fn(&vk::FramebufferCreateInfo) -> VkRawResult<VkFramebuffer>) -> Vec<vk::Framebuffer> {
  let mut framebuffers = Vec::with_capacity(mage_views.len());
  for swapchain_image_view in image_views.iter() {
    unsafe {
      let framebuffer_create_info = vk::FramebufferCreateInfo {
        sType: vk::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        renderPass: render_pass.pass,
        attachmentCount: 1,
        pAttachments: swapchain_image_view,
        width: swapchain_params.extent.width,
        height: swapchain_params.extent.height,
        layers: 1,
      };

      framebuffers.push(do_or_die!(create_fn(&framebuffer_create_info)));
    }
  }

  framebuffers
}

pub fn make_command_pool(capable_physical_device: &CapablePhysicalDevice, create_fn: &Fn(&vk::CommandPoolCreateInfo) -> VkRawResult<VkCommandPool>) -> VkCommandPool {
  let command_pool_create_info = vk::CommandPoolCreateInfo {
    sType: vk::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    queueFamilyIndex: capable_physical_device.gfx_supporting_queue_family_index,
  };

  do_or_die!(create_fn(&command_pool_create_info))
}

pub fn make_command_buffers(swapchain_params: &SwapchainParams, framebuffers: &Vec<VkFramebuffer>, command_pool: &VkCommandPool, allocate_fn: &Fn([&vk::CommandBufferAllocateInfo, &[VkFramebuffer]) -> VkRawResult<Vec<VkCommandBuffer>>) -> Vec<VkCommandBuffer> {
  let swapchain_params = self.swapchain_params_map.get(&swapchain_khr).unwrap();

  let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
    sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    pNext: ptr::null(),
    commandPool: command_pool,
    level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
    commandBufferCount: framebuffers.len() as u32,
  };

  do_or_die!(allocate_fn(&command_buffer_allocate_info, &framebuffers));
}

pub fn record_command_buffers(v_d: &VkDevice, swapchain_params: &SwapchainParams, framebuffers: &Vec<VkFramebuffer>, render_pass: &VkRenderPass, graphics_pipeline: &VkPipeline, command_buffers: &Vec<VkCommandBuffer>) {
  for (idx, command_buffer) in command_buffers.iter().enumerate() {
    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
      sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
      pNext: ptr::null(),
      flags: vk::COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
      pInheritanceInfo: ptr::null(),
    };

    do_or_die!(dooy("start command buffer", &|| v_d.ptrs().BeginCommandBuffer(*command_buffer, &command_buffer_begin_info)));

    {
      let clear_color = vk::ClearValue {
        color: vk::ClearColorValue {
          float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32]
        }
      };
      let render_pass_begin_info = vk::RenderPassBeginInfo {
        sType: vk::STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        pNext: ptr::null(),
        renderPass: *render_pass.pass,
        framebuffer: **(framebuffers.get(idx).unwrap().buffer),
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
        v_d.ptrs().CmdBindPipeline(*command_buffer, vk::PIPELINE_BIND_POINT_GRAPHICS, *graphics_pipeline.pipeline);
        v_d.ptrs().CmdDraw(*command_buffer, 3, 1, 0, 0);
        v_d.ptrs().CmdEndRenderPass(*command_buffer);
      }
    }

    do_or_die!(dooy("end command buffer", &|| v_d.ptrs().EndCommandBuffer(*command_buffer)))
  }
}

pub fn make_semaphore(create_fn: &Fn(&vk::SemaphoreCreateInfo) -> VkRawResult<VkSemaphore>) -> VkSemaphore {
  let semaphore_create_info = vk::SemaphoreCreateInfo {
    sType: vk::STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
  };
  do_or_die!(create_fn(logical_device, &semaphore_create_info));
}

pub fn vulkan<W: WindowSystemPlugin>(window_system_plugin: &mut W, vert_shader_bytes: &[u8], frag_shader_bytes: &[u8]) -> (VkCtx, VkRenderSession) {
  let v = Vk::new("libvulkan.so.1");

  let extension_spec = FeatureSpec {
    wanted: vec! [
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
  let enabled_extensions = v.select_extensions(extension_spec);

  let layer_spec = FeatureSpec {
    wanted: vec![
      "VK_LAYER_LUNARG_core_validation",
      "VK_LAYER_LUNARG_parameter_validation",
    ],
    required: vec! [
      "VK_LAYER_LUNARG_standard_validation",
    ],
  };
  let enabled_layers = v.select_layers(layer_spec);

  println!("setting up instance");
  let v_i = make_instance_create("dummy_application_name",
                                 "dummy_engine_name",
                                 &enabled_extensions,
                                 &enabled_layers,
                                 v.create_instance);

  // Configure debug callback
  println!("setting up debug callback");
  let debug_report_callback =
    make_debug_report_callback_create_info(vk_debug_report_callback_ext, v_i.create_debug_callback);

  let mut surface_khr = window_system_plugin.create_surface(&v_i);

  let capable_physical_device = find_capable_gfx_device(&v_i, &v_surface)
    .expect("there was no suitable physical device!");

  let v_d = make_logical_device(&capable_physical_device, &enabled_layers, v_i.create_logical_device);

  //let queue = v_d.get_device_queue(capable_physical_device.queue_family_idx, 0 /* queueIndex */);

  let (swapchain_params, swapchain_khr) = make_swap_chain(&capable_physical_device, &v_surface, v_d.create_swapchain);

  let swapchain_images = do_or_die!(unsafe {v_d.get_swapchain_images(&swapchain) })

  let image_views = make_image_views(&swapchain_images, &swapchain_params, v_d.create_image_view)

  let render_pass = make_render_pass(&swapchain_params, v_d.create_render_pass);

  let vert_shader_module = make_shader_module(vert_shader_bytes, v_d.create_shader_module);

  let frag_shader_module = make_shader_module(vert_shader_bytes, v_d.create_shader_module);

  let pipeline_layout = make_pipeline_layout(v_d.create_pipeline_layout);

  let graphics_pipeline = make_graphics_pipeline(&vert_shader_module, &frag_shader_module, &swapchain_params, &pipeline_layout, v_d.create_graphics_pipelines);

  let framebuffers = make_framebuffers(&image_views, &swapchain_params, &render_pass, v_d.create_framebuffer);

  let command_pool = make_command_pool(&capable_physical_device, v_d.create_command_pool);

  let command_buffers = make_command_buffers(&swapchain_params, &framebuffers, &command_pool, v_d.allocate_command_buffers)

  record_command_buffers(&v_d, &swapchain_params, &framebuffers, &render_pass, &graphics_pipeline, &command_buffers)

  let mut image_available_semaphore = make_semaphore(v_d.create_semaphore);
  let mut render_finished_semaphore = make_semaphore(v_d.create_semaphore);

  // Return Something!
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
