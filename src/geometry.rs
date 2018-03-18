use sys::*;

pub struct Geometry {
    pub(crate) geometry_handle: RTCGeometry;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum GeometryType {
    Triangle = RTCGeometryType_RTC_GEOMETRY_TYPE_TRIANGLE;
    Quad = RTCGeometryType_RTC_GEOMETRY_TYPE_QUAD;
    // Subdivision = RTCGeometryType_RTC_GEOMETRY_TYPE_SUBDIVISION;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_LINEAR_CURVE;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_ROUND_BEZIER_CURVE;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_BEZIER_CURVE;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_ROUND_BSPLINE_CURVE;
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_BSPLINE_CURVE;
    // UserGeometry = RTCGeometryType_RTC_GEOMETRY_TYPE_USER;
    // Instance = RTCGeometryType_RTC_GEOMETRY_TYPE_INSTANCE;
}

impl Drop for Geometry {
    fn drop(&mut self) {
        unsafe { rtcReleaseGeometry(self.geometry_handle); }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum GeometryBuildQuality {
    Low = RTCBuildQuality_RTC_BUILD_QUALITY_LOW,
    Medium = RTCBuildQuality_RTC_BUILD_QUALITY_MEDIUM,
    High = RTCBuildQuality_RTC_BUILD_QUALITY_HIGH,
}