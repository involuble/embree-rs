use cgmath::*;

use common::*;
use device::*;
use geometry::*;

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
}

impl Geometry for SphereGeometry {
    fn handle(&self) -> &GeometryHandle {
        &self.handle
    }

    fn handle_mut(&mut self) -> &mut GeometryHandle {
        &mut self.handle
    }

    fn bind_buffers(&mut self) {
        self.prims.reserve(1);
        unsafe {
            self.handle.bind_shared_geometry_buffer(&mut self.prims, BufferType::Vertex, Format::f32x4, 0, 0);
        }
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
}

impl Geometry for DiscGeometry {
    fn handle(&self) -> &GeometryHandle {
        &self.handle
    }

    fn handle_mut(&mut self) -> &mut GeometryHandle {
        &mut self.handle
    }

    fn bind_buffers(&mut self) {
        self.prims.reserve(1);
        unsafe {
            self.handle.bind_shared_geometry_buffer(&mut self.prims, BufferType::Vertex, Format::f32x4, 0, 0);
            self.handle.bind_shared_geometry_buffer(&mut self.prims, BufferType::Normal, Format::f32x3, 0, offset_of!(Disc, normal));
        }
    }
}