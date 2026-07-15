use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect; // Asegúrate de importar Rect
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;



fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut event_pump = sdl_context.event_pump()?;

    let window = video_subsystem
        .window("Prueba de Rectángulo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // ESTADO: Una variable que controla si el rectángulo debe dibujarse o no
    let mut deba_dibujar_rectangulo = false;

    'running: loop {
        // 1. MANEJO DE EVENTOS (Solo cambian variables de estado)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                
                // Cuando se PRESIONA Espacio, activamos el dibujo
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    deba_dibujar_rectangulo = true;
                }
                
                // OPCIONAL: Cuando se SUELTA Espacio, lo desactivamos
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    deba_dibujar_rectangulo = false;
                }
                _ => {}
            }
        }

        // 2. LÓGICA DE RENDERIZADO (Dibuja según el estado actual)
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Fondo negro
        canvas.clear();

        // Si el estado dice que es verdadero, lo dibujamos en el buffer
        if deba_dibujar_rectangulo {
            canvas.set_draw_color(Color::RGB(255, 0, 0)); // Color rojo
            let rect = Rect::new(300, 200, 200, 200); // x, y, ancho, alto
            canvas.fill_rect(rect)?;
        }

        // 3. MOSTRAR EN PANTALLA (Volcamos todo el buffer a la pantalla del usuario)
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}