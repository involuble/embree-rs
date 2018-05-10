use std::os::raw::c_void;
use std::ptr;
use std::u32;
use std::f32;

use cgmath::*;

use sys::*;

use aabb::*;
use device::*;
use geometry::*;
use ray::*;
use scene::BuildQuality;

pub trait UserPrimitive: 'static + Send + Sync {
    fn intersect(&self, ray: &Ray) -> UserPrimHit;
    fn bounds(&self) -> AABB;

    // TODO: Maybe expose this for convenience?
    // fn transform_by(&mut self, mat: Matrix4);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct UserPrimHit {
    pub t: f32,
    pub Ng: Vector3<f32>,
    pub uv: Vector2<f32>,
}

impl UserPrimHit {
    pub fn new(t: f32, normal: Vector3<f32>, uv: Vector2<f32>) -> Self {
        UserPrimHit {
            t: t,
            Ng: normal,
            uv: uv,
        }
    }

    pub fn miss() -> Self {
        UserPrimHit {
            t: f32::MIN,
            Ng: Vector3::zero(),
            uv: Vector2::zero(),
        }
    }
}

pub struct UserGeometry<T: UserPrimitive> {
    pub handle: GeometryHandle,
    id: GeomID,
    // TODO: Evaluate using smallvec for this
    pub data: Vec<T>,
}

impl<T: UserPrimitive> UserGeometry<T> {
    pub fn new(device: &Device, data: Vec<T>) -> Self {
        let handle = GeometryHandle::new(device, GeometryType::User);
        UserGeometry {
            handle: handle,
            id: GeomID::invalid(),
            data: data,
        }
    }

    pub fn set_build_quality(&self, quality: BuildQuality) {
        self.handle.set_build_quality(quality);
    }

    pub fn build(self) -> Geometry {
        assert!(self.data.len() <= u32::MAX as usize);
        let count = self.data.len();
        let handle = self.handle.as_ptr();
        // Pass the array as user_ptr
        // let user_ptr = self.data.as_mut_ptr() as *mut c_void;
        let mut boxed_geom = Box::new(self);
        let user_ptr = boxed_geom.as_mut() as *mut UserGeometry<T> as *mut c_void;
        unsafe {
            rtcSetGeometryUserPrimitiveCount(handle, count as u32);
            rtcSetGeometryUserData(handle, user_ptr);
            rtcSetGeometryBoundsFunction(handle, Some(bounds_func::<T>), user_ptr);
            rtcSetGeometryIntersectFunction(handle, Some(intersect_func::<T>));
            rtcSetGeometryOccludedFunction(handle, Some(occluded_func::<T>));

            rtcCommitGeometry(handle);
        }
        Geometry::new(GeometryInternal::Other(boxed_geom))
    }
}

impl<T: UserPrimitive> GeometryData for UserGeometry<T> {
    fn set_geom_id(&mut self, id: GeomID) {
        self.id = id;
    }

    fn handle(&self) -> &GeometryHandle {
        &self.handle
    }
}

unsafe extern "C" fn bounds_func<T: UserPrimitive>(args: *const RTCBoundsFunctionArguments) {
    // If the userPtr = &[T] then do this
    // let data_ptr = (*args).geometryUserPtr as *const T;
    // let prim_ptr = data_ptr.offset((*args).primID as isize);

    // let prim: &T = prim_ptr.as_ref().unwrap();

    let geometry: &UserGeometry<T> = ((*args).geometryUserPtr as *const UserGeometry<T>).as_ref().unwrap();

    let prim: &T = &geometry.data[(*args).primID as usize];

    ptr::write((*args).bounds_o, prim.bounds().into());
}

unsafe extern "C" fn intersect_func<T: UserPrimitive>(args: *const RTCIntersectFunctionNArguments) {
    let geometry: &UserGeometry<T> = ((*args).geometryUserPtr as *const UserGeometry<T>).as_ref().unwrap();

    let prim: &T = &geometry.data[(*args).primID as usize];

    debug_assert!((*args).N == 1);
    if *(*args).valid == 0 { return; }

    let rayhit = (*args).rayhit as *mut RTCRayHit;

    // let rtcray: &mut RTCRay = &mut (*rayhit).ray;
    let hit: &mut RTCHit = &mut (*rayhit).hit;

    let ray: Ray = (*rayhit).ray.into();

    // TODO: need to expose a way to call rtcFilterIntersection (possibly by passing a closure in)
    let prim_hit = prim.intersect(&ray);

    if prim_hit.t > 0.0 {
        debug_assert!(ray.in_range(prim_hit.t), "Intersect function returning distance out of ray bounds");
        (*rayhit).ray.tfar = prim_hit.t;
        hit.Ng_x = prim_hit.Ng.x;
        hit.Ng_y = prim_hit.Ng.y;
        hit.Ng_z = prim_hit.Ng.z;
        hit.u = prim_hit.uv.x;
        hit.v = prim_hit.uv.y;
        hit.primID = (*args).primID;
        hit.geomID = geometry.id.unwrap();
        hit.instID = (*(*args).context).instID;
    }
}

unsafe extern "C" fn occluded_func<T: UserPrimitive>(_args: *const RTCOccludedFunctionNArguments) {
    unimplemented!();
}