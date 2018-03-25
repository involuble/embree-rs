use std::os::raw::c_void;
use std::u32;

use sys::*;

use cgmath;

use device::Device;
use scene::BuildQuality;

pub trait Geometry {
    fn get_handle(&self) -> &GeometryHandle;

    fn set_build_quality(&self, quality: BuildQuality) {
        let handle = self.get_handle();
        unsafe { rtcSetGeometryBuildQuality(handle.ptr, quality.into()); }
    }

    fn set_transform(&self, transform: &cgmath::Matrix4<f32>) {
        let handle = self.get_handle();
        let xfm: &[f32; 16] = transform.as_ref();
        unsafe {
            rtcSetGeometryTransform(handle.ptr, 0,
                RTCFormat_RTC_FORMAT_FLOAT4X4_COLUMN_MAJOR,
                xfm.as_ptr() as *const c_void);
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ID {
    id: u32,
}

pub const INVALID_ID: u32 = u32::MAX;

impl ID {
    pub(crate) fn new(id: u32) -> Self {
        ID { id: id }
    }

    pub fn invalid() -> Self {
        ID {
            id: INVALID_ID,
        }
    }

    pub fn is_invalid(&self) -> bool {
        self.id == INVALID_ID
    }

    pub fn unwrap(&self) -> u32 {
        assert!(!self.is_invalid());
        self.id
    }
}

#[repr(C)]
pub struct GeometryHandle {
    pub(crate) ptr: RTCGeometry,
}

impl GeometryHandle {
    pub(crate) fn new(device: &Device, geom_type: GeometryType) -> Self {
        let h = unsafe { rtcNewGeometry(device.ptr, geom_type.into()) };
        GeometryHandle { ptr: h }
    }
}

impl Clone for GeometryHandle {
    fn clone(&self) -> GeometryHandle {
        unsafe { rtcRetainGeometry(self.ptr) }
        GeometryHandle { ptr: self.ptr }
    }
}

impl Drop for GeometryHandle {
    fn drop(&mut self) {
        unsafe { rtcReleaseGeometry(self.ptr) }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GeometryType {
    Triangle = RTCGeometryType_RTC_GEOMETRY_TYPE_TRIANGLE,
    Quad = RTCGeometryType_RTC_GEOMETRY_TYPE_QUAD,
    // Subdivision = RTCGeometryType_RTC_GEOMETRY_TYPE_SUBDIVISION;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_LINEAR_CURVE;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_ROUND_BEZIER_CURVE;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_BEZIER_CURVE;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_ROUND_BSPLINE_CURVE;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_BSPLINE_CURVE;
    // User = RTCGeometryType_RTC_GEOMETRY_TYPE_USER;
    // Instance = RTCGeometryType_RTC_GEOMETRY_TYPE_INSTANCE;
}

into_primitive!(GeometryType, i32);