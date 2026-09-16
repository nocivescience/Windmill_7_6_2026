use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
// use sdl2::ttf::Sdl2TtfContext;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
// use rand::Rng;

// Configuración de la pantalla
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;


struct Windmill {
    pivot: (f32, f32),
    angle: f32,
    speed: f32,
    length: f32,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    // El EventPump es crucial para capturar teclado, ratón y cierres de ventana
    let mut event_pump = sdl_context.event_pump()?;

    let window = video_subsystem
        .window("Ventana de prueba", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut windmill = Windmill {
        pivot: (WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
        angle: 0.0,
        speed: 1.0,
        length: 100.0,
    };

    'running: loop {
        
    }
}