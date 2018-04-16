use std::f32;

use sys::*;

use cgmath::*;
use vec_map::*;

use aabb::*;
use device::Device;
use geometry::*;
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

    pub fn attach(&mut self, mut geometry: Geometry) -> GeomID {
        let id = unsafe { rtcAttachGeometry(self.handle.ptr, geometry.handle().as_ptr()) };
        assert!(!self.geometries.contains_key(id as usize), "Geometry id already assigned");
        let geom_id = GeomID::new(id);
        geometry.set_geom_id(geom_id);
        self.geometries.insert(id as usize, geometry);
        geom_id
    }

    pub fn set_build_quality(&self, quality: BuildQuality) {
        unsafe { rtcSetSceneBuildQuality(self.handle.ptr, quality.into()); }
    }

    pub fn set_flags(&self, flags: SceneFlags) {
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

// bitflags! {
//     pub struct IntersectionContextFlags: i32 {
//         const INCOHERENT = RTCIntersectContextFlags_RTC_INTERSECT_CONTEXT_FLAG_INCOHERENT;
//         const COHERENT = RTCIntersectContextFlags_RTC_INTERSECT_CONTEXT_FLAG_COHERENT;
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
            instance_id: GeomID::new(rayhit.hit.instID[0]),
            t: rayhit.ray.tfar,
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

    pub fn edit(self) -> SceneBuilder {
        SceneBuilder {
            handle: self.handle,
            geometries: self.geometries,
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum BuildQuality {
    Low = RTCBuildQuality_RTC_BUILD_QUALITY_LOW,
    Medium = RTCBuildQuality_RTC_BUILD_QUALITY_MEDIUM,
    High = RTCBuildQuality_RTC_BUILD_QUALITY_HIGH,
}

into_primitive!(BuildQuality, i32);

bitflags! {
    #[repr(C)]
    pub struct SceneFlags: i32 {
        const DYNAMIC = RTCSceneFlags_RTC_SCENE_FLAG_DYNAMIC;
        const COMPACT = RTCSceneFlags_RTC_SCENE_FLAG_COMPACT;
        const ROBUST  = RTCSceneFlags_RTC_SCENE_FLAG_ROBUST;
        // const FILTER_FUNCTION = RTCSceneFlags_RTC_SCENE_FLAG_CONTEXT_FILTER_FUNCTION;
    }
}