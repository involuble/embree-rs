use std::mem;
use std::u32;

use sys::*;

use cgmath::*;

use device::Device;
use geometry::*;

pub struct Scene {
    // TODO
    pub(crate) handle: SceneHandle,
    pub(crate) geometries: Vec<Box<Geometry>>,
}

pub struct SceneBuilder {
    pub(crate) handle: SceneHandle,
    pub(crate) geometries: Vec<Box<Geometry>>,
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
            geometries: Vec::new(),
        }
    }

    pub fn attach(&mut self, geometry: Box<Geometry>) -> ID {
        let id: u32;
        {
            let geom_handle = geometry.get_handle();
            id = unsafe { rtcAttachGeometry(self.handle.ptr, geom_handle.ptr) };
        }
        // FIXME
        self.geometries.push(geometry);
        ID::new(id)
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

// TODO: align(16)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub lower: Vector3<f32>,
    pub upper: Vector3<f32>,
}

// bitflags! {
//     pub struct IntersectionContextFlags: i32 {
//         const INCOHERENT = RTCIntersectContextFlags_RTC_INTERSECT_CONTEXT_FLAG_INCOHERENT;
//         const COHERENT = RTCIntersectContextFlags_RTC_INTERSECT_CONTEXT_FLAG_COHERENT;
//     }
// }

// TODO: align(16)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub tnear: f32,
    pub dir: Vector3<f32>,
    time: f32,
    pub tfar: f32,
    mask: u32,
    id: u32,
    flags: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub Ng: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub prim_id: ID,
    pub geom_id: ID,
    pub instance_id: ID,
}

impl Hit {
    pub fn new() -> Self {
        Hit {
            Ng: Vector3::zero(),
            uv: Vector2::zero(),
            prim_id: ID::invalid(),
            geom_id: ID::invalid(),
            instance_id: ID::invalid(),
        }
    }
}

// TODO: align(16)
// TODO: add tests to ensure the field offsets stay the same
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RayHit {
    pub ray: Ray,
    pub hit: Hit,
}

// TODO: Could make new struct for this
fn new_intersect_context() -> RTCIntersectContext{
    RTCIntersectContext {
        flags: 0,
        filter: None,
        instID: [u32::MAX],
    }
}

impl Scene {
    pub fn bounds(&self) -> Bounds {
        let mut b = RTCBounds {
            lower_x: 0.0,
            lower_y: 0.0,
            lower_z: 0.0,
            align0: 0.0,
            upper_x: 0.0,
            upper_y: 0.0,
            upper_z: 0.0,
            align1: 0.0,
        };
        unsafe { rtcGetSceneBounds(self.handle.ptr, &mut b); }
        Bounds {
            lower: Vector3::new(b.lower_x, b.lower_y, b.lower_z),
            upper: Vector3::new(b.upper_x, b.upper_y, b.upper_z),
        }
    }

    pub fn intersect(&self, ray: Ray) -> RayHit {
        let mut context: RTCIntersectContext = new_intersect_context();
        let mut rayhit = RayHit {
            ray: ray,
            hit: Hit::new(),
        };
        unsafe {
            rtcIntersect1(self.handle.ptr,
                &mut context,
                mem::transmute::<*mut RayHit, *mut RTCRayHit>(&mut rayhit));
        }
        rayhit
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