use std::ptr;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use std::sync::atomic::{AtomicIsize, Ordering};

use sys::*;

use error::*;

pub fn set_flush_to_zero_mode() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::{_mm_setcsr, _mm_getcsr, _MM_FLUSH_ZERO_ON};
        #[cfg(target_arch = "x86")]
        use std::arch::x86::{_mm_setcsr, _mm_getcsr, _MM_FLUSH_ZERO_ON};

        // Note: this flag requires the processor to support SSE3
        const _MM_DENORMALS_ZERO_ON: u32 = 0x0040;

        _mm_setcsr(_mm_getcsr() | _MM_FLUSH_ZERO_ON | _MM_DENORMALS_ZERO_ON);
    }
}

pub struct Device {
    pub(crate) ptr: RTCDevice,
}

impl Device {
    pub fn new() -> Self {
        let device = unsafe { rtcNewDevice(ptr::null()) };
        let err = unsafe { rtcGetDeviceError(ptr::null_mut()) };
        if err != RTC_ERROR_NONE {
            panic!("Embree device creation failed");
        }

        unsafe {
            rtcSetDeviceErrorFunction(device, Some(error_callback), ptr::null_mut());
            rtcSetDeviceMemoryMonitorFunction(device, Some(memory_monitor_callback), ptr::null_mut());
        }
        Device { ptr: device }
    }
    
    pub fn last_error(&self) -> Result<(), ErrorKind> {
        let err = unsafe { rtcGetDeviceError(self.ptr) };
        match err {
            RTC_ERROR_NONE => Ok(()),
            _ => Err(ErrorKind::from_i32(err)),
        }
    }
}

unsafe extern "C" fn error_callback(_user_ptr: *mut c_void, error: i32, str: *const c_char) {
    debug_assert!(!str.is_null());
    let msg = CStr::from_ptr(str);
    error!("Embree error {}: {}", ErrorKind::from_i32(error), msg.to_string_lossy());
}

// This breaks when there's more than one device. Could store Arc<AtomicIsize> in Device
static DEVICE_MEMORY_USAGE: AtomicIsize = AtomicIsize::new(0);

/// ptr: The provided payload when registering the callback
/// bytes: Number of bytes allocated or deallocated
/// post: Whether this callback was invoked before or after the (de)allocation took place
unsafe extern "C" fn memory_monitor_callback(_ptr: *mut c_void, bytes: isize, _post: bool) -> bool {
    DEVICE_MEMORY_USAGE.fetch_add(bytes, Ordering::SeqCst);
    true
}

impl Clone for Device {
    fn clone(&self) -> Device {
        unsafe { rtcRetainDevice(self.ptr) };
        Device { ptr: self.ptr }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { rtcReleaseDevice(self.ptr) };
    }
}
