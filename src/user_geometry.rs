use std::ffi::c_void;
use std::ptr;
use std::u32;
use std::f32;

use cgmath::*;

use sys::*;

use common::*;
use device::*;
use geometry::*;
use ray::*;

pub trait UserPrimitive: 'static + Send + Sync {
    fn intersect(&self, ray: &Ray) -> UserPrimHit;
    fn bounds(&self) -> Bounds;

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

pub struct UserGeometry<T> {
    handle: GeometryHandle,
    id: u32,
    pub prims: Vec<T>,
}

impl<T: UserPrimitive> UserGeometry<T> {
    pub fn new(device: &Device, prims: Vec<T>) -> Self {
        let handle = GeometryHandle::new(device, GeometryType::User);
        UserGeometry {
            handle,
            prims,
            id: 0,
        }
    }
}

impl<T: UserPrimitive> Geometry for UserGeometry<T> {
    fn handle(&self) -> &GeometryHandle {
        &self.handle
    }

    fn handle_mut(&mut self) -> &mut GeometryHandle {
        &mut self.handle
    }
    
    fn set_geom_id(&mut self, id: u32) {
        self.id = id;
    }

    fn bind_buffers(&mut self) {
        assert!(self.prims.len() <= u32::MAX as usize);

        // Pass self as user_ptr
        let user_ptr = self as *const UserGeometry<T> as *mut c_void;
        unsafe {
            let handle = self.handle.as_raw_ptr();
            rtcSetGeometryUserPrimitiveCount(handle, self.prims.len() as u32);
            rtcSetGeometryUserData(handle, user_ptr);
            rtcSetGeometryBoundsFunction(handle, Some(bounds_func::<T>), user_ptr);
            rtcSetGeometryIntersectFunction(handle, Some(intersect_func::<T>));
            rtcSetGeometryOccludedFunction(handle, Some(occluded_func::<T>));
        }
    }
}

unsafe extern "C" fn bounds_func<T: UserPrimitive>(args: *const RTCBoundsFunctionArguments) {
    // If the userPtr = &[T] then do this to retrieve a primitive
    // let data_ptr = (*args).geometryUserPtr as *const T;
    // let prim_ptr = data_ptr.offset((*args).primID as isize);

    // let prim: &T = prim_ptr.as_ref().unwrap();

    let geometry: &UserGeometry<T> = ((*args).geometryUserPtr as *const UserGeometry<T>).as_ref().unwrap();

    let prim: &T = &geometry.prims[(*args).primID as usize];

    ptr::write((*args).bounds_o as *mut Bounds, prim.bounds());
}

unsafe extern "C" fn intersect_func<T: UserPrimitive>(args: *const RTCIntersectFunctionNArguments) {
    let geometry: &UserGeometry<T> = ((*args).geometryUserPtr as *const UserGeometry<T>).as_ref().unwrap();

    let prim: &T = &geometry.prims[(*args).primID as usize];

    debug_assert!((*args).N == 1);
    if *(*args).valid == 0 { return; }

    let rayhit = (*args).rayhit as *mut RTCRayHit as *mut RayHit;
    let rayhit = &mut *rayhit;

    let ray = &mut rayhit.ray;
    let hit = &mut rayhit.hit;

    // TODO: need to expose a way to call rtcFilterIntersection (possibly by passing a closure in)
    let prim_hit = prim.intersect(ray);

    if prim_hit.t >= ray.tnear {
        // The UserPrimitive intersect function should make sure the below invariant holds
        //  but check it anyways. This could be turned into a runtime check instead of an assert
        debug_assert!(ray.in_range(prim_hit.t), "Intersect function returning distance out of ray bounds");
        ray.tfar = prim_hit.t;
        hit.Ng = prim_hit.Ng;
        hit.uv = prim_hit.uv;
        hit.prim_id = (*args).primID.into();
        hit.geom_id = geometry.id.into();
        hit.inst_id = (*(*args).context).instID[0].into();
    }
}

unsafe extern "C" fn occluded_func<T: UserPrimitive>(_args: *const RTCOccludedFunctionNArguments) {
    unimplemented!();
}