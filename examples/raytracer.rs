#[macro_use]
extern crate derive_more;
extern crate minifb;
extern crate num_traits;

use minifb::*;
use num_traits::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

#[derive(Debug, Copy, Clone, Constructor, Add, Mul, AddAssign)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn to_rgba8(&self) -> u32 {
        let r: u32 = clamp(self.r * 255.0, 0.0, 255.0) as u32;
        let g: u32 = clamp(self.g * 255.0, 0.0, 255.0) as u32;
        let b: u32 = clamp(self.b * 255.0, 0.0, 255.0) as u32;
        0xFF | r << 16 | g << 8 | b
    }
}

pub fn render_scene(buffer: &mut Vec<u32>) {
    buffer[5400] = Colour::new(0.0, 0.0, 1.0).to_rgba8();
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH*HEIGHT];

    let mut window = Window::new("Embree Raytracer Example", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("Unable to create window: {}", e));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_keys().map(|keys| {
            for k in keys {
                match k {
                    Key::W => (),
                    _ => (),
                }
            }
        });

        render_scene(&mut buffer);

        window.update_with_buffer(&buffer).unwrap();
    }
}