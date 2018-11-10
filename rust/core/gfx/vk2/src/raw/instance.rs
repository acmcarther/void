use vk_sys;
use raw::code::VulkanResult;
use raw::code::VulkanStatus;
use raw::vulkan::Vulkan;
use std::os::raw::c_void;

pub struct Instance<'vk> {
  vk: &'vk Vulkan,
  raw_instance: vk_sys::Instance,
  instance_points: vk_sys::InstancePointers,
}

impl<'vk> Instance<'vk> {
  fn new(vk: &'vk Vulkan, create_info: &vk_sys::InstanceCreateInfo) -> VulkanResult<Instance<'vk>> {
    let raw_instance;
    let instance_points;
    let status;
    unsafe {
      let status_or_raw_instance = vk.create_instance(create_info);
      if let Some(err_code) = status_or_raw_instance.error_opt() {
        return VulkanResult::error(err_code)
      }
      raw_instance = *status_or_raw_instance.value_opt().unwrap();
      instance_points = load_instance_points(vk, raw_instance);
      status = status_or_raw_instance.status();
    }

    VulkanResult::from_nonerr_unchecked(
      status,
      Instance {
        vk: vk,
        raw_instance: raw_instance,
        instance_points: instance_points
      })
  }
}


unsafe fn load_instance_points(vk: &Vulkan, raw_instance: vk_sys::Instance) -> vk_sys::InstancePointers {
  vk_sys::InstancePointers::load(|symbol_name| {
    vk.get_instance_proc_addr(raw_instance, symbol_name.as_ptr()) as *const c_void
  })
}
