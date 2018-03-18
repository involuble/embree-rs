use std::ptr;

use sys::*;

pub struct Device {
    pub(crate) device_handle: RTCDevice;
}

impl Device {
    pub fn new() -> Self {
        let device = unsafe { rtcNewDevice(ptr::null()) };
        Device { device_handle: device }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Clone for Device {
    fn clone(&self) -> Device {
        unsafe { rtcRetainDevice(self.device) };
        Device { self.device_handle }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { rtcReleaseDevice(self.device_handle); }
    }
}
