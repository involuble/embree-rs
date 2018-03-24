use sys::*;

use device::Device;

pub struct Scene {
    // TODO
    #[allow(dead_code)]
    pub(crate) handle: SceneHandle,
}

pub struct SceneBuilder {
    pub(crate) handle: SceneHandle,
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
        }
    }

    // pub fn add(&self) {
        
    // }

    pub fn set_build_quality(&self, quality: BuildQuality) {
        unsafe { rtcSetSceneBuildQuality(self.handle.ptr, quality.into()); }
    }

    pub fn commit(self) -> Scene {
        unsafe {
            rtcCommitScene(self.handle.ptr);
        }
        Scene {
            handle: self.handle,
        }
    }
}

impl Scene {
    // pub fn intersect(&self) {

    // }
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
    struct SceneFlags: i32 {
        const DYNAMIC = RTCSceneFlags_RTC_SCENE_FLAG_DYNAMIC;
        const COMPACT = RTCSceneFlags_RTC_SCENE_FLAG_COMPACT;
        const ROBUST  = RTCSceneFlags_RTC_SCENE_FLAG_ROBUST;
    }
}