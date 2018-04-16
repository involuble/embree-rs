use cgmath::*;

use sys::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub lower: Point3<f32>,
    align0: f32,
    pub upper: Point3<f32>,
    align1: f32,
}

impl AABB {
    pub fn zero() -> Self {
        AABB {
            lower: Point3::origin(),
            align0: 0.0,
            upper: Point3::origin(),
            align1: 0.0,
        }
    }

    pub fn new(lower: Point3<f32>, upper: Point3<f32>) -> Self {
        AABB {
            lower: lower,
            align0: 0.0,
            upper: upper,
            align1: 0.0,
        }
    }
}

impl Into<RTCBounds> for AABB {
    fn into(self) -> RTCBounds {
        RTCBounds {
            lower_x: self.lower.x,
            lower_y: self.lower.y,
            lower_z: self.lower.z,
            align0:  self.align0,
            upper_x: self.upper.x,
            upper_y: self.upper.y,
            upper_z: self.upper.z,
            align1:  self.align1,
        }
    }
}

impl From<RTCBounds> for AABB {
    fn from(other: RTCBounds) -> Self {
        AABB {
            lower: Point3::new(other.lower_x, other.lower_y, other.lower_z),
            align0: other.align0,
            upper: Point3::new(other.upper_x, other.upper_y, other.upper_z),
            align1: other.align1,
        }
    }
}
