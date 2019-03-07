use std::mem;
use std::ffi::c_void;
use std::u32;

use sys::*;

use device::Device;
use common::*;

pub trait Geometry: Send + Sync + 'static {
    fn handle(&self) -> &GeometryHandle;
    fn handle_mut(&mut self) -> &mut GeometryHandle;

    fn set_build_quality(&mut self, quality: BuildQuality) {
        self.handle_mut().set_build_quality(quality);
    }

    fn set_geom_id(&mut self, _id: u32) {}

    fn bind_buffers(&mut self);
}

#[repr(C)]
pub struct GeometryHandle {
    pub(crate) ptr: RTCGeometry,
}

impl GeometryHandle {
    pub(crate) fn new(device: &Device, geom_type: GeometryType) -> Self {
        let ptr = unsafe { rtcNewGeometry(device.ptr, geom_type.into()) };
        GeometryHandle { ptr }
    }

    pub(crate) fn as_raw_ptr(&self) -> RTCGeometry {
        self.ptr
    }

    pub(crate) fn set_build_quality(&mut self, quality: BuildQuality) {
        unsafe { rtcSetGeometryBuildQuality(self.ptr, quality.into()); }
    }

    // pub(crate) fn set_instance_transform(&mut self, transform: &cgmath::Matrix4<f32>) {
    //     unsafe {
    //         rtcSetGeometryTransform(self.ptr, 0,
    //             cgmath::Matrix4::FORMAT.into(),
    //             transform.as_ptr() as *const c_void);
    //     }
    // }

    /// slot: slot is used as the time_step for a vertex buffer and the slot for a vertex attribute
    pub(crate) unsafe fn bind_shared_geometry_buffer<T>(&mut self, data: &Vec<T>, buf_type: BufferType, format: Format, slot: u32, byte_offset: usize) {
        // SSE 16 byte aligned reads are used for these buffers and thus they
        //  must be padded so a read doesn't go past the end of the array
        if buf_type == BufferType::Vertex || buf_type == BufferType::VertexAttribute {
            let required_padding;
            if mem::size_of::<T>() == 4 {
                required_padding = 3;
            } else if mem::size_of::<T>() % 16 == 0 {
                required_padding = 0;
            } else {
                required_padding = 1;
            }
            if data.capacity() - data.len() < required_padding {
                warn!("Vertex and vertex attribute buffers require padding at the end");
            }
        }
        debug_assert!(byte_offset % 4 == 0, "offset must be 4 byte aligned");
        debug_assert!(mem::size_of::<T>() % 4 == 0, "stride must be 4 byte aligned");
        rtcSetSharedGeometryBuffer(self.ptr,
            buf_type.into(),
            slot,
            format.into(),
            data.as_ptr() as *const c_void,
            byte_offset,
            mem::size_of::<T>(),
            data.len());
    }
    
    pub(crate) fn commit(&mut self) {
        unsafe { rtcCommitGeometry(self.ptr); }
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
    Triangle = RTC_GEOMETRY_TYPE_TRIANGLE,
    Quad = RTC_GEOMETRY_TYPE_QUAD,
    // Grid = RTC_GEOMETRY_TYPE_GRID,
    // Subdivision = RTC_GEOMETRY_TYPE_SUBDIVISION,
    //  TODO: various curve types...
    Sphere = RTC_GEOMETRY_TYPE_SPHERE_POINT,
    // RayFacingDisc = RTC_GEOMETRY_TYPE_DISC_POINT,
    Disc = RTC_GEOMETRY_TYPE_ORIENTED_DISC_POINT,
    User = RTC_GEOMETRY_TYPE_USER,
    // Instance = RTC_GEOMETRY_TYPE_INSTANCE,
}

into_primitive!(GeometryType, i32);

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum BufferType {
    Index  = RTC_BUFFER_TYPE_INDEX,
    Vertex = RTC_BUFFER_TYPE_VERTEX,
    VertexAttribute = RTC_BUFFER_TYPE_VERTEX_ATTRIBUTE,
    Normal = RTC_BUFFER_TYPE_NORMAL,
    // Tangent = RTC_BUFFER_TYPE_TANGENT,
    // Grid = RTC_BUFFER_TYPE_GRID,
    // Face = RTC_BUFFER_TYPE_FACE,
    // Level = RTC_BUFFER_TYPE_LEVEL,
    // EdgeCreaseIndex = RTC_BUFFER_TYPE_EDGE_CREASE_INDEX,
    // EdgeCreaseWeight = RTC_BUFFER_TYPE_EDGE_CREASE_WEIGHT,
    // VertexCreaseIndex = RTC_BUFFER_TYPE_VERTEX_CREASE_INDEX,
    // VertexCreaseWeight = RTC_BUFFER_TYPE_VERTEX_CREASE_WEIGHT,
    // Hole = RTC_BUFFER_TYPE_HOLE,
    // Flags = RTC_BUFFER_TYPE_FLAGS,
}

into_primitive!(BufferType, i32);