use std::u32;

use sys::*;

use cgmath::*;

use common::GeomID;

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub tnear: f32,
    pub dir: Vector3<f32>,
    time: f32,
    pub tfar: f32,
    mask: u32,
    id: u32,
    flags: u32,
}

impl Ray {
    pub fn new(origin: Point3<f32>, dir: Vector3<f32>, tnear: f32, tfar: f32) -> Self {
        debug_assert!(tnear >= 0.0, "Invalid tnear");
        debug_assert!(tfar > tnear, "Invalid tfar");
        Ray {
            origin: origin,
            tnear: tnear,
            dir: dir,
            time: 0.0,
            tfar: tfar,
            mask: u32::MAX,
            id: 0,
            flags: 0,
        }
    }

    pub fn in_range(&self, t: f32) -> bool {
        t > self.tnear && t < self.tfar
    }

    pub fn point_at_dist(&self, t: f32) -> Point3<f32> {
        self.origin + t*self.dir
    }
}

#[test]
fn test_ray_layout() {
    assert_eq!(std::mem::size_of::<Ray>(), std::mem::size_of::<RTCRay>());
    assert_eq!(offset_of!(Ray, origin.x), offset_of!(RTCRay, org_x));
    assert_eq!(offset_of!(Ray, tnear), offset_of!(RTCRay, tnear));
    assert_eq!(offset_of!(Ray, dir.x), offset_of!(RTCRay, dir_x));
    assert_eq!(offset_of!(Ray, time), offset_of!(RTCRay, time));
    assert_eq!(offset_of!(Ray, tfar), offset_of!(RTCRay, tfar));
    assert_eq!(offset_of!(Ray, mask), offset_of!(RTCRay, mask));
    assert_eq!(offset_of!(Ray, id), offset_of!(RTCRay, id));
    assert_eq!(offset_of!(Ray, flags), offset_of!(RTCRay, flags));
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Hit {
    pub Ng: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub prim_id: GeomID,
    pub geom_id: GeomID,
    pub inst_id: GeomID,
}

impl Hit {
    pub fn empty() -> Self {
        Hit {
            Ng: Vector3::zero(),
            uv: Vector2::zero(),
            geom_id: GeomID::invalid(),
            prim_id: GeomID::invalid(),
            inst_id: GeomID::invalid(),
        }
    }

    pub fn is_hit(&self) -> bool {
        !self.geom_id.is_invalid()
    }
}

#[test]
fn test_hit_layout() {
    assert_eq!(std::mem::size_of::<Hit>(), std::mem::size_of::<RTCHit>());
    assert_eq!(offset_of!(Hit, Ng.x), offset_of!(RTCHit, Ng_x));
    assert_eq!(offset_of!(Hit, uv.x), offset_of!(RTCHit, u));
    assert_eq!(offset_of!(Hit, uv.y), offset_of!(RTCHit, v));
    assert_eq!(offset_of!(Hit, geom_id), offset_of!(RTCHit, geomID));
    assert_eq!(offset_of!(Hit, prim_id), offset_of!(RTCHit, primID));
    assert_eq!(offset_of!(Hit, inst_id), offset_of!(RTCHit, instID));
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct RayHit {
    pub ray: Ray,
    pub hit: Hit,
}

impl RayHit {
    pub fn as_raw_ptr(&mut self) -> *mut RTCRayHit {
        self as *mut RayHit as *mut RTCRayHit
    }
}
