use std::ffi::CString;
use std::ptr;
#[macro_use]
use lite;
use vk_sys;

#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct InstanceCfg {
  application_name: String,
  engine_name: String,
}

impl Default for InstanceCfg {
  fn default() -> InstanceCfg {
    InstanceCfg {
      application_name: "vulkan_application".to_owned(),
      engine_name: "custom_engine".to_owned(),
    }
  }
}

#[allow(non_snake_case)]
pub fn make_instance(
  instance_cfg: InstanceCfg,
  enabled_extensions: &Vec<[i8; 256]>,
  enabled_layers: &Vec<[i8; 256]>,
  create_fn: &Fn(&vk_sys::InstanceCreateInfo) -> lite::RawResult<lite::LInstance>,
) -> lite::RawResult<lite::LInstance> {
  let pApplicationName = CString::new(instance_cfg.application_name).unwrap();
  let pEngineName = CString::new(instance_cfg.engine_name).unwrap();

  // Set up application
  let vk_application_info = vk_sys::ApplicationInfo {
    sType: vk_sys::STRUCTURE_TYPE_APPLICATION_INFO,
    pNext: ptr::null(),
    pApplicationName: pApplicationName.as_ptr(),
    applicationVersion: 1,
    pEngineName: pEngineName.as_ptr(),
    engineVersion: 1,
    apiVersion: 0, /* 1? */
  };

  let ppEnabledExtensionNames = enabled_extensions
    .iter()
    .map(|i| i.as_ptr())
    .collect::<Vec<_>>();
  let ppEnabledLayerNames = enabled_layers
    .iter()
    .map(|i| i.as_ptr())
    .collect::<Vec<_>>();

  create_fn(&vk_sys::InstanceCreateInfo {
    sType: vk_sys::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
    pApplicationInfo: &vk_application_info as *const _,
    flags: 0,
    pNext: ptr::null(),
    enabledLayerCount: ppEnabledLayerNames.len() as u32,
    ppEnabledLayerNames: ppEnabledLayerNames.as_ptr(),
    enabledExtensionCount: ppEnabledExtensionNames.len() as u32,
    ppEnabledExtensionNames: ppEnabledExtensionNames.as_ptr(),
  })
}
