use std::mem;
use std::os::raw::c_void;

use sys::*;

use geometry::GeometryHandle;
use type_format::*;

pub struct Buffer<T> where T: TypeFormat {
    pub data: Vec<T>,
}

impl<T> Buffer<T> where T: TypeFormat {
    pub fn new(v: Vec<T>) -> Self {
        Buffer { data: v }
    }

    pub(crate) fn bind_to_geometry(&mut self, handle: &GeometryHandle, buf_type: BufferType, slot: u32) {
        assert!(mem::size_of::<T>() % 4 == 0, "size_of::<T> must be a multiple of 4");
        // embree reads from these buffers at a granularity of 16 bytes. So we make sure there's enough space at the
        //  end for this not to segfault
        if buf_type == BufferType::Vertex || buf_type == BufferType::VertexAttribute {
            if mem::size_of::<T>() == 4 {
                self.data.reserve(3);
            } else {
                self.data.reserve(1);
            }
        }
        unsafe {
            rtcSetSharedGeometryBuffer(handle.ptr,
                buf_type.into(),
                slot,
                T::FORMAT.into(),
                self.data.as_ptr() as *const c_void,
                0, // byteOffset
                mem::size_of::<T>(),
                self.data.len());
        }
    }
}

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