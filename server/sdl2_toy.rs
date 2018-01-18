extern crate sdl2;
extern crate dylib;
extern crate vk_sys as vk;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::ffi::CString;
use std::ffi::CStr;
use std::path::Path;
use std::path::PathBuf;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

fn main() {
  println!("1");
  let sdl_context = sdl2::init().unwrap();
  println!("{:?}", sdl2::video::drivers().collect::<Vec<_>>());
  println!("{:?}", sdl2::audio::drivers().collect::<Vec<_>>());
  println!("{:?}", sdl2::render::drivers().collect::<Vec<_>>());
  println!("2");
  let video_subsystem = sdl_context.video().unwrap();
  println!("3");

  vulkan();

  let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
      .position_centered()
      .vulkan()
      .build()
      .unwrap();

  println!("3");

  let mut canvas = window.into_canvas().build().unwrap();

  canvas.set_draw_color(Color::RGB(0, 255, 255));
  canvas.clear();
  canvas.present();
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut i = 0;
  'running: loop {
      i = (i + 1) % 255;
      canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
      canvas.clear();
      for event in event_pump.poll_iter() {
          match event {
              Event::Quit {..} |
              Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                  break 'running
              },
              _ => {}
          }
      }
      // The rest of the game loop goes here...

      canvas.present();
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}

fn vulkan() {
  let lib_path = PathBuf::from("libvulkan.so.1");
  let lib = dylib::DynamicLibrary::open(Some(lib_path.as_path())).unwrap();

  let entry_points = unsafe {
    vk::EntryPoints::load(|symbol_name| lib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void)
  };

  /*
  let instance_pointers = unsafe {
    vk::InstancePointers::load(|symbol_name| lib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void)
  };*/

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

  for extension in extensions.iter() {
    let extension_name = String::from_utf8(extension.extensionName.to_vec().into_iter().map(|i| i as u8).collect()).unwrap();
    println!("got some extension {}, {}", extension_name, extension.specVersion);
  }

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

  for layer in layers.iter() {
    let layer_name = String::from_utf8(layer.layerName.to_vec().into_iter().map(|i| i as u8).collect()).unwrap();
    let layer_description = String::from_utf8(layer.description.to_vec().into_iter().map(|i| i as u8).collect()).unwrap();
    println!("got some layer: {}, {}, {}, {}", layer_name, layer.specVersion, layer.implementationVersion, layer_description);
  }


  let vk_application_info = vk::ApplicationInfo {
    sType: vk::STRUCTURE_TYPE_APPLICATION_INFO,
    pNext: ptr::null(),
    pApplicationName: CString::new("sdl2_toy").unwrap().as_ptr(),
    applicationVersion: 1,
    pEngineName: CString::new("No Engine").unwrap().as_ptr(),
    engineVersion: 1,
    apiVersion: 0
  };

  let enabled_layers = unsafe {
    layers.into_iter()
      .filter(|l| CStr::from_ptr(l.layerName.as_ptr()).to_str().unwrap().contains("validation"))
      .map(|l| l.layerName)
      .collect::<Vec<_>>()
  };
  let enabled_extensions = unsafe {
    extensions.into_iter()
      .filter(|e| CStr::from_ptr(e.extensionName.as_ptr()).to_str().unwrap().contains("debug"))
      .map(|e| e.extensionName)
      .collect::<Vec<_>>()
  };

  unsafe {
    for layer in enabled_layers.iter() {
      println!("layer enabled: {}", CStr::from_ptr(layer.as_ptr()).to_str().unwrap())
    }
  }

  unsafe {
    for extension in enabled_extensions.iter() {
      println!("extension enabled: {}", CStr::from_ptr(extension.as_ptr()).to_str().unwrap())
    }
  }

  let vk_instance_create_info = vk::InstanceCreateInfo {
    sType: vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
    pApplicationInfo: &vk_application_info as *const _,
    flags: 0,
    pNext: ptr::null(),
    enabledLayerCount: enabled_layers.len() as u32,
    ppEnabledLayerNames: enabled_layers.iter().map(|i| i.as_ptr()).collect::<Vec<_>>().as_ptr(),
    enabledExtensionCount: num_extensions,
    ppEnabledExtensionNames: enabled_extensions.iter().map(|i| i.as_ptr()).collect::<Vec<_>>().as_ptr(),
  };

  let mut instance = 0;
  unsafe {
    let result = entry_points.CreateInstance(
      &vk_instance_create_info, ptr::null(), &mut instance);

    if result != vk::SUCCESS {
      panic!("failed to create vulkan instance with {}", vk_result_to_human(result as i32));
    }
  }

  let instance_pointers = unsafe {
    vk::InstancePointers::load(|symbol_name| lib.symbol::<*const std::os::raw::c_void>(symbol_name.to_str().unwrap()).unwrap() as *const std::os::raw::c_void)
  };

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

  unsafe {
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
