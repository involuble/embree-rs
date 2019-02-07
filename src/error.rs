use std::fmt;

use sys::*;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    None = RTCError_RTC_ERROR_NONE,
    Unknown = RTCError_RTC_ERROR_UNKNOWN,
    InvalidArgument = RTCError_RTC_ERROR_INVALID_ARGUMENT,
    InvalidOperation = RTCError_RTC_ERROR_INVALID_OPERATION,
    OutOfMemory = RTCError_RTC_ERROR_OUT_OF_MEMORY,
    UnsupportedCPU = RTCError_RTC_ERROR_UNSUPPORTED_CPU,
    Cancelled = RTCError_RTC_ERROR_CANCELLED,
}

impl ErrorKind {
    pub fn from_i32(err: i32) -> Self {
        #[allow(non_upper_case_globals)]
        match err {
            RTCError_RTC_ERROR_NONE => ErrorKind::None,
            RTCError_RTC_ERROR_UNKNOWN => ErrorKind::Unknown,
            RTCError_RTC_ERROR_INVALID_ARGUMENT => ErrorKind::InvalidArgument,
            RTCError_RTC_ERROR_INVALID_OPERATION => ErrorKind::InvalidOperation,
            RTCError_RTC_ERROR_OUT_OF_MEMORY => ErrorKind::OutOfMemory,
            RTCError_RTC_ERROR_UNSUPPORTED_CPU => ErrorKind::UnsupportedCPU,
            RTCError_RTC_ERROR_CANCELLED => ErrorKind::Cancelled,
            // RTCError_RTC_ERROR_NONE => panic!("cannot have a none error"),
            _ => ErrorKind::Unknown,
        }
    }
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::None => "none",
            ErrorKind::InvalidArgument => "invalid argument",
            ErrorKind::InvalidOperation => "invalid operation",
            ErrorKind::OutOfMemory => "out of memory",
            ErrorKind::UnsupportedCPU => "unsupported CPU",
            ErrorKind::Cancelled => "cancelled",
            _ => "unknown",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}