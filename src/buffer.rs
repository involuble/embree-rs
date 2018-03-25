use std::mem;
use std::os::raw::c_void;

use sys::*;

use cgmath;

use geometry::GeometryHandle;

pub struct Buffer<T> where T: FormattedType {
    pub data: Vec<T>,
}

impl<T> Buffer<T> where T: FormattedType {
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

pub trait FormattedType {
    const FORMAT: BufferFormat;
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

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum BufferFormat {
    u32x1  = RTCFormat_RTC_FORMAT_UINT,
    u32x2  = RTCFormat_RTC_FORMAT_UINT2,
    u32x3  = RTCFormat_RTC_FORMAT_UINT3,
    u32x4  = RTCFormat_RTC_FORMAT_UINT4,
    f32x1  = RTCFormat_RTC_FORMAT_FLOAT,
    f32x2  = RTCFormat_RTC_FORMAT_FLOAT2,
    f32x3  = RTCFormat_RTC_FORMAT_FLOAT3,
    f32x4  = RTCFormat_RTC_FORMAT_FLOAT4,
    f32x5  = RTCFormat_RTC_FORMAT_FLOAT5,
    f32x6  = RTCFormat_RTC_FORMAT_FLOAT6,
    f32x7  = RTCFormat_RTC_FORMAT_FLOAT7,
    f32x8  = RTCFormat_RTC_FORMAT_FLOAT8,
    f32x9  = RTCFormat_RTC_FORMAT_FLOAT9,
    f32x10 = RTCFormat_RTC_FORMAT_FLOAT10,
    f32x11 = RTCFormat_RTC_FORMAT_FLOAT11,
    f32x12 = RTCFormat_RTC_FORMAT_FLOAT12,
    f32x13 = RTCFormat_RTC_FORMAT_FLOAT13,
    f32x14 = RTCFormat_RTC_FORMAT_FLOAT14,
    f32x15 = RTCFormat_RTC_FORMAT_FLOAT15,
    f32x16 = RTCFormat_RTC_FORMAT_FLOAT16,
}

into_primitive!(BufferFormat, i32);

// TODO: Should there be a Derive for these?

impl FormattedType for cgmath::Vector2<f32> {
    const FORMAT: BufferFormat = BufferFormat::f32x2;
}

impl FormattedType for cgmath::Vector3<f32> {
    const FORMAT: BufferFormat = BufferFormat::f32x3;
}

impl FormattedType for cgmath::Point3<f32> {
    const FORMAT: BufferFormat = BufferFormat::f32x3;
}

impl FormattedType for cgmath::Vector4<f32> {
    const FORMAT: BufferFormat = BufferFormat::f32x4;
}