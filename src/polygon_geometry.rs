use std::cmp::max;

use cgmath::*;

use sys::*;

use buffer::*;
use device::*;
use geometry::*;
use type_format::*;

#[repr(C)]
pub struct Triangle(u32, u32, u32);

#[repr(C)]
pub struct Quad(u32, u32, u32, u32);

impl TypeFormat for Triangle {
    const FORMAT: Format = Format::u32x3;
}

impl TypeFormat for Quad {
    const FORMAT: Format = Format::u32x4;
}

pub trait PolygonType {
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

pub type TriangleGeometry = PolygonGeometry<Triangle>;
pub type QuadGeometry = PolygonGeometry<Quad>;

// Internal use constants
const NORMALS_SLOT: u32 = 0;
pub const UV_SLOT: u32 = 1;

pub struct PolygonGeometry<P, T1 = Vector3<f32>>
        where P: TypeFormat, P: PolygonType, P: 'static,
              T1: TypeFormat, T1: 'static {
    pub(crate) handle: GeometryHandle,
    pub indices: Buffer<P>,
    pub vertices: Buffer<Point3<f32>>,
    pub normals: Option<Buffer<Vector3<f32>>>,
    pub uv_buf: Option<Buffer<Vector2<f32>>>,
    pub attribs: Option<Buffer<T1>>,
}

impl<P, T1> PolygonGeometry<P, T1>
        where P: TypeFormat, P: PolygonType, P: 'static,
            T1: TypeFormat, T1: 'static {
    pub fn new(device: &Device, index_buffer: Vec<P>, vertex_buffer: Vec<Point3<f32>>) -> Self {
        assert!(P::POLYGON_TYPE == GeometryType::Triangle ||
            P::POLYGON_TYPE == GeometryType::Quad,
            "embree only supports triangles and quads");
        let handle = GeometryHandle::new(device, P::POLYGON_TYPE);
        PolygonGeometry {
            handle: handle,
            indices: Buffer::new(index_buffer),
            vertices: Buffer::new(vertex_buffer),
            normals: None,
            uv_buf: None,
            attribs: None,
        }
    }

    pub fn set_normal_buffer(&mut self, buf: Vec<Vector3<f32>>) {
        self.normals = Some(Buffer::new(buf));
    }

    pub fn set_uv_buffer(&mut self, buf: Vec<Vector2<f32>>) {
        self.uv_buf = Some(Buffer::new(buf));
    }

    pub fn set_vertex_attrib_buffer(&mut self, buf: Vec<T1>) {
        self.attribs = Some(Buffer::new(buf));
    }

    pub fn commit(mut self) -> Box<Geometry> {
        let mut attrib_count = 0;
        if self.normals.is_some() { attrib_count = NORMALS_SLOT + 1; }
        if self.normals.is_some() { attrib_count = UV_SLOT + 1; }
        if self.attribs.is_some() { attrib_count = 3; }
        
        self.indices.bind_to_geometry(&self.handle, BufferType::Index, 0);
        self.vertices.bind_to_geometry(&self.handle, BufferType::Vertex, 0);

        unsafe { rtcSetGeometryVertexAttributeCount(self.handle.ptr, attrib_count); }
        if let Some(ref mut buf) = self.normals {
            buf.bind_to_geometry(&self.handle, BufferType::VertexAttribute, NORMALS_SLOT);
        }
        if let Some(ref mut buf) = self.uv_buf {
            buf.bind_to_geometry(&self.handle, BufferType::VertexAttribute, UV_SLOT);
        }
        if let Some(ref mut buf) = self.attribs {
            buf.bind_to_geometry(&self.handle,
                BufferType::VertexAttribute,
                max(NORMALS_SLOT, UV_SLOT) + 1);
        }

        unsafe { rtcCommitGeometry(self.handle.ptr); }

        Box::new(self)
    }
}

impl<P, T1> Geometry for PolygonGeometry<P, T1>
        where P: TypeFormat, P: PolygonType, P: 'static,
            T1: TypeFormat, T1: 'static {
    fn get_handle(&self) -> &GeometryHandle {
        &self.handle
    }
}