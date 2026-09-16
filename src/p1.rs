use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

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
    let start = (WIDTH as i32 / 2, HEIGHT as i32 / 2);
    let end = (WIDTH as i32 / 2, HEIGHT as i32 / 2 - 100);
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
        // 1. Manejo de Eventos
        for event in event_pump.poll_iter() {
            match event {
                // Si cierran la ventana (clic en la X)
                Event::Quit { .. } => {
                    break 'running;
                }
                // Si se presiona una tecla
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    break 'running; // Rompe el loop si es Espacio
                }
                _ => {}
            }
        }

        

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line(start, end);

        // 2. Renderizado
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Mover el canvas.present() DENTRO del loop para que se actualice la pantalla constantemente
        canvas.present();

        // Evita que el procesador vaya al 100% de uso (aprox. 60 FPS)
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}