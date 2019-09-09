use std::ffi::c_void;

use sys::*;

use vec_map::*;

use common::*;
use device::Device;
use geometry::*;
use ray::*;

pub struct Scene {
    handle: SceneHandle,
    geometries: VecMap<Box<dyn Geometry>>,
}

pub struct SceneBuilder {
    handle: SceneHandle,
    geometries: VecMap<Box<dyn Geometry>>,
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

    pub fn attach<T: Geometry>(&mut self, geometry: T) -> GeomID {
        let id = unsafe { rtcAttachGeometry(self.handle.ptr, geometry.handle().as_raw_ptr()) };
        assert!(!self.geometries.contains_key(id as usize), "Geometry id already assigned");

        let mut boxed = Box::new(geometry);
        
        let user_ptr = boxed.as_mut() as *mut T as *mut c_void;
        unsafe { rtcSetGeometryUserData(boxed.handle().as_raw_ptr(), user_ptr); }
        
        boxed.set_geom_id(id);
        boxed.bind_buffers();
        boxed.handle_mut().commit();
        
        self.geometries.insert(id as usize, boxed as Box<dyn Geometry>);
        GeomID::new(id)
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
fn empty_intersect_context() -> RTCIntersectContext {
    RTCIntersectContext {
        flags: 0,
        filter: None,
        instID: [INVALID_ID],
    }
}

// struct GeometryQueryHandle<'a> {
//     ptr: RTCGeometry,
//     phantom: ::std::marker::PhantomData<&'a ()>,
// }

// impl<'a> GeometryQueryHandle<'a> {
//     fn interpolate() {}
// }

impl Scene {
    pub fn bounds(&self) -> Bounds {
        let mut b = Bounds::zero();
        unsafe { rtcGetSceneBounds(self.handle.ptr, b.as_raw_ptr()); }
        b
    }

    pub fn intersect(&self, rayhit: &mut RayHit) {
        let mut context: RTCIntersectContext = empty_intersect_context();
        unsafe {
            rtcIntersect1(self.handle.as_ptr(),
                &mut context,
                rayhit.as_raw_ptr());
        }
    }

    pub fn occluded(&self, ray: &mut Ray) -> bool {
        let mut context: RTCIntersectContext = empty_intersect_context();
        unsafe {
            rtcOccluded1(self.handle.as_ptr(),
                &mut context,
                ray as *mut Ray as *mut RTCRay);
        }
        ray.tfar == std::f32::NEG_INFINITY
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