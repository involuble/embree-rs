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

into_primitive!(ErrorKind, i32);

impl ErrorKind {
    pub fn from_i32(n: i32) -> Self {
        // TODO: Fix this
        unsafe { ::std::mem::transmute::<i32, Self>(n) }
    }
}