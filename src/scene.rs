use std::mem;

use sys::*;

pub struct Scene {
    pub(crate) scene_handle: RTCScene;
    pub device: Device;
}

pub struct SceneBuilder {
    pub(crate) scene_handle: RTCScene;
    pub device: Device;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum SceneBuildQuality {
    Low = RTCBuildQuality_RTC_BUILD_QUALITY_LOW,
    Medium = RTCBuildQuality_RTC_BUILD_QUALITY_MEDIUM,
    High = RTCBuildQuality_RTC_BUILD_QUALITY_HIGH,
}

bitflags! {
    struct SceneFlags: u32 {
        const DYNAMIC = RTCSceneFlags_RTC_SCENE_FLAG_DYNAMIC;
        const COMPACT = RTCSceneFlags_RTC_SCENE_FLAG_COMPACT;
        const ROBUST  = RTCSceneFlags_RTC_SCENE_FLAG_ROBUST;
    }
}

impl SceneBuilder {
    pub fn new(device: Device) {
        let raw = unsafe { rtcNewScene(device.raw_device) };
        SceneBuilder {
            scene_handle: raw,
            device: device
        }
    }

    // pub fn add(&self) {
        
    // }

    pub fn set_build_quality(&self, quality: SceneBuildQuality) {
        unsafe { rtcSetSceneBuildQuality(self.scene_handle, quality); }
    }

    pub fn commit(self) -> Scene {
        unsafe { rtcCommitScene(self.scene_handle); }
        mem::forget(self);
        Scene {
            scene_handle: self.scene_handle,
            device: self.device
        }
    }
}

impl Scene {
    pub fn intersect(&self) {

    }
}

impl Drop for SceneBuilder {
    fn drop(&mut self) {
        unsafe { rtcReleaseScene(self.scene_handle); }
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe { rtcReleaseScene(self.scene_handle); }
    }
}