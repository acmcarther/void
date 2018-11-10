use vk_sys;
use raw::code::VulkanResult;
use raw::code::VulkanStatus;

#[macro_export]
macro_rules! do_vk_exec {
  ($f:expr) => {
    #[allow(unused_unsafe)]
    $crate::raw::exec::vk_exec(&|| unsafe {
      $f()
    })
  };
  ($f:expr, $($arg:expr),*) => {
    #[allow(unused_unsafe)]
    $crate::raw::exec::vk_exec(&|| unsafe {
      $f($($arg,)*)
    })
  }
}
#[macro_export]
macro_rules! do_vk_exec_to_val {
  ($f:expr) => {
    #[allow(unused_unsafe)]
    $crate::raw::exec::vk_exec_to_val(&|a| unsafe {
      $f(a)
    })
  };
  ($f:expr, $($arg:expr),*) => {
    #[allow(unused_unsafe)]
    $crate::raw::exec::vk_exec_to_val(&|a| unsafe {
      $f($($arg,)* a)
    })
  }
}
#[macro_export]
macro_rules! do_vk_exec_to_vec {
  ($f:expr) => {
    #[allow(unused_unsafe)]
    $crate::raw::exec::vk_exec_to_vec(&|a, b| unsafe {
      $f(a, b)
    })
  };
  ($f:expr, $($arg:expr),*) => {
    #[allow(unused_unsafe)]
    $crate::raw::exec::vk_exec_to_vec(&|a, b| unsafe {
      $f($($arg,)* a, b)
    })
  }
}

pub fn vk_exec<F: Fn() -> vk_sys::Result>(f: &F) -> VulkanStatus {
  VulkanStatus::from_raw(f() as i32)
}

pub fn vk_exec_to_val<T, F: Fn(*mut T) -> vk_sys::Result>(f: F) -> VulkanResult<T> {
  use std::mem;
  unsafe {
    let mut item = mem::uninitialized();
    let status = VulkanStatus::from_raw(f(&mut item) as i32);
    if let Some(err_code) = status.error_opt() {
      return VulkanResult::error(err_code)
    }

    VulkanResult::from_nonerr_unchecked(status, item)
  }
}

pub fn vk_exec_to_vec<T, F: Fn(*mut u32, *mut T) -> vk_sys::Result>(f: &F) -> VulkanResult<Vec<T>> {
  use std::ptr;

  let mut num_items = 0;
  {
    let status = VulkanStatus::from_raw(
      f(&mut num_items, ptr::null::<T>() as *mut _) as i32);
    if let Some(err_code) = status.error_opt() {
      return VulkanResult::error(err_code)
    }
  }

  let mut output = Vec::with_capacity(num_items as usize);
  let status = VulkanStatus::from_raw(
    f(&mut num_items, output.as_mut_ptr()) as i32);
  if let Some(err_code) = status.error_opt() {
    return VulkanResult::error(err_code)
  }
  // Semi-Safe: Size known from contract of vulkan functions
  unsafe { output.set_len(num_items as usize) }

  VulkanResult::from_nonerr_unchecked(
    status,
    output)
}
