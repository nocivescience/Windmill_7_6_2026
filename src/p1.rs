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

// Configuración de la pantalla
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// #[derive(Clone, Copy, Debug)]


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context =sdl2::ttf::init().map_err(|e| e.to_string())?;
    let window = video_subsystem
        .window("Ventana de prueba", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font("arial.ttf", 14)?;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    }
    canvas.present();
    std::thread::sleep(Duration::from_millis(16)); // Espera para ver la ventana antes de cerrarla
    Ok(())
}