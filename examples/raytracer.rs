#[macro_use]
extern crate derive_more;
extern crate minifb;
extern crate num_traits;
extern crate cgmath;

extern crate embree;

mod common;

use common::*;
use cgmath::*;
use minifb::*;
use embree::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

const COLOURS: [Colour; 8] = [
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

    let cube = TriangleMesh::new(device, Vec::from(CUBE_INDICES.as_ref()), Vec::from(CUBE_VERTICES.as_ref()));
    scene.attach(cube.commit());

    scene.set_build_quality(BuildQuality::Medium);
    scene.set_flags(SceneFlags::ROBUST);

    scene.commit()
}

pub fn render_scene(buffer: &mut Vec<u32>, scene: &Scene) {
    buffer[5400] = COLOURS[0].to_rgba8();
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH*HEIGHT];

    let mut window = Window::new("Embree Raytracer Example", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("Unable to create window: {}", e));

    let device = Device::new();

    let scene = build_scene(&device);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_keys().map(|keys| {
            for k in keys {
                match k {
                    Key::W => (),
                    _ => (),
                }
            }
        });

        render_scene(&mut buffer, &scene);

        window.update_with_buffer(&buffer).unwrap();
    }
}