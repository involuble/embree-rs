use cgmath::*;

use sys::*;

use common::*;
use device::*;
use geometry::*;
use common::BuildQuality;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

pub struct SphereGeometry {
    pub(crate) handle: GeometryHandle,
    pub prims: Vec<Sphere>,
}

impl SphereGeometry {
    pub fn new(device: &Device, prims: Vec<Sphere>) -> Self {
        let handle = GeometryHandle::new(device, GeometryType::Sphere);
        SphereGeometry {
            handle,
            prims,
        }
    }

    pub fn set_build_quality(&self, quality: BuildQuality) {
        self.handle.set_build_quality(quality);
    }

    pub fn build(mut self) -> Geometry {
        self.handle.bind_shared_geometry_buffer(&mut self.prims, BufferType::Vertex, Format::f32x4, 0, 0);

        unsafe { rtcCommitGeometry(self.handle.ptr); }

        Geometry::new(GeometryInternal::Spheres(self))
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Disc {
    pub center: Point3<f32>,
    pub radius: f32,
    pub normal: Vector3<f32>,
}

pub struct DiscGeometry {
    pub(crate) handle: GeometryHandle,
    pub prims: Vec<Disc>,
}

impl DiscGeometry {
    pub fn new(device: &Device, prims: Vec<Disc>) -> Self {
        let handle = GeometryHandle::new(device, GeometryType::Disc);
        DiscGeometry {
            handle,
            prims,
        }
    }

    pub fn set_build_quality(&self, quality: BuildQuality) {
        self.handle.set_build_quality(quality);
    }

    pub fn build(mut self) -> Geometry {
        self.handle.bind_shared_geometry_buffer(&mut self.prims, BufferType::Vertex, Format::f32x4, 0, 0);
        self.handle.bind_shared_geometry_buffer(&mut self.prims, BufferType::Normal, Format::f32x3, 0, offset_of!(Disc, normal));

        unsafe { rtcCommitGeometry(self.handle.ptr); }

        Geometry::new(GeometryInternal::Discs(self))
    }
}