use std::os::raw::c_void;
use std::ptr;
use std::u32;
use std::f32;
use std::mem;
use std::any::TypeId;

use cgmath::*;

use sys::*;

use aabb::*;
use common::*;
use device::*;
use geometry::*;
use ray::*;
use common::BuildQuality;

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

pub struct UserGeometry<T> {
    pub(crate) handle: GeometryHandle,
    pub prims: Vec<(T, u32)>,
}

impl<T: UserPrimitive> UserGeometry<T> {
    pub fn new(device: &Device, prims: Vec<(T, u32)>) -> Self {
        let handle = GeometryHandle::new(device, GeometryType::User);
        UserGeometry {
            handle,
            prims,
        }
    }

    pub fn new_realloc(device: &Device, data: Vec<T>) -> Self {
        let handle = GeometryHandle::new(device, GeometryType::User);
        let prims = data.into_iter().map(|p| (p, 0)).collect();
        UserGeometry {
            handle,
            prims,
        }
    }

    pub fn set_build_quality(&self, quality: BuildQuality) {
        self.handle.set_build_quality(quality);
    }

    pub fn build(mut self) -> BuiltUserGeometry<T> {
        assert!(self.prims.len() <= u32::MAX as usize);
        let handle = self.handle.as_ptr();

        // Pass the array as user_ptr
        let user_ptr = self.prims.as_mut_ptr() as *mut c_void;
        unsafe {
            rtcSetGeometryUserPrimitiveCount(handle, self.prims.len() as u32);
            rtcSetGeometryUserData(handle, user_ptr);
            rtcSetGeometryBoundsFunction(handle, Some(bounds_func::<T>), user_ptr);
            rtcSetGeometryIntersectFunction(handle, Some(intersect_func::<T>));
            rtcSetGeometryOccludedFunction(handle, Some(occluded_func::<T>));

            rtcCommitGeometry(handle);
        }

        BuiltUserGeometry {
            inner: self,
        }
    }

    pub(crate) fn update_geom_id(&mut self, id: GeomID) {
        let id = id.unwrap();
        for p in self.prims.iter_mut() {
            p.1 = id;
        }
    }

    pub(crate) fn into_erased(self) -> ErasedUserGeometry {
        let ptr = self.prims.as_ptr();
        let len = self.prims.len();
        let cap = self.prims.capacity();

        mem::forget(self.prims);

        ErasedUserGeometry {
            handle: self.handle,
            ty: TypeId::of::<T>(),
            vec_ptr: ptr as *const (),
            len,
            cap,
        }
    }
}

pub struct BuiltUserGeometry<T> {
    pub(crate) inner: UserGeometry<T>,
}

#[allow(dead_code)]
pub struct ErasedUserGeometry {
    pub(crate) handle: GeometryHandle,
    ty: TypeId,
    vec_ptr: *const (),
    len: usize,
    cap: usize,
}

unsafe impl Send for ErasedUserGeometry {}
unsafe impl Sync for ErasedUserGeometry {}

unsafe fn prim_from_user_ptr<T: UserPrimitive>(user_ptr: *mut c_void, prim_id: u32) -> &'static (T, u32) {
    let data_ptr = user_ptr as *mut (T, u32);
    let prim_ptr = data_ptr.offset(prim_id as isize);

    &mut *prim_ptr
}

unsafe extern "C" fn bounds_func<T: UserPrimitive>(args: *const RTCBoundsFunctionArguments) {
    let (prim, _geom_id) = prim_from_user_ptr::<T>((*args).geometryUserPtr, (*args).primID);

    ptr::write((*args).bounds_o, prim.bounds().into());
}

unsafe extern "C" fn intersect_func<T: UserPrimitive>(args: *const RTCIntersectFunctionNArguments) {
    let (prim, geom_id) = prim_from_user_ptr::<T>((*args).geometryUserPtr, (*args).primID);

    debug_assert!((*args).N == 1);
    if *(*args).valid == 0 { return; }

    let rayhit_ptr = (*args).rayhit as *mut RTCRayHit;

    // let rtcray: &mut RTCRay = &mut (*rayhit).ray;
    let hit: &mut RTCHit = &mut (*rayhit_ptr).hit;

    let ray: Ray = (*rayhit_ptr).ray.into();

    // TODO: need to expose a way to call rtcFilterIntersection (possibly by passing a closure in)
    let prim_hit = prim.intersect(&ray);

    if prim_hit.t > 0.0 {
        debug_assert!(ray.in_range(prim_hit.t), "Intersect function returning distance out of ray bounds");
        (*rayhit_ptr).ray.tfar = prim_hit.t;
        hit.Ng_x = prim_hit.Ng.x;
        hit.Ng_y = prim_hit.Ng.y;
        hit.Ng_z = prim_hit.Ng.z;
        hit.u = prim_hit.uv.x;
        hit.v = prim_hit.uv.y;
        hit.primID = (*args).primID;
        hit.geomID = *geom_id;
        hit.instID = (*(*args).context).instID;
    }
}

unsafe extern "C" fn occluded_func<T: UserPrimitive>(_args: *const RTCOccludedFunctionNArguments) {
    unimplemented!();
}