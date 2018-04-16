use std::mem;
use std::os::raw::c_void;
use std::u32;

use sys::*;

use cgmath;

use device::Device;
use scene::BuildQuality;
use type_format::*;
use polygon_geometry::*;

pub struct Geometry {
    pub(crate) internal: GeometryInternal,
}

pub(crate) trait GeometryData {
    fn set_geom_id(&mut self, id: GeomID);
    fn handle(&self) -> &GeometryHandle;
}

pub(crate) enum GeometryInternal {
    Triangles(TriangleMesh),
    Other(Box<GeometryData>),
}

impl Geometry {
    pub(crate) fn new(geom: GeometryInternal) -> Self {
        Geometry {
            internal: geom,
        }
    }

    pub fn handle(&self) -> &GeometryHandle {
        match self.internal {
            GeometryInternal::Triangles(ref t) => &t.handle,
            GeometryInternal::Other(ref data) => data.handle(),
        }
    }

    pub(crate) fn set_geom_id(&mut self, id: GeomID) {
        match self.internal {
            GeometryInternal::Triangles(_) => (),
            GeometryInternal::Other(ref mut data) => data.set_geom_id(id),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeomID {
    id: u32,
}

pub const INVALID_ID: u32 = u32::MAX;

impl GeomID {
    pub(crate) fn new(id: u32) -> Self {
        GeomID { id: id }
    }

    pub fn invalid() -> Self {
        GeomID {
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

    pub(crate) fn as_ptr(&self) -> RTCGeometry {
        self.ptr
    }

    pub(crate) fn set_build_quality(&self, quality: BuildQuality) {
        unsafe { rtcSetGeometryBuildQuality(self.ptr, quality.into()); }
    }

    #[allow(dead_code)]
    pub(crate) fn set_instance_transform(&self, transform: &cgmath::Matrix4<f32>) {
        unsafe {
            rtcSetGeometryTransform(self.ptr, 0,
                cgmath::Matrix4::FORMAT.into(),
                transform.as_ptr() as *const c_void);
        }
    }

    pub(crate) fn bind_shared_geometry_buffer<T>(&self, data: &mut Vec<T>, buf_type: BufferType, format: Format, slot: u32, byte_offset: usize) {
        if buf_type == BufferType::Vertex || buf_type == BufferType::VertexAttribute {
            if mem::size_of::<T>() == 4 {
                data.reserve(3);
            } else if mem::size_of::<T>() % 16 == 0 {
                // Do nothing
            } else {
                data.reserve(1);
            }
        }
        unsafe {
            rtcSetSharedGeometryBuffer(self.ptr,
                buf_type.into(),
                slot,
                format.into(),
                data.as_ptr() as *const c_void,
                byte_offset,
                mem::size_of::<T>(),
                data.len());
        }
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
    // Subdivision = RTCGeometryType_RTC_GEOMETRY_TYPE_SUBDIVISION,
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_LINEAR_CURVE,
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_ROUND_BEZIER_CURVE,
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_BEZIER_CURVE,
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_ROUND_BSPLINE_CURVE,
    // Curve = RTCGeometryType_RTC_GEOMETRY_TYPE_FLAT_BSPLINE_CURVE,
    User = RTCGeometryType_RTC_GEOMETRY_TYPE_USER,
    // Instance = RTCGeometryType_RTC_GEOMETRY_TYPE_INSTANCE,
}

into_primitive!(GeometryType, i32);

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum BufferType {
    Index  = RTCBufferType_RTC_BUFFER_TYPE_INDEX,
    Vertex = RTCBufferType_RTC_BUFFER_TYPE_VERTEX,
    VertexAttribute = RTCBufferType_RTC_BUFFER_TYPE_VERTEX_ATTRIBUTE,
    // Face = RTCBufferType_RTC_BUFFER_TYPE_FACE,
    // Level = RTCBufferType_RTC_BUFFER_TYPE_LEVEL,
    // EdgeCreaseIndex = RTCBufferType_RTC_BUFFER_TYPE_EDGE_CREASE_INDEX,
    // EdgeCreaseWeight = RTCBufferType_RTC_BUFFER_TYPE_EDGE_CREASE_WEIGHT,
    // VertexCreaseIndex = RTCBufferType_RTC_BUFFER_TYPE_VERTEX_CREASE_INDEX,
    // VertexCreaseWeight = RTCBufferType_RTC_BUFFER_TYPE_VERTEX_CREASE_WEIGHT,
    // Hole = RTCBufferType_RTC_BUFFER_TYPE_HOLE,
    // Flags = RTCBufferType_RTC_BUFFER_TYPE_FLAGS,
}

into_primitive!(BufferType, i32);