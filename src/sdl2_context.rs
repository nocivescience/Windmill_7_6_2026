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

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    
    let window = video_subsystem
        .window("Ventana de prueba", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
        
    let mut canvas = window.into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    // 1. Iniciamos el cronómetro JUSTO antes de que empiece el bucle
    let tiempo_inicio = Instant::now();

    println!("¡Bucle iniciado! Se cerrará automáticamente en 5 segundos...");

    'running: loop {
        // 2. REVISIÓN DE TIEMPO: ¿Cuánto tiempo ha pasado desde el inicio?
        if tiempo_inicio.elapsed() >= Duration::from_secs(5) {
            println!("¡Tiempo límite alcanzado! Cerrando programa de forma segura...");
            break 'running; 
        }

        // Procesamos eventos normales (por si quieres cerrarlo antes con ESC)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Dibujamos el fondo
        canvas.set_draw_color(Color::RGB(20, 30, 45)); 
        canvas.clear(); 
        canvas.present();

        // 3. SEGURO ANTI-CONGELAMIENTO: 
        // Dormir el hilo 16 milisegundos le da un respiro enorme al procesador 
        // y nos mantiene cerca de los 60 FPS estables.
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("Ventana cerrada con éxito.");
    Ok(())
}