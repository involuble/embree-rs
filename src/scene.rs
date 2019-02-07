use std::f32;

use sys::*;

use cgmath::*;
use vec_map::*;

use aabb::*;
use common::*;
use device::Device;
use geometry::*;
use user_geometry::*;
use ray::*;

pub struct Scene {
    handle: SceneHandle,
    geometries: VecMap<Geometry>,
}

pub struct SceneBuilder {
    handle: SceneHandle,
    geometries: VecMap<Geometry>,
}

#[repr(C)]
pub struct SceneHandle {
    pub(crate) ptr: RTCScene,
    // device_handle: Device,
}

impl SceneHandle {
    pub(crate) fn new(device: &Device) -> Self {
        let h = unsafe { rtcNewScene(device.ptr) };
        SceneHandle { ptr: h }
    }

    pub(crate) fn as_ptr(&self) -> RTCScene {
        self.ptr
    }
}

unsafe impl Send for SceneHandle {}
unsafe impl Sync for SceneHandle {}

impl Clone for SceneHandle {
    fn clone(&self) -> SceneHandle {
        unsafe { rtcRetainScene(self.ptr) }
        SceneHandle { ptr: self.ptr }
    }
}

impl Drop for SceneHandle {
    fn drop(&mut self) {
        unsafe { rtcReleaseScene(self.ptr) }
    }
}

impl SceneBuilder {
    pub fn new(device: &Device) -> Self {
        SceneBuilder {
            handle: SceneHandle::new(device),
            geometries: VecMap::new(),
        }
    }

    pub fn attach(&mut self, geometry: Geometry) -> GeomID {
        let id = unsafe { rtcAttachGeometry(self.handle.ptr, geometry.handle().as_ptr()) };
        assert!(!self.geometries.contains_key(id as usize), "Geometry id already assigned");
        let geom_id = GeomID::new(id);
        self.geometries.insert(id as usize, geometry);
        geom_id
    }

    pub fn attach_user_geometry<T: UserPrimitive>(&mut self, geometry: BuiltUserGeometry<T>) -> GeomID {
        let mut geometry = geometry.inner;
        let id = unsafe { rtcAttachGeometry(self.handle.ptr, geometry.handle.as_ptr()) };
        let geom_id = GeomID::new(id);
        geometry.update_geom_id(geom_id);
        self.geometries.insert(id as usize, Geometry::new(GeometryInternal::User(geometry.into_erased())));
        geom_id
    }

    pub fn set_build_quality(&mut self, quality: BuildQuality) {
        unsafe { rtcSetSceneBuildQuality(self.handle.ptr, quality.into()); }
    }

    pub fn set_flags(&mut self, flags: SceneFlags) {
        unsafe { rtcSetSceneFlags(self.handle.ptr, flags.bits()); }
    }

    pub fn get_flags(&self) -> SceneFlags {
        let flags: i32 = unsafe { rtcGetSceneFlags(self.handle.ptr) };
        SceneFlags::from_bits_truncate(flags)
    }

    pub fn build(self) -> Scene {
        unsafe {
            rtcCommitScene(self.handle.ptr);
        }
        Scene {
            handle: self.handle,
            geometries: self.geometries,
        }
    }
}

// unsafe extern "C" fn scene_progress_monitor_callback(ptr: *mut c_void, n: f64) -> bool {
//     true
// }

// bitflags! {
//     pub struct IntersectionContextFlags: i32 {
//         const INCOHERENT = RTC_INTERSECT_CONTEXT_FLAG_INCOHERENT;
//         const COHERENT = RTC_INTERSECT_CONTEXT_FLAG_COHERENT;
//     }
// }

// TODO: Could make new struct for this
fn empty_intersect_context() -> RTCIntersectContext{
    RTCIntersectContext {
        flags: 0,
        filter: None,
        instID: [INVALID_ID],
    }
}

#[repr(C)]
#[repr(align(16))]
struct RTCRayHitAligned {
    pub ray: RTCRay,
    pub hit: RTCHit,
}

#[repr(C)]
#[repr(align(16))]
struct RTCRayAligned {
    pub ray: RTCRay,
}

#[repr(C)]
#[repr(align(16))]
struct RTCBoundsAligned {
    pub bounds: RTCBounds,
}

// struct GeometryQueryHandle<'a> {
//     ptr: RTCGeometry,
//     phantom: ::std::marker::PhantomData<&'a ()>,
// }

// impl<'a> GeometryQueryHandle<'a> {
//     fn interpolate() {}
// }

impl Scene {
    pub fn bounds(&self) -> AABB {
        let mut b = RTCBoundsAligned { bounds: AABB::zero().into() };
        unsafe { rtcGetSceneBounds(self.handle.ptr, &mut b.bounds); }
        b.bounds.into()
    }

    pub fn intersect(&self, ray: Ray) -> Hit {
        let mut context: RTCIntersectContext = empty_intersect_context();
        let mut rayhit = RTCRayHitAligned {
            ray: ray.into(),
            hit: RTCHit {
                Ng_x: 0.0,
                Ng_y: 0.0,
                Ng_z: 0.0,
                u: 0.0,
                v: 0.0,
                geomID: INVALID_ID,
                primID: INVALID_ID,
                instID: [INVALID_ID],
            }
        };
        unsafe {
            rtcIntersect1(self.handle.as_ptr(),
                &mut context,
                &mut rayhit as *mut RTCRayHitAligned as *mut RTCRayHit);
        }
        Hit {
            Ng: Vector3::new(rayhit.hit.Ng_x, rayhit.hit.Ng_y, rayhit.hit.Ng_z).normalize(),
            uv: Vector2::new(rayhit.hit.u, rayhit.hit.v),
            geom_id: GeomID::new(rayhit.hit.geomID),
            prim_id: GeomID::new(rayhit.hit.primID),
            t: rayhit.ray.tfar,
            // instance_id: GeomID::new(rayhit.hit.instID[0]),
        }
    }

    pub fn occluded(&self, ray: Ray) -> bool {
        let mut r = RTCRayAligned {
            ray: ray.into(),
        };
        let mut context: RTCIntersectContext = empty_intersect_context();
        unsafe {
            rtcOccluded1(self.handle.as_ptr(),
                &mut context,
                &mut r.ray);
        }
        r.ray.tfar == f32::NEG_INFINITY
    }

    // fn query(&self, id: GeomID) -> GeometryQueryHandle<'_> {
    //     unimplemented!()
    // }

    pub fn edit(self) -> SceneBuilder {
        SceneBuilder {
            handle: self.handle,
            geometries: self.geometries,
        }
    }
}

bitflags! {
    #[repr(C)]
    pub struct SceneFlags: i32 {
        const DYNAMIC = RTC_SCENE_FLAG_DYNAMIC;
        const COMPACT = RTC_SCENE_FLAG_COMPACT;
        const ROBUST  = RTC_SCENE_FLAG_ROBUST;
        // const ENABLE_FILTER_FUNCTION = RTC_SCENE_FLAG_CONTEXT_FILTER_FUNCTION;
    }
}