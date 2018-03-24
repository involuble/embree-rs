use sys::*;

use mint::*;

use buffer::*;
use device::*;
use geometry::*;

#[repr(C)]
pub struct Triangle(u32, u32, u32);

impl FormattedType for Triangle {
    const FORMAT: BufferFormat = BufferFormat::u32x3;
}

pub struct TriangleGeometry<T1 = Vector3<f32>, T2 = Vector3<f32>>
        where T1: FormattedType, T1: 'static, T2: FormattedType, T2: 'static {
    pub(crate) handle: GeometryHandle,
    pub indices: Buffer<Triangle>,
    pub vertices: Buffer<Point3<f32>>,
    // TODO: Kinda hacky but eh
    pub attribs1: Option<Buffer<T1>>,
    pub attribs2: Option<Buffer<T2>>,
}

impl<T1, T2> TriangleGeometry<T1, T2> where T1: FormattedType, T1: 'static, T2: FormattedType, T2: 'static {
    pub fn new(device: &Device, index_buffer: Vec<Triangle>, vertex_buffer: Vec<Point3<f32>>) -> Self {
        let handle = GeometryHandle::new(device, GeometryType::Triangle);
        TriangleGeometry {
            handle: handle,
            indices: Buffer::new(index_buffer),
            vertices: Buffer::new(vertex_buffer),
            attribs1: None,
            attribs2: None,
        }
    }

    pub fn set_vertex_attrib_buffer_slot1(&mut self, buf: Vec<T1>) {
        self.attribs1 = Some(Buffer::new(buf));
    }

    pub fn set_vertex_attrib_buffer_slot2(&mut self, buf: Vec<T2>) {
        self.attribs2 = Some(Buffer::new(buf));
    }

    pub fn commit(mut self) -> Box<Geometry> {
        let mut attrib_count = 0;
        if self.attribs1.is_some() { attrib_count = 1; }
        if self.attribs2.is_some() { attrib_count = 2; }
        
        self.indices.bind_to_geometry(&self.handle, BufferType::Index, 0);
        self.vertices.bind_to_geometry(&self.handle, BufferType::Vertex, 0);

        unsafe { rtcSetGeometryVertexAttributeCount(self.handle.ptr, attrib_count); }
        if let Some(ref mut buf) = self.attribs1 {
            buf.bind_to_geometry(&self.handle, BufferType::VertexAttribute, 0);
        }
        if let Some(ref mut buf) = self.attribs2 {
            buf.bind_to_geometry(&self.handle, BufferType::VertexAttribute, 1);
        }

        unsafe { rtcCommitGeometry(self.handle.ptr); }

        Box::new(self)
    }
}

impl<T1, T2> Geometry for TriangleGeometry<T1, T2> where T1: FormattedType, T2: FormattedType {
    fn get_handle(&self) -> &GeometryHandle {
        &self.handle
    }
}