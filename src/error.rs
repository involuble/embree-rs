use std::fmt;

use sys::*;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    Unknown = RTC_ERROR_UNKNOWN,
    InvalidArgument = RTC_ERROR_INVALID_ARGUMENT,
    InvalidOperation = RTC_ERROR_INVALID_OPERATION,
    OutOfMemory = RTC_ERROR_OUT_OF_MEMORY,
    UnsupportedCPU = RTC_ERROR_UNSUPPORTED_CPU,
    Cancelled = RTC_ERROR_CANCELLED,
}

impl ErrorKind {
    pub fn from_i32(err: i32) -> Self {
        match err {
            RTC_ERROR_UNKNOWN => ErrorKind::Unknown,
            RTC_ERROR_INVALID_ARGUMENT => ErrorKind::InvalidArgument,
            RTC_ERROR_INVALID_OPERATION => ErrorKind::InvalidOperation,
            RTC_ERROR_OUT_OF_MEMORY => ErrorKind::OutOfMemory,
            RTC_ERROR_UNSUPPORTED_CPU => ErrorKind::UnsupportedCPU,
            RTC_ERROR_CANCELLED => ErrorKind::Cancelled,
            RTC_ERROR_NONE => panic!("cannot have a none error"),
            _ => ErrorKind::Unknown,
        }
    }
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::Unknown => "unknown",
            ErrorKind::InvalidArgument => "invalid argument",
            ErrorKind::InvalidOperation => "invalid operation",
            ErrorKind::OutOfMemory => "out of memory",
            ErrorKind::UnsupportedCPU => "unsupported CPU",
            ErrorKind::Cancelled => "cancelled",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}