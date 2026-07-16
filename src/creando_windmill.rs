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

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Clone, Copy, Debug)]
struct Dot {
    pos: (f32, f32), // Coordenadas matemáticas centradas en (0,0)
    counter: u32,
}

struct Windmill {
    pivot: (f32, f32),
    angle: f32,
    speed: f32,
    length: f32,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let window = video_subsystem
    .window("IMOP 2011 - Windmill Problem en Rust", WIDTH, HEIGHT)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font("arial.ttf", 14)?;
    let mut dots: Vec<Dot>= (0..11)
    .map(|_| Dot{
        pos: (
            rand::random::<f32>() * 400.0 - 200.0, // x entre -200 y 200
            rand::random::<f32>() * 400.0 - 200.0, // y entre -200 y 200
        ),
        counter: 0,
    })
    .collect();
    dots[0].counter = 1;
    let mut windmill = Windmill {
        pivot: dots[0].pos,
        angle: PI / 6.0,
        speed: 1.5, // Velocidad angular (radianes por segundo)
        length: 700.0,
    };
    let mut event_pump = sdl_context.event_pump()?;
    let mut last_time = Instant::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape), .. } | Event::KeyDown {keycode: Some(Keycode::U), .. } => break 'running, _ => {}
            }
        }
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();
    }
    Ok(())
}