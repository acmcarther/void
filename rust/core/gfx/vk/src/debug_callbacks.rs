use lite::LInstance;
use vk_sys;
use std::sync::Arc;
use std::sync::Mutex;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::collections::HashMap;
use std::any::Any;

/*
  pub extern "system" fn vk_debug_report_callback_ext(
    flags: vk_sys::DebugReportFlagsEXT,
    object_type: vk_sys::DebugReportObjectTypeEXT,
    obj: u64,
    location: usize,
    code: i32,
    layer_prefix: *const c_char,
    msg: *const c_char,
    user_data: *mut c_void,
  ) -> vk_sys::Bool32 {
    unsafe {
      debug!(
        "validation layer: {}",
        CStr::from_ptr(msg).to_str().unwrap()
      );
    }
    vk_sys::FALSE
  }
  */


pub trait VkDebugCallback {
  fn debug_callback(
    &mut self,
    flags: vk_sys::DebugReportFlagsEXT,
    object_type: vk_sys::DebugReportObjectTypeEXT,
    obj: u64,
    location: usize,
    code: i32,
    layer_prefix: *const c_char,
    msg: *const c_char,
    user_data: *mut c_void);
}

pub struct VkCallbackManager {
  vk_instance: Arc<Mutex<LInstance>>,
  callbacks: Mutex<HashMap<vk_sys::DebugReportCallbackEXT,
                           Option<Box<Any>>>>,
}

impl VkCallbackManager {
  pub fn new(vk_instance: Arc<Mutex<LInstance>>) -> VkCallbackManager {
    VkCallbackManager {
      vk_instance: vk_instance,
      callbacks: Mutex::new(HashMap::new()),
    }
  }

  pub fn register(&mut self,
                  cb: vk_sys::PFN_vkDebugReportCallbackEXT,
                  callback_state: Option<Box<Any>>) {
    // TODO?
  }

  pub fn unregister(&mut self, debug_report_callback: vk_sys::DebugReportCallbackEXT) {
    let instance = self.vk_instance.lock().unwrap();
    let mut callbacks = self.callbacks.lock().unwrap();
    if !callbacks.contains_key(&debug_report_callback) {
      return;
    }

    callbacks.remove(&debug_report_callback);
    instance.destroy_debug_callback(debug_report_callback);
  }

  pub fn unregister_all(&mut self) {
    let instance = self.vk_instance.lock().unwrap();
    let mut callbacks = self.callbacks.lock().unwrap();
    for callback in callbacks.keys() {
      instance.destroy_debug_callback(*callback);
    }
    callbacks.clear();
  }
}

impl Drop for VkCallbackManager {
  fn drop(&mut self) {
    self.unregister_all();
  }
}
