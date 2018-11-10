pub struct VulkanResult<T> {
  value_opt: Option<T>,
  status: VulkanStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VulkanStatus {
  Success,
  Warning(WarnCode),
  Error(ErrorCode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarnCode {
  NotReady,
  Timeout,
  EventSet,
  EventReset,
  Incomplete,
  Other(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
  OutOfHostDeviceMemory,
  OutOfDeviceMemory,
  InitializationFailed,
  DeviceLost,
  MemoryMapFailed,
  LayerNotPresent,
  ExtensionNotPresent,
  FeatureNotPresent,
  IncompatibleDriver,
  TooManyObjects,
  FormatNotSupported,
  FragmentedPool,
  Other(i32),
}

impl <T> VulkanResult<T> {
  pub fn from_nonerr_unchecked(status: VulkanStatus, value: T) -> VulkanResult<T> {
    debug_assert!(!status.is_error());

    VulkanResult {
      value_opt: Some(value),
      status: status,
    }
  }

  pub fn success(val: T) -> VulkanResult<T> {
    VulkanResult {
      value_opt: Some(val),
      status: VulkanStatus::success(),
    }
  }

  pub fn warning(val: T, code: WarnCode) -> VulkanResult<T> {
    VulkanResult {
      value_opt: Some(val),
      status: VulkanStatus::warning(code),
    }
  }

  pub fn error(code: ErrorCode) -> VulkanResult<T> {
    VulkanResult {
      value_opt: None,
      status: VulkanStatus::error(code),
    }
  }


  pub fn error_opt(&self) -> Option<ErrorCode> {
    self.status.error_opt()
  }

  pub fn value_opt(&self) -> Option<&T> {
    self.value_opt.as_ref()
  }

  pub fn status(&self) -> VulkanStatus {
    self.status.clone()
  }

  pub fn is_ok(&self) -> bool {
    self.status.is_ok()
  }
}

impl VulkanStatus {
  pub fn success() -> VulkanStatus {
    VulkanStatus::Success
  }

  pub fn warning(code: WarnCode) -> VulkanStatus {
    VulkanStatus::Warning(code)
  }

  pub fn error(code: ErrorCode) -> VulkanStatus {
    VulkanStatus::Error(code)
  }

  pub fn is_ok(&self) -> bool {
    match self {
      &VulkanStatus::Success => true,
      &VulkanStatus::Warning(_) => true,
      &VulkanStatus::Error(_) => false,
    }
  }

  pub fn is_success(&self) -> bool {
    self == &VulkanStatus::Success
  }

  pub fn is_warning(&self) -> bool {
    match self {
      &VulkanStatus::Warning(_) => true,
      _ => false,
    }
  }

  pub fn is_error(&self) -> bool {
    match self {
      &VulkanStatus::Error(_) => true,
      _ => false,
    }
  }

  pub fn error_opt(&self) -> Option<ErrorCode> {
    match self {
      &VulkanStatus::Error(code) => Some(code),
      _ => None,
    }
  }

  pub fn from_raw(code: i32) -> VulkanStatus {
    if code == 0 {
      return VulkanStatus::success();
    }

    if code > 0 {
      VulkanStatus::warning(WarnCode::from_raw(code))
    } else {
      VulkanStatus::error(ErrorCode::from_raw(code))
    }
  }

  pub fn raw_code(&self) -> i32 {
    match *self {
      VulkanStatus::Success => 0,
      VulkanStatus::Warning(ref code) => code.raw_code(),
      VulkanStatus::Error(ref code) => code.raw_code(),

    }
  }
}

impl WarnCode {
  pub fn from_raw(code: i32) -> WarnCode {
    debug_assert!(code > 0);

    match code {
      1 => WarnCode::NotReady,
      2 => WarnCode::Timeout,
      3 => WarnCode::EventSet,
      4 => WarnCode::EventReset,
      5 => WarnCode::Incomplete,
      _ => WarnCode::Other(code),
    }
  }

  pub fn raw_code(&self) -> i32 {
    match *self {
      WarnCode::NotReady => 1,
      WarnCode::Timeout => 2,
      WarnCode::EventSet => 3,
      WarnCode::EventReset => 4,
      WarnCode::Incomplete => 5,
      WarnCode::Other(code) => code,
    }
  }
}

impl ErrorCode {
  pub fn from_raw(code: i32) -> ErrorCode {
    debug_assert!(code < 0);

    match code {
      -1 => ErrorCode::OutOfHostDeviceMemory,
      -2 => ErrorCode::OutOfDeviceMemory,
      -3 => ErrorCode::InitializationFailed,
      -4 => ErrorCode::DeviceLost,
      -5 => ErrorCode::MemoryMapFailed,
      -6 => ErrorCode::LayerNotPresent,
      -7 => ErrorCode::ExtensionNotPresent,
      -8 => ErrorCode::FeatureNotPresent,
      -9 => ErrorCode::IncompatibleDriver,
      -10 => ErrorCode::TooManyObjects,
      -11 => ErrorCode::FormatNotSupported,
      -12 => ErrorCode::FragmentedPool,
      _ => ErrorCode::Other(code),
    }
  }

  pub fn raw_code(&self) -> i32 {
    match *self {
      ErrorCode::OutOfHostDeviceMemory => -1,
      ErrorCode::OutOfDeviceMemory => -2,
      ErrorCode::InitializationFailed => -3,
      ErrorCode::DeviceLost => -4,
      ErrorCode::MemoryMapFailed => -5,
      ErrorCode::LayerNotPresent => -6,
      ErrorCode::ExtensionNotPresent => -7,
      ErrorCode::FeatureNotPresent => -8,
      ErrorCode::IncompatibleDriver => -9,
      ErrorCode::TooManyObjects => -10,
      ErrorCode::FormatNotSupported => -11,
      ErrorCode::FragmentedPool => -12,
      ErrorCode::Other(code) => code,
    }
  }
}
