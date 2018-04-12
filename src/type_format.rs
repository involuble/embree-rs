use sys::*;

use cgmath;

pub(crate) trait TypeFormat {
    const FORMAT: Format;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Format {
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

into_primitive!(Format, i32);

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum MatrixFormat {
    float3x4RowMajor = RTCFormat_RTC_FORMAT_FLOAT3X4_ROW_MAJOR,
    float3x4ColumnMajor = RTCFormat_RTC_FORMAT_FLOAT3X4_COLUMN_MAJOR,
    float4x4ColumnMajor = RTCFormat_RTC_FORMAT_FLOAT4X4_COLUMN_MAJOR,
}

into_primitive!(MatrixFormat, i32);

pub(crate) trait MatrixTypeFormat {
    const FORMAT: MatrixFormat;

    fn as_ptr(&self) -> *const f32;
}

impl MatrixTypeFormat for cgmath::Matrix4<f32> {
    const FORMAT: MatrixFormat = MatrixFormat::float4x4ColumnMajor;

    fn as_ptr(&self) -> *const f32 {
        <cgmath::Matrix4<f32> as cgmath::Matrix>::as_ptr(self)
    }
}

// impl MatrixTypeFormat for mint::ColumnMatrix3x4<f32> {
//     const FORMAT: MatrixFormat = MatrixFormat::float3x4ColumnMajor;

//     fn as_ptr(&self) -> *const f32 {
//         let xfm: &[f32; 12] = self.as_ref();
//         xfm.as_ptr()
//     }
// }

// impl MatrixTypeFormat for mint::RowMatrix3x4<f32> {
//     const FORMAT: MatrixFormat = MatrixFormat::float3x4RowMajor;

//     fn as_ptr(&self) -> *const f32 {
//         let xfm: &[f32; 12] = self.as_ref();
//         xfm.as_ptr()
//     }
// }

impl TypeFormat for cgmath::Vector2<f32> {
    const FORMAT: Format = Format::f32x2;
}

impl TypeFormat for cgmath::Vector3<f32> {
    const FORMAT: Format = Format::f32x3;
}

impl TypeFormat for cgmath::Point3<f32> {
    const FORMAT: Format = Format::f32x3;
}

impl TypeFormat for cgmath::Vector4<f32> {
    const FORMAT: Format = Format::f32x4;
}
