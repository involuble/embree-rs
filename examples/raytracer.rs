#[macro_use]
extern crate derive_more;
extern crate minifb;
extern crate num_traits;
extern crate cgmath;
extern crate fern;

extern crate embree;

mod common;

use std::f32;

use common::*;
use cgmath::*;
use minifb::*;
use embree::*;
use num_traits::clamp;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

const CAMERA_POS: [f32; 3] = [3.0, 4.0, 6.0];

/// Vector pointing towards the sun
const SUN_DIR: [f32; 3] = [0.1, 1.0, -0.3];

const AMBIENT: f32 = 0.1;

const COLOURS: [Colour; 9] = [
    Colour { r: 1.0, g: 1.0, b: 1.0 },
    Colour { r: 1.0, g: 0.6, b: 0.9 },
    Colour { r: 0.99, g: 0.7, b: 0.35 },
    Colour { r: 0.0, g: 0.8, b: 0.8 },
    Colour { r: 0.2, g: 0.13, b: 0.63 },
    Colour { r: 0.86, g: 0.39, b: 0.4 },
    Colour { r: 0.25, g: 0.55, b: 0.81 },
    Colour { r: 0.05, g: 0.75, b: 0.59 },
    Colour { r: 0.32, g: 0.15, b: 0.74 },
];

const CUBE_VERTICES: [Point3<f32>; 8] = [
    Point3 { x: -1.0, y: -1.0, z: -1.0 },
    Point3 { x: -1.0, y: -1.0, z:  1.0 },
    Point3 { x: -1.0, y:  1.0, z: -1.0 },
    Point3 { x: -1.0, y:  1.0, z:  1.0 },
    Point3 { x:  1.0, y: -1.0, z: -1.0 },
    Point3 { x:  1.0, y: -1.0, z:  1.0 },
    Point3 { x:  1.0, y:  1.0, z: -1.0 },
    Point3 { x:  1.0, y:  1.0, z:  1.0 },
];

const CUBE_INDICES: [Triangle; 12] = [
    // Left side
    Triangle { v0: 0, v1: 1, v2: 2 },
    Triangle { v0: 1, v1: 3, v2: 2 },
    // Right side
    Triangle { v0: 4, v1: 6, v2: 5 },
    Triangle { v0: 5, v1: 6, v2: 7 },
    // Bottom side
    Triangle { v0: 0, v1: 4, v2: 1 },
    Triangle { v0: 1, v1: 4, v2: 5 },
    // Top side
    Triangle { v0: 2, v1: 3, v2: 6 },
    Triangle { v0: 3, v1: 7, v2: 6 },
    // Front side
    Triangle { v0: 0, v1: 2, v2: 4 },
    Triangle { v0: 2, v1: 6, v2: 4 },
    // Back side
    Triangle { v0: 1, v1: 5, v2: 3 },
    Triangle { v0: 3, v1: 5, v2: 7 },
];

pub fn build_scene(device: &Device) -> Scene {
    let mut scene = SceneBuilder::new(device);

    let plane_v = vec![
        Point3::new(-10.0, -2.0, -10.0),
        Point3::new(-10.0, -2.0,  10.0),
        Point3::new( 10.0, -2.0, -10.0),
        Point3::new( 10.0, -2.0,  10.0),
    ];
    let plane_i = vec![Triangle::new(0, 1, 2), Triangle::new(1, 3, 2)];

    let plane = TriangleMesh::new(device, plane_i, plane_v);
    scene.attach(plane.build());

    let cube = TriangleMesh::new(device, Vec::from(CUBE_INDICES.as_ref()), Vec::from(CUBE_VERTICES.as_ref()));
    let _ = scene.attach(cube.build());

    let sphere = UserGeometry::new_realloc(device, vec![Sphere { center: Point3::new(-3.0, 0.0, 0.0), radius: 1.0 }]);
    scene.attach_user_geometry(sphere.build());

    scene.set_build_quality(BuildQuality::Medium);
    scene.set_flags(SceneFlags::ROBUST | SceneFlags::COMPACT);

    scene.build()
}

pub fn render_scene(buffer: &mut Vec<u32>, scene: &Scene, camera: &Camera) {
    let sun_dir = Vector3::from(SUN_DIR).normalize();

    buffer.iter_mut().enumerate().for_each(|(index, value)| {
        let x = index % WIDTH;
        let y = index / WIDTH;
        let x = (x as f32 + 0.5) / (WIDTH as f32);
        let y = (y as f32 + 0.5) / (HEIGHT as f32);

        let ray = camera.get_ray(x, y);
        let hit = scene.intersect(ray);
        if hit.is_hit() {
            let hit_pos = ray.point_at_dist(hit.t);

            let shadow_ray = Ray::new(hit_pos, sun_dir, 1e-5, f32::MAX);
            let shadow_hit = scene.intersect(shadow_ray);
            let shadowing = if shadow_hit.is_hit() { 0.0 } else { 1.0 };

            let lighting: f32 = AMBIENT + shadowing * (1.0 - AMBIENT) * clamp(dot(hit.Ng, sun_dir), 0.0, 1.0);
            debug_assert!(lighting <= 1.0);
            let colour = COLOURS[hit.geom_id.unwrap() as usize] * lighting;
            *value = colour.to_rgba8();
        }
    })
}

fn main() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Error) // Trace is default
        .chain(std::io::stdout())
        .apply()
        .expect("Unable to initialize logger");
    
    let mut buffer: Vec<u32> = vec![0; WIDTH*HEIGHT];

    let mut window = Window::new("Embree Raytracer Example", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Unable to create window");

    let device = Device::new();

    let scene = build_scene(&device);

    let aspect_ratio = (WIDTH as f32) / (HEIGHT as f32);
    let camera = Camera::new(Point3::from(CAMERA_POS), Point3::origin(), Vector3::unit_y(), Deg(60.0), aspect_ratio);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_keys().map(|keys| {
            for k in keys {
                match k {
                    Key::W => (),
                    _ => (),
                }
            }
        });

        render_scene(&mut buffer, &scene, &camera);

        window.update_with_buffer(&buffer).unwrap();
    }
}