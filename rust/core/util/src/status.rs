#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ErrorCode {
  Cancelled,
  Unknown,
  InvalidArgument,
  DeadlineExceeded,
  NotFound,
  AlreadyExists,
  PermissionDenied,
  ResourceExhausted,
  FailedPrecondition,
  Aborted,
  OutOfRange,
  Unimplemented,
  Internal,
  Unavailable,
  DataLoss,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Error {
  pub code: ErrorCode,
  pub message: String,
}

pub type EResult<T> = Result<T, Error>;

impl Error {
  pub fn cancelled<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::Cancelled,
      message: message.into()
    }
  }
  pub fn unknown<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::Unknown,
      message: message.into()
    }
  }
  pub fn invalid_argument<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::InvalidArgument,
      message: message.into()
    }
  }
  pub fn deadline_exceeded<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::DeadlineExceeded,
      message: message.into()
    }
  }
  pub fn not_found<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::NotFound,
      message: message.into()
    }
  }
  pub fn already_exists<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::AlreadyExists,
      message: message.into()
    }
  }
  pub fn permission_denied<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::PermissionDenied,
      message: message.into()
    }
  }
  pub fn resource_exhausted<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::ResourceExhausted,
      message: message.into()
    }
  }
  pub fn failed_precondition<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::FailedPrecondition,
      message: message.into()
    }
  }
  pub fn aborted<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::Aborted,
      message: message.into()
    }
  }
  pub fn out_of_range<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::OutOfRange,
      message: message.into()
    }
  }
  pub fn unimplemented<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::Unimplemented,
      message: message.into()
    }
  }
  pub fn internal<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::Internal,
      message: message.into()
    }
  }
  pub fn unavailable<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::Unavailable,
      message: message.into()
    }
  }
  pub fn data_loss<T: Into<String>>(message: T) -> Error {
    Error {
      code: ErrorCode::DataLoss,
      message: message.into()
    }
  }
}
