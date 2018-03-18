use sys::*;

use mint;

use geometry::*;

pub struct Triangle(u32, u32, u32);

pub struct TriangleGeometry {
    pub(crate) geometry_handle: RTCGeometry;
    pub index_buf: Vec<Triangle>;
    pub vertex_buf: Vec<mint::Point3>;
}