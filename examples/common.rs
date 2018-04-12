#![allow(dead_code)]

use cgmath::*;
use num_traits::*;

pub struct Camera {
    pub origin: Point3<f32>,
    forward: Vector3<f32>,
    down: Vector3<f32>,
    right: Vector3<f32>,
    upper_left: Vector3<f32>,
}

impl Camera {
    pub fn new(origin: Point3<f32>, look_at: Point3<f32>, up: Vector3<f32>, fov: Deg<f32>, aspect_ratio: f32) -> Self {
        let forward = (look_at - origin).normalize();
        let fov: Rad<f32> = fov.into();
        let half_height = (fov.0 / 2.0).tan();
        let half_width  = half_height * aspect_ratio;
        let right = forward.cross(up).normalize();
        let down = forward.cross(right).normalize();
        Camera {
            origin: origin,
            forward: forward,
            down: down,
            right: right,
            upper_left: -half_width*right + half_height*down,
        }
    }

    pub fn get_ray(&self, coord: Vector2<f32>) {
        let _ = coord.extend(1.0);
    }
}

#[derive(Debug, Copy, Clone, Constructor, Add, Mul, AddAssign)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Colour {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }

    pub fn to_rgba8(&self) -> u32 {
        let r: u32 = clamp(self.r.powf(1.0/2.2) * 255.0, 0.0, 255.0) as u32;
        let g: u32 = clamp(self.g.powf(1.0/2.2) * 255.0, 0.0, 255.0) as u32;
        let b: u32 = clamp(self.b.powf(1.0/2.2) * 255.0, 0.0, 255.0) as u32;
        0xFF | r << 16 | g << 8 | b
    }
}