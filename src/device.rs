use std::ptr;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;

use sys::*;

use error::*;

pub struct Device {
    pub(crate) ptr: RTCDevice,
}

impl Device {
    pub fn new() -> Self {
        let device = unsafe { rtcNewDevice(ptr::null()) };
        let user = ptr::null_mut();
        unsafe { rtcSetDeviceErrorFunction(device, Some(error_callback), user); }
        Device { ptr: device }
    }
}

unsafe extern "C" fn error_callback(_: *mut c_void, error: i32, str: *const c_char) {
    let msg = CStr::from_ptr(str);
    error!("Embree error {:?}: {:?}", ErrorKind::from_i32(error), msg);
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
