use std::mem;
use std::os::raw::c_void;
use std::u32;

use sys::*;

use device::Device;
use common::*;
use polygon_geometry::*;
use user_geometry::*;
use point_geometry::*;

pub struct Geometry {
    pub(crate) internal: GeometryInternal,
}

pub(crate) enum GeometryInternal {
    Triangles(TriangleMesh),
    Quads(QuadMesh),
    Spheres(SphereGeometry),
    Discs(DiscGeometry),
    User(ErasedUserGeometry),
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
            GeometryInternal::Quads(ref q) => &q.handle,
            GeometryInternal::Spheres(ref s) => &s.handle,
            GeometryInternal::Discs(ref d) => &d.handle,
            GeometryInternal::User(ref u) => &u.handle,
        }
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

    // pub(crate) fn set_instance_transform(&self, transform: &cgmath::Matrix4<f32>) {
    //     unsafe {
    //         rtcSetGeometryTransform(self.ptr, 0,
    //             cgmath::Matrix4::FORMAT.into(),
    //             transform.as_ptr() as *const c_void);
    //     }
    // }

    pub(crate) fn bind_shared_geometry_buffer<T>(&self, data: &mut Vec<T>, buf_type: BufferType, format: Format, slot: u32, byte_offset: usize) {
        // TODO: This can reallocate and isn't safe
        if buf_type == BufferType::Vertex || buf_type == BufferType::VertexAttribute {
            if mem::size_of::<T>() == 4 {
                data.reserve(3);
            } else if mem::size_of::<T>() % 16 == 0 {
                // Do nothing
            } else {
                data.reserve(1);
            }
        }
        debug_assert!(byte_offset % 4 == 0, "offset must be 4 byte aligned");
        debug_assert!(mem::size_of::<T>() % 4 == 0, "stride must be 4 byte aligned");
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

unsafe impl Send for GeometryHandle {}
unsafe impl Sync for GeometryHandle {}

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
    // Grid = RTCGeometryType_RTC_GEOMETRY_TYPE_GRID,
    // Subdivision = RTCGeometryType_RTC_GEOMETRY_TYPE_SUBDIVISION,
    //  TODO: various curve types...
    Sphere = RTCGeometryType_RTC_GEOMETRY_TYPE_SPHERE_POINT,
    // RayFacingDisc = RTCGeometryType_RTC_GEOMETRY_TYPE_DISC_POINT,
    Disc = RTCGeometryType_RTC_GEOMETRY_TYPE_ORIENTED_DISC_POINT,
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
    Normal = RTCBufferType_RTC_BUFFER_TYPE_NORMAL,
    // Tangent = RTCBufferType_RTC_BUFFER_TYPE_TANGENT,
    // Grid = RTCBufferType_RTC_BUFFER_TYPE_GRID,
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