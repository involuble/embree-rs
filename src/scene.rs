use std::u32;
use std::f32;

use sys::*;

use cgmath::*;
use vec_map::*;

use device::Device;
use geometry::*;

pub struct Scene {
    // TODO
    pub(crate) handle: SceneHandle,
    pub(crate) geometries: VecMap<Geometry>,
}

pub struct SceneBuilder {
    pub(crate) handle: SceneHandle,
    pub(crate) geometries: VecMap<Geometry>,
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

    pub(crate) fn as_mut_ptr(&self) -> RTCScene {
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

    pub fn attach(&mut self, geometry: Geometry) -> GeomID {
        let id = unsafe { rtcAttachGeometry(self.handle.ptr, geometry.handle().as_ptr()) };
        assert!(!self.geometries.contains_key(id as usize), "Geometry id already assigned");
        self.geometries.insert(id as usize, geometry);
        GeomID::new(id)
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

    pub fn commit(self) -> Scene {
        unsafe {
            rtcCommitScene(self.handle.ptr);
        }
        Scene {
            handle: self.handle,
            geometries: self.geometries,
        }
    }
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub lower: Vector3<f32>,
    align0: f32,
    pub upper: Vector3<f32>,
    align1: f32,
}

impl Bounds {
    pub fn zero() -> Self {
        Bounds {
            lower: Vector3::zero(),
            align0: 0.0,
            upper: Vector3::zero(),
            align1: 0.0,
        }
    }

    pub fn new(lower: Vector3<f32>, upper: Vector3<f32>) -> Self {
        Bounds {
            lower: lower,
            align0: 0.0,
            upper: upper,
            align1: 0.0,
        }
    }
}

// bitflags! {
//     pub struct IntersectionContextFlags: i32 {
//         const INCOHERENT = RTCIntersectContextFlags_RTC_INTERSECT_CONTEXT_FLAG_INCOHERENT;
//         const COHERENT = RTCIntersectContextFlags_RTC_INTERSECT_CONTEXT_FLAG_COHERENT;
//     }
// }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub tnear: f32,
    pub dir: Vector3<f32>,
    pub tfar: f32,
}

impl Ray {
}

impl Into<RTCRay> for Ray {
    fn into(self) -> RTCRay {
        RTCRay {
            org_x: self.origin.x,
            org_y: self.origin.y,
            org_z: self.origin.z,
            tnear: self.tnear,
            dir_x: self.dir.x,
            dir_y: self.dir.y,
            dir_z: self.dir.z,
            time: 0.0,
            tfar: self.tfar,
            mask: u32::MAX,
            id: 0,
            flags: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Hit {
    pub Ng: Vector3<f32>,
    pub t: f32,
    pub uv: Vector2<f32>,
    pub geom_id: GeomID,
    pub prim_id: GeomID,
    pub instance_id: GeomID,
}

impl Hit {
    pub fn empty() -> Self {
        Hit {
            Ng: Vector3::zero(),
            uv: Vector2::zero(),
            t: 0.0,
            geom_id: GeomID::invalid(),
            prim_id: GeomID::invalid(),
            instance_id: GeomID::invalid(),
        }
    }
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
struct RTCRayHitAligned {
    pub ray: RTCRay,
    pub hit: RTCHit,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
struct RTCRayAligned {
    pub ray: RTCRay,
}

// TODO: Could make new struct for this
fn empty_intersect_context() -> RTCIntersectContext{
    RTCIntersectContext {
        flags: 0,
        filter: None,
        instID: [u32::MAX],
    }
}

impl Scene {
    pub fn bounds(&self) -> Bounds {
        let mut b = Bounds::zero();
        unsafe { rtcGetSceneBounds(self.handle.ptr, &mut b as *mut Bounds as *mut RTCBounds); }
        b
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
                geomID: u32::MAX,
                primID: u32::MAX,
                instID: [u32::MAX],
            }
        };
        unsafe {
            rtcIntersect1(self.handle.ptr,
                &mut context,
                &mut rayhit as *mut RTCRayHitAligned as *mut RTCRayHit);
        }
        Hit {
            Ng: Vector3::new(rayhit.hit.Ng_x, rayhit.hit.Ng_y, rayhit.hit.Ng_z),
            t: rayhit.ray.tnear,
            uv: Vector2::new(rayhit.hit.u, rayhit.hit.v),
            geom_id: GeomID::new(rayhit.hit.geomID),
            prim_id: GeomID::new(rayhit.hit.primID),
            instance_id: GeomID::new(rayhit.hit.instID[0]),
        }
    }

    pub fn occluded(&self, ray: Ray) -> bool {
        let mut r = RTCRayAligned {
            ray: ray.into(),
        };
        let mut context: RTCIntersectContext = empty_intersect_context();
        unsafe {
            rtcOccluded1(self.handle.as_mut_ptr(),
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
    }
}