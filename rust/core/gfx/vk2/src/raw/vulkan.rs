use dylib::DynamicLibrary;
use vk_sys;
use util::status;
use util::status::Error;
use std::os::raw::c_char;
use std::ffi::CString;
use util::status::EResult;
use std::ptr;
use raw::code::VulkanResult;
use raw::code::VulkanStatus;

pub struct Vulkan {
  _dylib: DynamicLibrary,
  entry_points: vk_sys::EntryPoints,
  static_points: vk_sys::Static,
}

impl Vulkan {
  pub fn new(dylib: DynamicLibrary) -> Vulkan {
    use std::os::raw::c_void;

    Vulkan {
      entry_points: load_entry_points(&dylib),
      static_points: load_static_points(&dylib),
      _dylib: dylib,
    }
  }

  pub fn list_instance_extensions(&self) -> VulkanResult<Vec<vk_sys::ExtensionProperties>> {
    do_vk_exec_to_vec!(
      self.entry_points.EnumerateInstanceExtensionProperties,
      ptr::null() /* pLayerName */)
  }

  pub fn list_instance_extensions_for_layer(&self, layer_name: &str) -> VulkanResult<Vec<vk_sys::ExtensionProperties>> {
    let c_s = CString::new(layer_name).unwrap();
    do_vk_exec_to_vec!(
      self.entry_points.EnumerateInstanceExtensionProperties,
      c_s.as_ptr())
  }

  pub fn list_instance_layers(&self) -> VulkanResult<Vec<vk_sys::LayerProperties>> {
    do_vk_exec_to_vec!(self.entry_points.EnumerateInstanceLayerProperties)
  }

  pub unsafe fn create_instance(&self, create_info: &vk_sys::InstanceCreateInfo) -> VulkanResult<vk_sys::Instance> {
    do_vk_exec_to_val!(self.entry_points.CreateInstance,
                       create_info,
                       ptr::null() /* pAllocator */)
  }

  pub unsafe fn get_instance_proc_addr(
      &self,
      instance: vk_sys::Instance,
      proc_name: *const c_char) -> vk_sys::PFN_vkVoidFunction {
    self.static_points.GetInstanceProcAddr(instance, proc_name)
  }
}

// Load the entry point function pointers from the provided dylib
fn load_entry_points(dylib: &DynamicLibrary) -> vk_sys::EntryPoints {
  use std::os::raw::c_void;

  vk_sys::EntryPoints::load(|symbol_name| unsafe {
    dylib
      .symbol::<*const c_void>(symbol_name.to_str().unwrap())
      .unwrap() as *const c_void
  })
}

// Load the static function pointers from the provided dylib
fn load_static_points(dylib: &DynamicLibrary) -> vk_sys::Static {
  use std::os::raw::c_void;

  vk_sys::Static::load(|symbol_name| unsafe {
    dylib
      .symbol::<*const c_void>(symbol_name.to_str().unwrap())
      .unwrap() as *const c_void
  })
}
