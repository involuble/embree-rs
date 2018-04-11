#[macro_use]
extern crate derive_more;
extern crate minifb;
extern crate num_traits;

use minifb::*;
use num_traits::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

const COLOURS: &[Colour] = [
    Colour::rgb(255, 159, 243),
    Colour::rgb(254, 202, 87),
    Colour::rgb(0, 210, 211),
    Colour::rgb(52, 31, 151),
    Colour::rgb(238, 82, 83),
    Colour::rgb(46, 134, 222),
    Colour::rgb(16, 172, 132),
    Colour::rgb(95, 39, 205),
    Colour::rgb(34, 47, 62),
    Colour::rgb(10, 189, 227),
    Colour::rgb(243, 104, 224),
];

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

pub fn build_scene() -> () {
    ()
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