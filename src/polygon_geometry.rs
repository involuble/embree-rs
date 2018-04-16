use cgmath::*;

use sys::*;

use device::*;
use geometry::*;
use type_format::*;
use scene::BuildQuality;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
}

impl Triangle {
    pub fn new(v0: u32, v1: u32, v2: u32) -> Self {
        Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quad {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
}

impl Quad {
    pub fn new(v0: u32, v1: u32, v2: u32, v3: u32) -> Self {
        Quad {
            v0: v0,
            v1: v1,
            v2: v2,
            v3: v3,
        }
    }
}

impl TypeFormat for Triangle {
    const FORMAT: Format = Format::u32x3;
}

impl TypeFormat for Quad {
    const FORMAT: Format = Format::u32x4;
}

trait PolygonType {
    // const VERTEX_COUNT: u32;
    const POLYGON_TYPE: GeometryType;
}

impl PolygonType for Triangle {
    // const VERTEX_COUNT: u32 = 3;
    const POLYGON_TYPE: GeometryType = GeometryType::Triangle;
}

impl PolygonType for Quad {
    // const VERTEX_COUNT: u32 = 4;
    const POLYGON_TYPE: GeometryType = GeometryType::Quad;
}

// Internal use constants
const NORMALS_SLOT: u32 = 0;
const UV_SLOT: u32 = 1;

pub struct TriangleMesh {
    pub handle: GeometryHandle,
    pub indices: Vec<Triangle>,
    pub vertices: Vec<Point3<f32>>,
    pub normals: Option<Vec<Vector3<f32>>>,
    pub tex_coords: Option<Vec<Vector2<f32>>>,
    pub attribs: Option<Vec<f32>>,
}

impl TriangleMesh {
    pub fn new(device: &Device, index_buffer: Vec<Triangle>, vertex_buffer: Vec<Point3<f32>>) -> Self {
        let handle = GeometryHandle::new(device, Triangle::POLYGON_TYPE);
        TriangleMesh {
            handle: handle,
            indices: index_buffer,
            vertices: vertex_buffer,
            normals: None,
            tex_coords: None,
            attribs: None,
        }
    }

    pub fn set_normal_buffer(&mut self, buf: Vec<Vector3<f32>>) {
        self.normals = Some(buf);
    }

    pub fn set_texcoord_buffer(&mut self, buf: Vec<Vector2<f32>>) {
        self.tex_coords = Some(buf);
    }

    pub fn set_attrib_buffer(&mut self, buf: Vec<f32>) {
        self.attribs = Some(buf);
    }

    pub fn transform_mesh(&mut self, transform: Matrix4<f32>) {
        for v in self.vertices.iter_mut() {
            *v = transform.transform_point(*v);
        }
        if let Some(ref mut normal_buf) = self.normals {
            let normal_transform = transform.invert().unwrap().transpose();
            for n in normal_buf.iter_mut() {
                *n = normal_transform.transform_vector(*n);
            }
        }
    }

    pub fn set_build_quality(&self, quality: BuildQuality) {
        self.handle.set_build_quality(quality);
    }

    pub fn build(mut self) -> Geometry {
        let mut attrib_count = 0;
        if self.normals.is_some() { attrib_count = NORMALS_SLOT + 1; }
        if self.tex_coords.is_some() { attrib_count = UV_SLOT + 1; }
        if self.attribs.is_some() { attrib_count = 3; }
        
        self.handle.bind_shared_geometry_buffer(&mut self.indices, BufferType::Index, Triangle::FORMAT, 0, 0);
        self.handle.bind_shared_geometry_buffer(&mut self.vertices, BufferType::Vertex, Format::f32x3, 0, 0);

        unsafe { rtcSetGeometryVertexAttributeCount(self.handle.ptr, attrib_count); }

        if let Some(ref mut data) = self.normals {
            self.handle.bind_shared_geometry_buffer(data, BufferType::VertexAttribute, Format::f32x3, NORMALS_SLOT, 0);
        }
        if let Some(ref mut data) = self.tex_coords {
            self.handle.bind_shared_geometry_buffer(data, BufferType::VertexAttribute, Format::f32x2, UV_SLOT, 0);
        }
        if let Some(ref mut _data) = self.attribs {
            // TODO
            // self.handle.bind_shared_geometry_buffer(data, BufferType::VertexAttribute, Format::f32x3, 2, 0);
            unimplemented!();
        }

        unsafe { rtcCommitGeometry(self.handle.ptr); }

        Geometry::new(GeometryInternal::Triangles(self))
    }
}