extern crate dylib;
extern crate vk_sys as vk;

use std::ffi::CStr;
use std::path::PathBuf;
use std::ptr;
use std::os::raw::c_void;

pub struct FeatureSpec {
  pub wanted: Vec<&'static str>,
  pub required: Vec<&'static str>
}

pub mod util {
  use ::RawReturnCode;
  use ::RawResult;
  use vk;
  use std::ptr;
  use std;

  /** Performs a no-result Vulkan action, yielding a Rust-idiomatic result type. */
  pub fn dooy(msg: &str, f: &Fn() -> u32) -> RawResult<()> {
    let result = f();

    if result != vk::SUCCESS {
      return Err(RawReturnCode(result as i32, format!("while doing {}", msg)));
    }

    Ok(())
  }

  /** Loads a Vulkan value, yielding a Rust-idiomatic result type. */
  pub fn loady<T>(msg: &str, f: &Fn(*mut T) -> u32) -> RawResult<T> {
    unsafe {
      let mut item = std::mem::uninitialized();
      let result = f(&mut item);

      if result != vk::SUCCESS {
        return Err(RawReturnCode(result as i32, format!("while getting {}", msg)));
      }

      Ok(item)
    }
  }

  /** Fetches a list of vulkan values, yielding a Rust-idiomatic result type. */
  pub fn loady_listy<T>(msg: &str, f: &Fn(&mut u32, *mut T) -> u32) -> RawResult<Vec<T>> {
    let mut num_items = 0;
    let result = f(&mut num_items, ptr::null::<T>() as *mut _);
    if result != vk::SUCCESS {
      return Err(RawReturnCode(result as i32, format!("while enumerating {}", msg)))
    }

    let mut items = Vec::with_capacity(num_items as usize);

    let result = f(&mut num_items, items.as_mut_ptr());
    if result != vk::SUCCESS {
      return Err(RawReturnCode(result as i32, format!("while fetching list of {}", msg)))
    }

    unsafe { items.set_len(num_items as usize); }

    Ok(items)
  }
}

pub trait WindowSystemPlugin {
  /** Creates a LSurface object using foreign window system internals */
  fn create_surface(&mut self, instance: &LInstance) -> RawResult<vk::SurfaceKHR>;
}

/** The wrapped vulkan return code, plus call context */
#[derive(Debug)]
pub struct RawReturnCode(pub i32, pub String);

pub type RawResult<T> = Result<T, RawReturnCode>;

pub struct Vulkan {
  _dylib: dylib::DynamicLibrary,
  entry_points: vk::EntryPoints,
  static_points: vk::Static,
}

impl Vulkan {
  pub fn new(dylib_path: &'static str) -> Vulkan {
    let dylib_path = PathBuf::from(dylib_path);
    let dylib = dylib::DynamicLibrary::open(Some(dylib_path.as_path())).unwrap();

    let entry_points = vk::EntryPoints::load(|symbol_name| unsafe {
      dylib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void
    });
    let static_points = vk::Static::load(|symbol_name| unsafe {
      dylib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void
    });

    Vulkan {
      _dylib: dylib,
      entry_points: entry_points,
      static_points: static_points,
    }
  }

  pub fn select_extensions(&self, spec: FeatureSpec) -> RawResult<Vec<[i8; 256]>> {
    let extensions = try!(self.list_instance_extensions());
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

      Ok(enabled_extensions)
    }
  }

  pub fn select_layers(&self, spec: FeatureSpec) -> RawResult<Vec<[i8; 256]>> {
    let layers = try!(self.list_instance_layers());
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

      Ok(enabled_layers)
    }
  }


  pub fn create_instance(&self, instance_create_info: &vk::InstanceCreateInfo) -> RawResult<LInstance> {
    util::loady("instance", &|a| unsafe {
      self.entry_points.CreateInstance(instance_create_info, ptr::null(), a)
    }).map(|instance| {
      let instance_ptrs = unsafe {
        vk::InstancePointers::load(|symbol_name| {
          self.static_points.GetInstanceProcAddr(instance, symbol_name.as_ptr()) as *const std::os::raw::c_void
        })
      };
      LInstance {
        instance: instance,
        instance_ptrs: instance_ptrs
      }
    })
  }

  pub fn list_instance_extensions(&self) -> RawResult<Vec<vk::ExtensionProperties>>  {
    util::loady_listy("instance extensions", &|a, b| unsafe {
      self.entry_points.EnumerateInstanceExtensionProperties(ptr::null() /* pLayerName */, a, b)
    })
  }

  pub fn list_instance_layers(&self) -> RawResult<Vec<vk::LayerProperties>> {
    util::loady_listy("instance layers", &|a, b| unsafe {
      self.entry_points.EnumerateInstanceLayerProperties(a, b)
    })
  }
}

pub struct LInstance {
  instance: vk::Instance,
  instance_ptrs: vk::InstancePointers,
}

impl Drop for LInstance {
  fn drop(&mut self) {
    unsafe {
      self.instance_ptrs.DestroyInstance(self.instance, ptr::null())
    }
  }
}

impl LInstance {
  // TODO(acmcarther): Create a "dies on drop" object for debug report callback.
  pub fn create_debug_callback(&self, debug_report_callback_create_info_ext: &vk::DebugReportCallbackCreateInfoEXT) -> RawResult<vk::DebugReportCallbackEXT> {
    util::loady("register debug callback", &|a| unsafe {
      self.instance_ptrs
        .CreateDebugReportCallbackEXT(self.instance, debug_report_callback_create_info_ext, ptr::null(), a)
    })
  }

  pub fn destroy_debug_callback(&self, debug_report_callback: vk::DebugReportCallbackEXT) {
    unsafe {
      self.instance_ptrs.DestroyDebugReportCallbackEXT(self.instance, debug_report_callback, ptr::null())
    }
  }

  pub fn list_physical_devices(&self) -> RawResult<Vec<usize>> {
    util::loady_listy("physical devices", &|a, b| unsafe {
      self.instance_ptrs.EnumeratePhysicalDevices(self.instance, a, b)
    })
  }

  pub fn list_queue_family_properties(&self, physical_device: usize) -> Vec<vk::QueueFamilyProperties> {
    // N.B.: Not generic because FFI method does not return a RawResult
    unsafe {
      let mut num_queue_family_properties = 0u32;
      self.instance_ptrs.GetPhysicalDeviceQueueFamilyProperties(
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

  pub fn list_device_extension_properties(&self, physical_device: vk::PhysicalDevice) -> RawResult<Vec<vk::ExtensionProperties>> {
    util::loady_listy("physical device extension properties", &|a, b| unsafe {
      self.instance_ptrs.EnumerateDeviceExtensionProperties(physical_device, ptr::null() /* pLayerName */, a, b)
    })
  }

  pub fn get_physical_device_surface_capabilities(&self, physical_device: vk::PhysicalDevice, surface: &vk::SurfaceKHR) -> RawResult<vk::SurfaceCapabilitiesKHR> {
    util::loady("physical device surface properties", &|a| unsafe {
      self.instance_ptrs.GetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, *surface, a)
    })
  }

  pub fn list_physical_device_surface_formats(&self, physical_device: vk::PhysicalDevice, surface: &vk::SurfaceKHR) -> RawResult<Vec<vk::SurfaceFormatKHR>> {
    util::loady_listy("physical device surface formats", &|a, b| unsafe {
      self.instance_ptrs.GetPhysicalDeviceSurfaceFormatsKHR(physical_device, *surface, a, b)
    })
  }

  pub fn list_physical_device_present_modes(&self, physical_device: vk::PhysicalDevice, surface: &vk::SurfaceKHR) -> RawResult<Vec<vk::PresentModeKHR>> {
    util::loady_listy("physical device present modes", &|a, b| unsafe {
      self.instance_ptrs.GetPhysicalDeviceSurfacePresentModesKHR(physical_device, *surface, a, b)
    })
  }

  pub fn create_logical_device(&self, physical_device: vk::PhysicalDevice, device_create_info: &vk::DeviceCreateInfo) -> RawResult<LDevice> {
    util::loady("logical device", &|a| unsafe {
      self.instance_ptrs.CreateDevice(physical_device, device_create_info, ptr::null(), a)
    }).map(|logical_device| {
      let device_ptrs = unsafe {
        vk::DevicePointers::load(|symbol_name| {
          self.instance_ptrs.GetDeviceProcAddr(logical_device, symbol_name.as_ptr()) as *const std::os::raw::c_void
        })
      };

      LDevice {
        logical_device: logical_device,
        device_ptrs: device_ptrs
      }
    })
  }

  // TODO(acmcarther): Different semantics for "get" vs "fetch" or something
  pub fn get_physical_device_properties(&self, physical_device: vk::PhysicalDevice) -> vk::PhysicalDeviceProperties {
    unsafe {
      let mut physical_device_properties: vk::PhysicalDeviceProperties = std::mem::uninitialized();
      self.instance_ptrs.GetPhysicalDeviceProperties(physical_device, &mut physical_device_properties);
      physical_device_properties
    }
  }

  pub fn get_physical_device_features(&self, physical_device: vk::PhysicalDevice) -> vk::PhysicalDeviceFeatures {
    unsafe {
      let mut physical_device_features: vk::PhysicalDeviceFeatures = std::mem::uninitialized();
      self.instance_ptrs.GetPhysicalDeviceFeatures(physical_device, &mut physical_device_features);
      physical_device_features
    }
  }

  pub fn get_physical_device_memory_properties(&self, physical_device: vk::PhysicalDevice) -> vk::PhysicalDeviceMemoryProperties {
    unsafe {
      let mut physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties = std::mem::uninitialized();
      self.instance_ptrs.GetPhysicalDeviceMemoryProperties(physical_device, &mut physical_device_memory_properties);
      physical_device_memory_properties
    }
  }

  pub fn get_physical_device_surface_support(&self, physical_device: vk::PhysicalDevice, queue_family_idx: u32, surface: &vk::SurfaceKHR) -> RawResult<vk::Bool32> {
    util::loady("surface-device suitability", &|a| unsafe {
      self.instance_ptrs
        .GetPhysicalDeviceSurfaceSupportKHR(physical_device, queue_family_idx, *surface, a)
    })
  }

  pub fn create_xlib_surface_khr(&self, xlib_surface_create_info_khr: &vk::XlibSurfaceCreateInfoKHR) -> RawResult<vk::SurfaceKHR> {
    util::loady("xlib surface", &|a| unsafe {
      self.instance_ptrs
        .CreateXlibSurfaceKHR(self.instance, xlib_surface_create_info_khr, std::ptr::null(), a)
    })
  }
}

pub struct LDevice {
  // Public for drawing in demo
  pub logical_device: vk::Device,
  device_ptrs: vk::DevicePointers,
}

impl Drop for LDevice {
  fn drop(&mut self) {
    unsafe {
      self.device_ptrs.DestroyDevice(self.logical_device, ptr::null())
    }
  }
}

impl LDevice {
  // Use for recording command buffers
  pub unsafe fn ptrs(&self) -> &vk::DevicePointers {
    &self.device_ptrs
  }

  pub fn create_swapchain(&self, swapchain_create_info_khr: &vk::SwapchainCreateInfoKHR) -> RawResult<vk::SwapchainKHR> {
    util::loady("swapchain", &|a| unsafe {
      self.device_ptrs.CreateSwapchainKHR(self.logical_device, swapchain_create_info_khr, ptr::null(), a)
    })
  }

  pub fn destroy_swapchain(&self, swapchain: vk::SwapchainKHR) {
    unsafe {
      self.device_ptrs.DestroySwapchainKHR(self.logical_device, swapchain, ptr::null())
    }
  }

  // Unsafe because vk::Images aren't properly tied to their parent swapchain
  pub unsafe fn get_swapchain_images(&self, swapchain: &vk::SwapchainKHR) -> RawResult<Vec<vk::Image>> {
    util::loady_listy("swapchain images", &|a, b| {
      self.device_ptrs.GetSwapchainImagesKHR(self.logical_device, *swapchain, a, b)
    })
  }

  pub fn create_image_view(&self, image_view_create_info: &vk::ImageViewCreateInfo) -> RawResult<vk::ImageView> {
    util::loady("swapchain image view", &|a| unsafe {
      self.device_ptrs.CreateImageView(self.logical_device, image_view_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_image_view(&self, image_view: vk::ImageView) {
    unsafe {self.device_ptrs.DestroyImageView(self.logical_device, image_view, ptr::null()) }
  }

  pub fn create_render_pass(&self, render_pass_create_info: &vk::RenderPassCreateInfo) -> RawResult<vk::RenderPass> {
    util::loady("render pass", &|a| unsafe {
      self.device_ptrs.CreateRenderPass(self.logical_device, render_pass_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_render_pass(&self, render_pass: vk::RenderPass) {
    unsafe {self.device_ptrs.DestroyRenderPass(self.logical_device, render_pass, ptr::null()) }
  }


  pub fn create_pipeline_layout(&self, pipeline_layout_create_info: &vk::PipelineLayoutCreateInfo) -> RawResult<vk::PipelineLayout> {
    util::loady("pipeline layout", &|a| unsafe {
      self.device_ptrs.CreatePipelineLayout(self.logical_device, pipeline_layout_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_pipeline_layout(&self, pipeline_layout: vk::PipelineLayout) {
    unsafe {self.device_ptrs.DestroyPipelineLayout(self.logical_device, pipeline_layout, ptr::null()) }
  }


  pub fn create_graphics_pipelines(&self, graphics_pipeline_layout_create_infos: &[vk::GraphicsPipelineCreateInfo]) -> RawResult<Vec<vk::Pipeline>> {
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
        return Err(RawReturnCode(result as i32, format!("while fetching list of graphics pipelines")))
      }
      graphics_pipelines.set_len(num_items);
      Ok(graphics_pipelines)
    }
  }

  pub fn destroy_pipeline(&self, pipeline: vk::Pipeline) {
    unsafe {self.device_ptrs.DestroyPipeline(self.logical_device, pipeline, ptr::null()) }
  }

  pub fn create_shader_module(&self, shader_module_create_info: &vk::ShaderModuleCreateInfo) -> RawResult<vk::ShaderModule> {
    util::loady("shader module", &|a| unsafe {
      self.device_ptrs.CreateShaderModule(self.logical_device, shader_module_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_shader_module(&self, shader_module: vk::ShaderModule) {
    unsafe {self.device_ptrs.DestroyShaderModule(self.logical_device, shader_module, ptr::null()) }
  }

  pub fn create_framebuffer(&self, framebuffer_create_info: &vk::FramebufferCreateInfo) -> RawResult<vk::Framebuffer> {
    util::loady("framebuffer", &|a| unsafe {
      self.device_ptrs.CreateFramebuffer(self.logical_device, framebuffer_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_framebuffer(&self, framebuffer: vk::Framebuffer) {
    unsafe {self.device_ptrs.DestroyFramebuffer(self.logical_device, framebuffer, ptr::null()) }
  }

  pub fn create_command_pool(&self, command_pool_create_info: &vk::CommandPoolCreateInfo) -> RawResult<vk::CommandPool> {
    util::loady("command pool", &|a| unsafe {
      self.device_ptrs.CreateCommandPool(self.logical_device, command_pool_create_info, ptr::null(), a)
    })
  }

  pub fn reset_command_pool(&self, command_pool: vk::CommandPool) -> RawResult<()> {
    util::dooy("reset command pool", &|| unsafe {
      self.device_ptrs.ResetCommandPool(self.logical_device, command_pool, 0/* VkCommandPoolResetFlagsBit */)
    })
  }

  pub fn destroy_command_pool(&self, command_pool: vk::CommandPool) {
    unsafe {self.device_ptrs.DestroyCommandPool(self.logical_device, command_pool, ptr::null()) }
  }

  pub fn allocate_command_buffers(&self, command_buffer_allocate_info: &vk::CommandBufferAllocateInfo, framebuffers: &[vk::Framebuffer]) -> RawResult<Vec<vk::CommandBuffer>> {
    // N.B.: Not generic because the number of command buffers is known in advance
    unsafe {
      let mut command_buffers = Vec::with_capacity(framebuffers.len());
      let result = self.device_ptrs.AllocateCommandBuffers(
        self.logical_device, command_buffer_allocate_info, command_buffers.as_mut_ptr());

      if result != vk::SUCCESS {
        return Err(RawReturnCode(result as i32, format!("while allocating list of command buffers")))
      }

      command_buffers.set_len(framebuffers.len() as usize);
      Ok(command_buffers)
    }
  }

  pub fn reset_command_buffer(&self, command_buffer: &vk::CommandBuffer) -> RawResult<()> {
    util::dooy("reset command buffer", &|| unsafe {
      self.device_ptrs.ResetCommandBuffer(*command_buffer, 0/* VkCommandBufferResetFlagsBit */)
    })
  }

  pub fn free_command_buffers(&self, command_pool: &vk::CommandPool, command_buffers: Vec<vk::CommandBuffer>) {
    unsafe {
      self.device_ptrs.FreeCommandBuffers(
        self.logical_device, *command_pool, command_buffers.len() as u32, command_buffers.as_ptr());
    }
  }

  pub fn create_fence(&self, fence_create_info: &vk::FenceCreateInfo) -> RawResult<vk::Fence> {
    util::loady("fence", &|a| unsafe {
      self.device_ptrs.CreateFence(self.logical_device, fence_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_fence(&self, fence: vk::Fence) {
    unsafe {self.device_ptrs.DestroyFence(self.logical_device, fence, ptr::null()) }
  }

  pub fn create_buffer(&self, buffer_create_info: &vk::BufferCreateInfo) -> RawResult<vk::Buffer> {
    util::loady("buffer", &|a| unsafe {
      self.device_ptrs.CreateBuffer(self.logical_device, buffer_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_buffer(&self, buffer: vk::Buffer) {
    unsafe {self.device_ptrs.DestroyBuffer(self.logical_device, buffer, ptr::null()) }
  }

  pub fn create_buffer_view(&self, buffer_view_create_info: &vk::BufferViewCreateInfo) -> RawResult<vk::BufferView> {
    util::loady("buffer view", &|a| unsafe {
      self.device_ptrs.CreateBufferView(self.logical_device, buffer_view_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_buffer_view(&self, buffer_view: vk::BufferView) {
    unsafe {self.device_ptrs.DestroyBufferView(self.logical_device, buffer_view, ptr::null()) }
  }

  pub fn get_buffer_memory_requirements(&self, buffer: &vk::Buffer) -> vk::MemoryRequirements {
    unsafe {
      let mut memory_requirements = std::mem::uninitialized();
      self.device_ptrs.GetBufferMemoryRequirements(self.logical_device, *buffer, &mut memory_requirements);
      memory_requirements
    }
  }

  pub fn allocate_memory(&self, memory_allocate_info: &vk::MemoryAllocateInfo) -> RawResult<vk::DeviceMemory> {
    util::loady("allocate memory", &|a| unsafe {
      self.device_ptrs.AllocateMemory(self.logical_device, memory_allocate_info, ptr::null(), a)
    })
  }

  pub fn free_memory(&self, memory: vk::DeviceMemory) {
    unsafe {self.device_ptrs.FreeMemory(self.logical_device, memory, ptr::null()) }
  }

  pub unsafe fn bind_buffer_memory(&self, buffer: &vk::Buffer, device_memory: &vk::DeviceMemory) -> RawResult<()> {
    unsafe {
      util::dooy("bind buffer memory", &|| unsafe {
        self.device_ptrs.BindBufferMemory(self.logical_device,
                                          *buffer,
                                          *device_memory,
                                          0 /* TODO(acmcarther): device offset (necessary if there's more than one binding */)
                                          })
    }
  }

  pub unsafe fn map_data_to_memory<T>(&self, memory: &vk::DeviceMemory, data: &Vec<T>) -> RawResult<()> {
    unsafe {
      let mut bound_data: *mut *mut c_void = std::mem::uninitialized();
      let bound_data = try!(util::loady("map memory", &|a| unsafe {
        self.device_ptrs.MapMemory(self.logical_device,
                                   *memory,
                                   0 /* TODO(acmcarther): device offset (necessary if there's more than one binding) */,
                                   (std::mem::size_of::<T>() * data.len()) as u64,
                                   0 /* vk::MemoryMapFlags */,
                                   a)}));

      ptr::copy_nonoverlapping(data.as_ptr(), bound_data as *mut T, data.len());

      self.device_ptrs.UnmapMemory(self.logical_device, *memory);
      Ok(())
    }
  }

  // TODO(acmcarther): ResetFences
  // TODO(acmcarther): GetFenceStatus
  // TODO(acmcarther): WaitForFences
  // TODO(acmcarther): CreateEvent
  // TODO(acmcarther): DestroyEvent
  // TODO(acmcarther): GetEventStatus
  // TODO(acmcarther): SetEvent
  // TODO(acmcarther): ResetEvent
  // TODO(acmcarther): CreateQueryPool
  // TODO(acmcarther): DestroyQueryPool
  // TODO(acmcarther): GetQueryPoolResults
  // TODO(acmcarther): CreateImage
  // TODO(acmcarther): DestroyImage
  // TODO(acmcarther): GetImageSubresourceLayout
  // TODO(acmcarther): CreatePipelineCache
  // TODO(acmcarther): DestroyPipelineCache
  // TODO(acmcarther): CreateSampler
  // TODO(acmcarther): DestroySampler
  // TODO(acmcarther): CreateDescriptorSetLayout
  // TODO(acmcarther): DestroyDescriptorSetLayout
  // TODO(acmcarther): CreateDescriptorPool
  // TODO(acmcarther): DestroyDescriptorPool
  // TODO(acmcarther): AllocateDescriptorSets
  // TODO(acmcarther): FreeDescriptorSets
  // TODO(acmcarther): UpdateDescriptorSets
  // TODO(acmcarther): GetRenderAreaGranularity
  // TODO(acmcarther): TrimCommandPoolKHR

  pub fn create_semaphore(&self, semaphore_create_info: &vk::SemaphoreCreateInfo) -> RawResult<vk::Semaphore> {
    util::loady("semaphore", &|a| unsafe {
      self.device_ptrs.CreateSemaphore(self.logical_device, semaphore_create_info, ptr::null(), a)
    })
  }

  pub fn destroy_semaphore(&self, semaphore: vk::Semaphore) {
    unsafe {self.device_ptrs.DestroySemaphore(self.logical_device, semaphore, ptr::null()) }
  }

  pub fn get_device_queue(&self, queue_family_idx: u32, queue_index: u32) -> vk::Queue {
    unsafe {
      let mut queue = std::mem::uninitialized();
      self.device_ptrs.GetDeviceQueue(self.logical_device, queue_family_idx, queue_index, &mut queue);
      queue
    }
  }

  pub fn queue_wait_idle(&self, queue: &vk::Queue) -> RawResult<()> {
    util::dooy("waiting for queue to become idle", &|| unsafe {
      self.device_ptrs.QueueWaitIdle(*queue)
    })
  }

  pub fn device_wait_idle(&self) -> RawResult<()> {
    util::dooy("waiting for device to become idle", &|| unsafe {
      self.device_ptrs.DeviceWaitIdle(self.logical_device)
    })
  }
}
