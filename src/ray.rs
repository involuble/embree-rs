use std::u32;

use sys::*;

use cgmath::*;

use geometry::GeomID;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub tnear: f32,
    pub dir: Vector3<f32>,
    pub tfar: f32,
}

impl Ray {
    pub fn new(origin: Point3<f32>, dir: Vector3<f32>, tnear: f32, tfar: f32) -> Self {
        debug_assert!(tnear >= 0.0, "Invalid tnear");
        debug_assert!(tfar > tnear, "Invalid tfar");
        Ray {
            origin: origin,
            tnear: tnear,
            dir: dir,
            tfar: tfar,
        }
    }

    pub fn in_range(&self, t: f32) -> bool {
        t > self.tnear && t < self.tfar
    }

    pub fn point_at_dist(&self, t: f32) -> Point3<f32> {
        self.origin + t*self.dir
    }
}

impl Into<RTCRay> for Ray {
    fn into(self) -> RTCRay {
        RTCRay {
            org_x: self.origin.x,
            org_y: self.origin.y,
            org_z: self.origin.z,
            tnear: self.tnear,
            dir_x: self.dir.x,
            dir_y: self.dir.y,
            dir_z: self.dir.z,
            time: 0.0,
            tfar: self.tfar,
            mask: u32::MAX,
            id: 0,
            flags: 0,
        }
    }
}

impl From<RTCRay> for Ray {
    fn from(ray: RTCRay) -> Self {
        Ray {
            origin: Point3::new(ray.org_x, ray.org_y, ray.org_z),
            tnear: ray.tnear,
            dir: Vector3::new(ray.dir_x, ray.dir_y, ray.dir_z),
            tfar: ray.tfar,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Hit {
    pub Ng: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub geom_id: GeomID,
    pub prim_id: GeomID,
    pub instance_id: GeomID,
    pub t: f32,
}

impl Hit {
    pub fn empty() -> Self {
        Hit {
            Ng: Vector3::zero(),
            uv: Vector2::zero(),
            t: 0.0,
            geom_id: GeomID::invalid(),
            prim_id: GeomID::invalid(),
            instance_id: GeomID::invalid(),
        }
    }

    pub fn is_hit(&self) -> bool {
        !self.geom_id.is_invalid()
    }
}
