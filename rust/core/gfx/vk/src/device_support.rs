/**
 * This module provides utilities for discovering Vulkan device details and choosing the best device
 * and queue family combinations for rendering.
 *
 * It is currently optimized to find a dedicated (non-multifunction) queue family, and a graphics
 * queue family. It prefers multifunctional (compute + graphics) queue families, but current usage
 * does not require compute, so a graphics-only queue family is also considered acceptable.
 */

use std::ffi;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use std::mem;
#[macro_use]
use lite;
use vk_sys;

/**
 * Information acquired about a particular candidate rendering device, including supported
 * functionalities in the context of a particular renderable surface.
 */
pub struct CandidateDeviceDetails {
  pub physical_device: vk_sys::PhysicalDevice,
  pub physical_device_properties: vk_sys::PhysicalDeviceProperties,
  pub surface_capabilities: vk_sys::SurfaceCapabilitiesKHR,
  pub surface_formats: Vec<vk_sys::SurfaceFormatKHR>,
  pub swapchain_present_modes: Vec<vk_sys::PresentModeKHR>,
  pub memory_properties: vk_sys::PhysicalDeviceMemoryProperties,
}

impl CandidateDeviceDetails {
  /**
   * Iterates over the provided selection of devices, evaluating supported Vulkan capabilities
   * against the provided surface. This function does not consider queues -- a subsequent function
   * inspects supported queue families per device.
   */
  pub fn inspect_devices(
    instance: &lite::LInstance,
    physical_devices: &Vec<vk_sys::PhysicalDevice>,
    surface: &vk_sys::SurfaceKHR,
    required_extension_names: &Vec<&str>,
  ) -> lite::RawResult<Vec<CandidateDeviceDetails>> {
    let mut candidates = Vec::new();
    for physical_device in physical_devices.iter() {
      let physical_device_properties: vk_sys::PhysicalDeviceProperties =
        instance.get_physical_device_properties(*physical_device);
      let surface_capabilities =
        try!(instance.get_physical_device_surface_capabilities(*physical_device, surface));

      let surface_formats =
        try!(instance.list_physical_device_surface_formats(*physical_device, surface));
      if surface_formats.is_empty() {
        error!("Vulkan device is not suitable for rendering, as it has no renderable formats.");
        dump_device_details(&physical_device_properties);
        continue;
      }

      let swapchain_present_modes =
        try!(instance.list_physical_device_present_modes(*physical_device, surface));
      if swapchain_present_modes.is_empty() {
        error!(
          "Vulkan device is not suitable for rendering, as it has no usable presentation modes."
        );
        dump_device_details(&physical_device_properties);
        continue;
      }

      let available_extensions = try!(instance.list_device_extension_properties(*physical_device));
      let available_extension_names = available_extensions
        .iter()
        .map(|e| unsafe { CStr::from_ptr(e.extensionName.as_ptr()).to_str().unwrap() })
        .collect::<Vec<_>>();
      let missing_required_extensions = required_extension_names
        .iter()
        .filter(|e| !available_extension_names.contains(e))
        .collect::<Vec<_>>();

      if !missing_required_extensions.is_empty() {
        error!(
          "Vulkan device is not suitable for rendering, as it is missing essential extensions, \
           [{:?}]",
          missing_required_extensions
        );
        dump_device_details(&physical_device_properties);
        continue;
      }

      let memory_properties = instance.get_physical_device_memory_properties(*physical_device);

      candidates.push(CandidateDeviceDetails {
        physical_device: *physical_device,
        physical_device_properties: physical_device_properties,
        surface_capabilities: surface_capabilities,
        surface_formats: surface_formats,
        swapchain_present_modes: swapchain_present_modes,
        memory_properties: memory_properties,
      });
    }

    Ok(candidates)
  }
}

/**
 * Details acquired about a potential device's queue family support.
 *
 * The application is primarily concerned about rendering, but is also interested in dedicated
 * transfer queue families, if they're available.
 */
pub struct CandidateDeviceQueueDetails {
  pub physical_device: vk_sys::PhysicalDevice,
  pub score: u32,
  pub dedicated_transfer_queue_family_idx_opt: Option<u32>,
  pub gfx_queue_family_idx: u32,
}

impl CandidateDeviceQueueDetails {
  /**
   * Inspects all available physical devices, and produces a list of viable candidates.
   *
   * Candidates are rated according to their available queue families. Importantly, a
   * Graphics Queue is required for a device to become a candidate at all. Devices with dedicated
   * transfer queues are preferred over ones without, though the particulars of ranking are
   * semi-arbitrary.
   *
   * The current algorithm ranks queue families that also support compute higher, but compute isn't
   * currently used. It may produce suboptimal candidates if a device has graphics-only queue
   * family that is less performant or has less queues than its multifunction queue families.
   */
  pub fn inspect_queue_families(
    instance: &lite::LInstance,
    candidate_device_details: &Vec<CandidateDeviceDetails>,
    surface: &vk_sys::SurfaceKHR,
  ) -> lite::RawResult<Vec<CandidateDeviceQueueDetails>> {
    let mut candidates = Vec::new();
    for candidate_device in candidate_device_details.iter() {
      let mut queue_families_with_idx = instance
        .list_queue_family_properties(candidate_device.physical_device)
        .into_iter()
        .enumerate()
        .map(|(idx, queue_family_properties)| (idx as u32, queue_family_properties))
        .collect::<Vec<_>>();

      // Inspect queue families ordered by queue count (desc) to find the best queue families first
      queue_families_with_idx.sort_by_key(|&(_, ref qf)| qf.queueCount);
      queue_families_with_idx.reverse();

      let mut dedicated_transfer_queue_family_idx = None;
      let mut dedicated_graphics_queue_family_idx = None;
      let mut multifunctional_queue_family_idx = None;
      let mut backup_multifunctional_queue_family_idx = None;

      for &(ref queue_family_idx, ref queue_family_properties) in queue_families_with_idx.iter() {
        // Check that this queue family can interact with our surface
        let queue_family_supports_surface = try!(instance.get_physical_device_surface_support(
          candidate_device.physical_device,
          *queue_family_idx,
          surface
        )) == vk_sys::TRUE;

        let queue_family_supports_graphics = queue_family_supports_surface
          && (queue_family_properties.queueFlags & vk_sys::QUEUE_GRAPHICS_BIT) > 0;
        let queue_family_supports_compute = queue_family_supports_surface
          && (queue_family_properties.queueFlags & vk_sys::QUEUE_COMPUTE_BIT) > 0;

        // Transfer suitability is implied by graphics or compute suitability, though it can be
        // less performant than dedicated queues
        let queue_family_supports_transfer =
          (queue_family_properties.queueFlags & vk_sys::QUEUE_TRANSFER_BIT) > 0
            || queue_family_supports_graphics || queue_family_supports_compute;

        info!(
          "Observed pipeline support {} with graphics: {}, compute: {}, transfer {}",
          queue_family_properties.queueFlags,
          queue_family_supports_graphics,
          queue_family_supports_compute,
          queue_family_supports_transfer
        );

        // Check for "dedicated transfer queue family"
        // These are common on NVidia hardware, and we get much better transfer performance on them
        if queue_family_supports_transfer && !queue_family_supports_graphics
          && !queue_family_supports_compute
        {
          // Assign the designated transfer queue IFF there isn't already a (better) one
          if dedicated_transfer_queue_family_idx.is_none() {
            dedicated_transfer_queue_family_idx = Some(*queue_family_idx);
          }

          // Lets move on, this queue family is obviously not suitable as any other queue
          continue;
        }

        // Check for "multifunctional" queues first (this becomes a priority if we want compute in
        // game)
        if queue_family_supports_compute && queue_family_supports_graphics {
          if multifunctional_queue_family_idx.is_none() {
            multifunctional_queue_family_idx = Some(*queue_family_idx);
            // Move on to the next queue family
            continue;
          } else if backup_multifunctional_queue_family_idx.is_none() {
            multifunctional_queue_family_idx = Some(*queue_family_idx);
            // Move on to the next queue family
            continue;
          }
        }

        // If all else fails, this might still be a useful graphics queue
        if queue_family_supports_graphics {
          if dedicated_graphics_queue_family_idx.is_none() {
            dedicated_graphics_queue_family_idx = Some(*queue_family_idx);
            // Move on to the next queue family
            continue;
          }
        }
      }

      info!(
        "Observed QF index availablility: dtqf: {:?}, mqfi: {:?}, bmqfi: {:?}, dgqfi: {:?}",
        dedicated_transfer_queue_family_idx,
        multifunctional_queue_family_idx,
        backup_multifunctional_queue_family_idx,
        dedicated_graphics_queue_family_idx
      );

      // Figure out the best possible of queue families for this device
      let candidate_device_queue_details = {
        match (
          dedicated_transfer_queue_family_idx,
          multifunctional_queue_family_idx,
          backup_multifunctional_queue_family_idx,
          dedicated_graphics_queue_family_idx,
        ) {
          // 1) Ideal arrangement of queue families:
          //   (transfer + multifunctional)
          (Some(t_idx), Some(m_idx), _, _) => CandidateDeviceQueueDetails {
            physical_device: candidate_device.physical_device,
            score: 100,
            dedicated_transfer_queue_family_idx_opt: Some(t_idx),
            gfx_queue_family_idx: m_idx,
          },
          // 2) Suboptimal queue family arrangement
          //   (transfer + graphics)
          (Some(t_idx), None, None, Some(g_idx)) => CandidateDeviceQueueDetails {
            physical_device: candidate_device.physical_device,
            score: 80,
            dedicated_transfer_queue_family_idx_opt: Some(t_idx),
            gfx_queue_family_idx: g_idx,
          },
          // 3) Slightly more suboptimal queue family arrangement
          //   (multifunctional + multifunctional)
          (None, Some(m_idx_1), Some(m_idx_2), _) => {
            CandidateDeviceQueueDetails {
              physical_device: candidate_device.physical_device,
              score: 70,
              // Choose the "fewer queue" multifunctional as the transfer queue
              dedicated_transfer_queue_family_idx_opt: Some(m_idx_2),
              gfx_queue_family_idx: m_idx_1,
            }
          }
          // 4) Equally suboptimal queue family arrangement
          //   (multifunctional + graphics)
          (None, Some(m_idx), None, Some(g_idx)) => CandidateDeviceQueueDetails {
            physical_device: candidate_device.physical_device,
            score: 70,
            dedicated_transfer_queue_family_idx_opt: Some(m_idx),
            gfx_queue_family_idx: g_idx,
          },
          // 5) Quite bad, but acceptable queue family arrangement
          //   (multifunctional)
          (None, Some(m_idx), None, None) => CandidateDeviceQueueDetails {
            physical_device: candidate_device.physical_device,
            score: 50,
            dedicated_transfer_queue_family_idx_opt: None,
            gfx_queue_family_idx: m_idx,
          },
          // 5) Worst acceptable queue family arrangement
          //   (graphics)
          (None, None, None, Some(g_idx)) => CandidateDeviceQueueDetails {
            physical_device: candidate_device.physical_device,
            score: 30,
            dedicated_transfer_queue_family_idx_opt: None,
            gfx_queue_family_idx: g_idx,
          },
          _ => {
            error!(
              "Vulkan device is not suitable for rendering, as it has no queue families that can \
               render to the created surface"
            );
            dump_device_details(&candidate_device.physical_device_properties);
            // Move on to the next device
            continue;
          }
        }
      };

      candidates.push(candidate_device_queue_details);
    }

    // Sort desc by score
    candidates.sort_by_key(|c| c.score);
    candidates.reverse();

    Ok(candidates)
  }
}

/**
 * Logs an error containing details for a device.
 *
 * This function is intended to be used when a device is not viable for rendering for some reason,
 * and is expected to preceed a logging call with details about the lack of viability.
 */
fn dump_device_details(physical_device_properties: &vk_sys::PhysicalDeviceProperties) {
  error!(
    "  Vulkan Device Details: {{\"name\": \"{}\", \"apiVersion\": {}, \
     \"driverVersion\": {}}}",
    unsafe {
      ffi::CStr::from_ptr(physical_device_properties.deviceName.as_ptr())
        .to_str()
        .unwrap()
    },
    physical_device_properties.apiVersion,
    physical_device_properties.driverVersion
  );
}

/**
 * Complete details about a device that has been determined to be a viable rendering device,
 * including the queue families that are optimal for the device and application.
 */
pub struct SelectedPhysicalDeviceSpec {
  pub physical_device: vk_sys::PhysicalDevice,
  pub physical_device_properties: vk_sys::PhysicalDeviceProperties,
  pub surface_capabilities: vk_sys::SurfaceCapabilitiesKHR,
  pub surface_formats: Vec<vk_sys::SurfaceFormatKHR>,
  pub swapchain_present_modes: Vec<vk_sys::PresentModeKHR>,
  pub dedicated_transfer_queue_family_idx_opt: Option<u32>,
  pub memory_properties: vk_sys::PhysicalDeviceMemoryProperties,
  pub gfx_queue_family_idx: u32,
}

/**
 * Inspects a selection of candidate devices, along with their optimal device queue details.
 *
 * Not all provided device details are expected to have a supporting queue details. This is
 * interpreted by the function as meaning that the device is not supported, and consideration for
 * that device will be dropped.
 *
 * The function makes hard assertions about the provided arguments, so callers should understand
 * its implementation.
 */
pub fn select_best_device_and_queue(
  candidate_device_details: Vec<CandidateDeviceDetails>,
  mut candidate_device_queue_details: Vec<CandidateDeviceQueueDetails>,
) -> SelectedPhysicalDeviceSpec {
  assert!(candidate_device_details.len() > 0);
  assert!(candidate_device_queue_details.len() > 0);

  candidate_device_queue_details.sort_by_key(|c| c.score);
  candidate_device_queue_details.reverse();

  let selected_queue_details = candidate_device_queue_details.get(0).unwrap();
  let candidate_device_details = candidate_device_details
    .into_iter()
    .find(|d| d.physical_device == selected_queue_details.physical_device)
    .unwrap();

  assert!(candidate_device_details.surface_formats.len() > 0);
  assert!(candidate_device_details.swapchain_present_modes.len() > 0);

  SelectedPhysicalDeviceSpec {
    physical_device: candidate_device_details.physical_device,
    physical_device_properties: candidate_device_details.physical_device_properties,
    surface_capabilities: candidate_device_details.surface_capabilities,
    surface_formats: candidate_device_details.surface_formats,
    swapchain_present_modes: candidate_device_details.swapchain_present_modes,
    dedicated_transfer_queue_family_idx_opt: selected_queue_details
      .dedicated_transfer_queue_family_idx_opt,
    gfx_queue_family_idx: selected_queue_details.gfx_queue_family_idx,
    memory_properties: candidate_device_details.memory_properties,
  }
}

#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct LogicalDeviceCfg {
  pub gfx_queues: Vec<QueueCfg>,
  pub transfer_queues: Vec<QueueCfg>,
}

impl Default for LogicalDeviceCfg {
  fn default() -> LogicalDeviceCfg {
    LogicalDeviceCfg {
      gfx_queues: vec![QueueCfgBuilder::default().build().unwrap()],
      transfer_queues: vec![],
    }
  }
}

#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct QueueCfg {
  priority: f32,
}

impl Default for QueueCfg {
  fn default() -> QueueCfg {
    QueueCfg { priority: 0.5 }
  }
}

#[allow(non_snake_case)]
pub fn make_logical_device(
  instance: &lite::LInstance,
  logical_device_cfg: &LogicalDeviceCfg,
  selected_physical_device_spec: &SelectedPhysicalDeviceSpec,
  enabled_layers: &Vec<[i8; 256]>,
) -> lite::RawResult<lite::LDevice> {
  let queue_family_properties =
    instance.list_queue_family_properties(selected_physical_device_spec.physical_device);

  let mut device_queue_create_infos = Vec::new();

  // Generate init data for all gfx queues
  let gfx_queue_priorities = logical_device_cfg
    .gfx_queues
    .iter()
    .map(|queue_cfg| queue_cfg.priority.clone())
    .collect::<Vec<_>>();
  {
    let qf_idx = selected_physical_device_spec.gfx_queue_family_idx;
    let props = queue_family_properties.get(qf_idx as usize).unwrap();

    // Verify that we're not trying to configure more queues than we actually have
    assert!(props.queueCount >= logical_device_cfg.gfx_queues.len() as u32);

    device_queue_create_infos.push(vk_sys::DeviceQueueCreateInfo {
      sType: vk_sys::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
      pNext: ptr::null(),
      flags: 0,
      queueFamilyIndex: qf_idx,
      queueCount: gfx_queue_priorities.len() as u32,
      pQueuePriorities: gfx_queue_priorities.as_ptr(),
    });
  }

  // Generate init data for all transfer queues
  let transfer_queue_priorities = logical_device_cfg
    .transfer_queues
    .iter()
    .map(|queue_cfg| queue_cfg.priority.clone())
    .collect::<Vec<_>>();
  if !logical_device_cfg.transfer_queues.is_empty() {
    // Verify that we even have transfer queues
    // If this throws, you need to be checking device properties before trying to perform
    // configuration.
    assert!(
      selected_physical_device_spec
        .dedicated_transfer_queue_family_idx_opt
        .is_some()
    );

    let qf_idx = selected_physical_device_spec
      .dedicated_transfer_queue_family_idx_opt
      .unwrap();
    let props = queue_family_properties.get(qf_idx as usize).unwrap();

    // Verify that we're not trying to configure more queues than we actually have
    assert!(props.queueCount >= logical_device_cfg.transfer_queues.len() as u32);

    device_queue_create_infos.push(vk_sys::DeviceQueueCreateInfo {
      sType: vk_sys::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
      pNext: ptr::null(),
      flags: 0,
      queueFamilyIndex: qf_idx,
      queueCount: transfer_queue_priorities.len() as u32,
      pQueuePriorities: transfer_queue_priorities.as_ptr(),
    });
  }

  // TODO(acmcarther): Evaluate adding some of these to DeviceCfg
  let physical_device_features = vk_sys::PhysicalDeviceFeatures {
    robustBufferAccess: vk_sys::FALSE,
    fullDrawIndexUint32: vk_sys::FALSE,
    imageCubeArray: vk_sys::FALSE,
    independentBlend: vk_sys::FALSE,
    geometryShader: vk_sys::FALSE,
    tessellationShader: vk_sys::FALSE,
    sampleRateShading: vk_sys::FALSE,
    dualSrcBlend: vk_sys::FALSE,
    logicOp: vk_sys::FALSE,
    multiDrawIndirect: vk_sys::FALSE,
    drawIndirectFirstInstance: vk_sys::FALSE,
    depthClamp: vk_sys::FALSE,
    depthBiasClamp: vk_sys::FALSE,
    fillModeNonSolid: vk_sys::FALSE,
    depthBounds: vk_sys::FALSE,
    wideLines: vk_sys::FALSE,
    largePoints: vk_sys::FALSE,
    alphaToOne: vk_sys::FALSE,
    multiViewport: vk_sys::FALSE,
    samplerAnisotropy: vk_sys::TRUE, /* enabled for demo */
    textureCompressionETC2: vk_sys::FALSE,
    textureCompressionASTC_LDR: vk_sys::FALSE,
    textureCompressionBC: vk_sys::FALSE,
    occlusionQueryPrecise: vk_sys::FALSE,
    pipelineStatisticsQuery: vk_sys::FALSE,
    vertexPipelineStoresAndAtomics: vk_sys::FALSE,
    fragmentStoresAndAtomics: vk_sys::FALSE,
    shaderTessellationAndGeometryPointSize: vk_sys::FALSE,
    shaderImageGatherExtended: vk_sys::FALSE,
    shaderStorageImageExtendedFormats: vk_sys::FALSE,
    shaderStorageImageMultisample: vk_sys::FALSE,
    shaderStorageImageReadWithoutFormat: vk_sys::FALSE,
    shaderStorageImageWriteWithoutFormat: vk_sys::FALSE,
    shaderUniformBufferArrayDynamicIndexing: vk_sys::FALSE,
    shaderSampledImageArrayDynamicIndexing: vk_sys::FALSE,
    shaderStorageBufferArrayDynamicIndexing: vk_sys::FALSE,
    shaderStorageImageArrayDynamicIndexing: vk_sys::FALSE,
    shaderClipDistance: vk_sys::FALSE,
    shaderCullDistance: vk_sys::FALSE,
    shaderf3264: vk_sys::FALSE,
    shaderInt64: vk_sys::FALSE,
    shaderInt16: vk_sys::FALSE,
    shaderResourceResidency: vk_sys::FALSE,
    shaderResourceMinLod: vk_sys::FALSE,
    sparseBinding: vk_sys::FALSE,
    sparseResidencyBuffer: vk_sys::FALSE,
    sparseResidencyImage2D: vk_sys::FALSE,
    sparseResidencyImage3D: vk_sys::FALSE,
    sparseResidency2Samples: vk_sys::FALSE,
    sparseResidency4Samples: vk_sys::FALSE,
    sparseResidency8Samples: vk_sys::FALSE,
    sparseResidency16Samples: vk_sys::FALSE,
    sparseResidencyAliased: vk_sys::FALSE,
    variableMultisampleRate: vk_sys::FALSE,
    inheritedQueries: vk_sys::FALSE,
  };

  let ppEnabledLayerNames = enabled_layers
    .iter()
    .map(|i| i.as_ptr())
    .collect::<Vec<_>>();
  let enabled_extension_names = vec![CString::new("VK_KHR_swapchain").unwrap()];
  let ppEnabledExtensionNames = enabled_extension_names
    .iter()
    .map(|i| i.as_c_str().as_ptr())
    .collect::<Vec<_>>();
  let device_create_info = vk_sys::DeviceCreateInfo {
    sType: vk_sys::STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    queueCreateInfoCount: device_queue_create_infos.len() as u32,
    pQueueCreateInfos: device_queue_create_infos.as_ptr(),
    enabledLayerCount: ppEnabledLayerNames.len() as u32,
    ppEnabledLayerNames: ppEnabledLayerNames.as_ptr(),
    enabledExtensionCount: ppEnabledExtensionNames.len() as u32,
    ppEnabledExtensionNames: ppEnabledExtensionNames.as_ptr(),
    pEnabledFeatures: &physical_device_features as *const _,
  };

  instance.create_logical_device(
    selected_physical_device_spec.physical_device,
    &device_create_info,
  )
}

/** Information about a particular device queue family */
pub struct DeviceQueueSpec {
  pub device_queue_family_idx: u32,
  pub queues: Vec<vk_sys::Queue>,
  pub queue_family_properties: vk_sys::QueueFamilyProperties,
}

impl DeviceQueueSpec {
  pub fn num_queues(&self) -> usize {
    self.queues.len()
  }
}

/**
 * A manager that retains information about a devices's queue families.
 *
 * It may describe a device that has only a single graphics queue family, or a device that also is
 * has an accompanying dedicated transfer queue.
 */
pub struct ExperimentalDeviceQueueManager {
  gfx_qf_spec: DeviceQueueSpec,
  transfer_qf_spec_opt: Option<DeviceQueueSpec>,
}

impl ExperimentalDeviceQueueManager {
  /**
   * Collects information about the selected physical device, generating a complete queue manager.
   */
  pub fn from_selected_physical_device_spec(
    instance: &lite::LInstance,
    device: &lite::LDevice,
    selected_physical_device_spec: &SelectedPhysicalDeviceSpec,
  ) -> ExperimentalDeviceQueueManager {
    // Make sure provided queues aren't the same, as we do unsafe things later with memory related
    // to the indexes
    // Specifically, we're memswapping out elements of a vec based on the idx, and replacing it
    // with uninitialized memory.
    if let Some(transfer_queue_family_idx) =
      selected_physical_device_spec.dedicated_transfer_queue_family_idx_opt
    {
      assert!(transfer_queue_family_idx != selected_physical_device_spec.gfx_queue_family_idx)
    }

    let mut queue_family_properties =
      instance.list_queue_family_properties(selected_physical_device_spec.physical_device);
    let gfx_qf_spec = {
      let gfx_queue_family_idx = selected_physical_device_spec.gfx_queue_family_idx;
      let mut gfx_queue_family_properties = queue_family_properties
        .get_mut(gfx_queue_family_idx as usize)
        .unwrap();
      let gfx_queues = (0..gfx_queue_family_properties.queueCount)
        .map(|queue_idx| device.get_device_queue(gfx_queue_family_idx, queue_idx))
        .collect::<Vec<_>>();

      // Mem swap the interesting props out of the list
      let mut qf_props = unsafe { mem::uninitialized() };
      mem::swap(&mut qf_props, gfx_queue_family_properties);

      DeviceQueueSpec {
        device_queue_family_idx: gfx_queue_family_idx,
        queues: gfx_queues,
        queue_family_properties: qf_props,
      }
    };

    let dedicated_transfer_queue_family_idx_opt =
      selected_physical_device_spec.dedicated_transfer_queue_family_idx_opt;
    let transfer_qf_spec_opt =
      if let Some(transfer_queue_family_idx) = dedicated_transfer_queue_family_idx_opt {
        let transfer_queue_family_properties = queue_family_properties
          .get_mut(transfer_queue_family_idx as usize)
          .unwrap();
        let transfer_queues = (0..transfer_queue_family_properties.queueCount)
          .map(|queue_idx| device.get_device_queue(transfer_queue_family_idx, queue_idx))
          .collect::<Vec<_>>();

        // Mem swap the interesting props out of the list.
        // This could conceivably fail if this queue is the same as the transfer queue,
        // but we're asserting that it isn't at the beginning on the function
        let mut qf_props = unsafe { mem::uninitialized() };
        mem::swap(&mut qf_props, transfer_queue_family_properties);

        Some(DeviceQueueSpec {
          device_queue_family_idx: transfer_queue_family_idx,
          queues: transfer_queues,
          queue_family_properties: qf_props,
        })
      } else {
        None
      };

    ExperimentalDeviceQueueManager {
      gfx_qf_spec: gfx_qf_spec,
      transfer_qf_spec_opt: transfer_qf_spec_opt,
    }
  }

  /** Fetches the DeviceQueueSpec for the graphics queue family. */
  pub fn gfx(&self) -> &DeviceQueueSpec {
    &self.gfx_qf_spec
  }

  /**
   * Fetches the DeviceQueueSpec for the transfer queue family.
   *
   * This may be the same as the graphics queue family if the physical device does not have a
   * dedicated transfer queue family.
   */
  pub fn transfer(&self) -> &DeviceQueueSpec {
    self
      .transfer_qf_spec_opt
      .as_ref()
      .unwrap_or(&self.gfx_qf_spec)
  }

  /**
   * An encapsulation-breaking function that callers can use to optimize their pipelines for the
   * single queue family case. The lack of a dedicated transfer queue may allow callers to avoid
   * some synchronization costs, though transfers will have contention with the graphics pipeline.
   */
  pub fn has_dedicated_transfer_queue(&self) -> bool {
    self.transfer_qf_spec_opt.is_some()
  }
}
