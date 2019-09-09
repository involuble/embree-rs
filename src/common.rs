use std::u32;

use cgmath::*;

use sys::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeomID {
    pub id: u32,
}

pub const INVALID_ID: u32 = u32::MAX;

impl GeomID {
    pub fn new(id: u32) -> Self {
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
        debug_assert!(!self.is_invalid());
        self.id
    }
}

impl From<u32> for GeomID {
    fn from(id: u32) -> GeomID {
        GeomID::new(id)
    }
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub lower: Point3<f32>,
    align0: f32,
    pub upper: Point3<f32>,
    align1: f32,
}

impl Bounds {
    pub fn zero() -> Self {
        Bounds {
            lower: Point3::origin(),
            align0: 0.0,
            upper: Point3::origin(),
            align1: 0.0,
        }
    }

    pub fn new(lower: Point3<f32>, upper: Point3<f32>) -> Self {
        Bounds {
            lower: lower,
            align0: 0.0,
            upper: upper,
            align1: 0.0,
        }
    }

    pub fn as_raw_ptr(&mut self) -> *mut RTCBounds {
        self as *mut Bounds as *mut RTCBounds
    }
}

#[test]
fn test_bounds_layout() {
    assert_eq!(std::mem::size_of::<Bounds>(), std::mem::size_of::<RTCBounds>());
    assert_eq!(offset_of!(Bounds, upper.x), offset_of!(RTCBounds, upper_x));
}

macro_rules! into_primitive {
    ($enum_name:ty, $prim:ty) => (
        impl Into<$prim> for $enum_name {
            fn into(self) -> $prim {
                unsafe { ::std::mem::transmute::<$enum_name, $prim>(self) }
            }
        }
    )
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum BuildQuality {
    Low = RTC_BUILD_QUALITY_LOW,
    Medium = RTC_BUILD_QUALITY_MEDIUM,
    High = RTC_BUILD_QUALITY_HIGH,
}

into_primitive!(BuildQuality, i32);

pub(crate) trait TypeFormat {
    const FORMAT: Format;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum Format {
    u32x1  = RTC_FORMAT_UINT,
    u32x2  = RTC_FORMAT_UINT2,
    u32x3  = RTC_FORMAT_UINT3,
    u32x4  = RTC_FORMAT_UINT4,
    f32x1  = RTC_FORMAT_FLOAT,
    f32x2  = RTC_FORMAT_FLOAT2,
    f32x3  = RTC_FORMAT_FLOAT3,
    f32x4  = RTC_FORMAT_FLOAT4,
    f32x5  = RTC_FORMAT_FLOAT5,
    f32x6  = RTC_FORMAT_FLOAT6,
    f32x7  = RTC_FORMAT_FLOAT7,
    f32x8  = RTC_FORMAT_FLOAT8,
    f32x9  = RTC_FORMAT_FLOAT9,
    f32x10 = RTC_FORMAT_FLOAT10,
    f32x11 = RTC_FORMAT_FLOAT11,
    f32x12 = RTC_FORMAT_FLOAT12,
    f32x13 = RTC_FORMAT_FLOAT13,
    f32x14 = RTC_FORMAT_FLOAT14,
    f32x15 = RTC_FORMAT_FLOAT15,
    f32x16 = RTC_FORMAT_FLOAT16,
}

into_primitive!(Format, i32);

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum MatrixFormat {
    float3x4RowMajor = RTC_FORMAT_FLOAT3X4_ROW_MAJOR,
    float3x4ColumnMajor = RTC_FORMAT_FLOAT3X4_COLUMN_MAJOR,
    float4x4ColumnMajor = RTC_FORMAT_FLOAT4X4_COLUMN_MAJOR,
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
