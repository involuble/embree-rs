#![allow(non_upper_case_globals)]

use sys::*;

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
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
        match err {
            RTCError_RTC_ERROR_NONE => ErrorKind::None,
            RTCError_RTC_ERROR_UNKNOWN => ErrorKind::Unknown,
            RTCError_RTC_ERROR_INVALID_ARGUMENT => ErrorKind::InvalidArgument,
            RTCError_RTC_ERROR_INVALID_OPERATION => ErrorKind::InvalidOperation,
            RTCError_RTC_ERROR_OUT_OF_MEMORY => ErrorKind::OutOfMemory,
            RTCError_RTC_ERROR_UNSUPPORTED_CPU => ErrorKind::UnsupportedCPU,
            RTCError_RTC_ERROR_CANCELLED => ErrorKind::Cancelled,
            _ => ErrorKind::Unknown,
        }
    }
}