// sdl2_context.rs
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::ttf::Sdl2TtfContext;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use rand::Rng;

#[derive(Debug)]
struct Windmill {
    pivot: (f32, f32),
    angle: f32,
    speed: f32,
    length: f32,
}

impl Windmill {
    fn new(pivot: (f32, f32), angle: f32, speed: f32, length: f32) -> Self {
        Windmill { pivot, angle, speed, length }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    print!("SDL2 initialized successfully!");
    let mi_molino = Windmill {
        pivot: (0.0, 0.0),
        angle: 0.0,
        speed: 1.0,
        length: 100.0,
    };
    let mi_molino_nuevo = Windmill::new((0.0, 0.0), 0.0, 1.0, 100.0);
    println!("Created windmill: {:?}", mi_molino);
    println!("Molino ordenado: \n{:#?}", mi_molino);
    print!("Molino nuevo: \n{:#?}", mi_molino_nuevo);
    Ok(())
}