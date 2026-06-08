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

// --- FUNCIONES DE AYUDA PARA RENDERIZADO EN SDL2 ---

// Traduce del plano cartesiano centrado (Manim-style) a píxeles de pantalla SDL2
fn to_screen(pos: (f32, f32)) -> (i32, i32) {
    let screen_x = (WIDTH as f32 / 2.0) + pos.0;
    let screen_y = (HEIGHT as f32 / 2.0) - pos.1; // El eje Y en SDL2 va hacia abajo
    (screen_x as i32, screen_y as i32)
}

// Dibuja un círculo relleno en SDL2 sin dependencias externas
fn draw_filled_circle(canvas: &mut Canvas<Window>, center: (i32, i32), radius: i32, color: Color) {
    canvas.set_draw_color(color);
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let _ = canvas.draw_point((center.0 + dx, center.1 + dy));
            }
        }
    }
}

// Dibuja el texto del contador sobre cada punto
fn draw_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    text: &str,
    position: (i32, i32),
) {
    if let Ok(surface) = font.render(text).blended(Color::RGB(255, 255, 255)) {
        if let Ok(texture) = texture_creator.create_texture_from_surface(&surface) {
            let query = texture.query();
            let target = Rect::new(
                position.0 - (query.width / 2) as i32,
                position.1 - query.height as i32 - 10, // Separación UP
                query.width,
                query.height,
            );
            let _ = canvas.copy(&texture, None, Some(target));
        }
    }
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
    
    // Cargar fuente (Asegúrate de tener un archivo 'arial.ttf' al lado de tu Cargo.toml)
    let font = ttf_context.load_font("arial.ttf", 14)?;

    // Generar puntos aleatorios (en un rango cartesiano equivalente a tu código de Manim)
    let mut dots: Vec<Dot> = (0..11)
        .map(|_| Dot {
            pos: (
                rand::random::<f32>() * 400.0 - 200.0, // x entre -200 y 200
                rand::random::<f32>() * 400.0 - 200.0, // y entre -200 y 200
            ),
            counter: 0,
        })
        .collect();

    // Forzar que el primer punto sea el pivote inicial y empiece en 1
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
        // 1. Manejo de Eventos
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Calcular Delta Time (tiempo real entre fotogramas)
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // 2. ACTUALIZAR FÍSICA Y LÓGICA DEL WINDMILL
        let prev_angle = windmill.angle;
        windmill.angle = (windmill.angle + windmill.speed * dt) % (2.0 * PI);

        // Buscar cuál será el siguiente impacto analizando los ángulos de todos los puntos
        let mut min_angle_diff = f32::MAX;
        let mut next_pivot_idx: Option<usize> = None;

        for (i, dot) in dots.iter().enumerate() {
            // Ignorar el pivote actual
            if (dot.pos.0 - windmill.pivot.0).abs() < 0.001 && (dot.pos.1 - windmill.pivot.1).abs() < 0.001 {
                continue;
            }

            // Calcular el ángulo del vector del pivote al punto dot
            let dx = dot.pos.0 - windmill.pivot.0;
            let dy = dot.pos.1 - windmill.pivot.1;
            let mut dot_angle = dy.atan2(dx);
            if dot_angle < 0.0 {
                dot_angle += 2.0 * PI;
            }

            // Adaptación a la rotación horaria/antihoraria de tu Manim
            // Buscamos la menor diferencia angular en la dirección del barrido
            let mut diff = dot_angle - prev_angle;
            if diff < 0.0 {
                diff += 2.0 * PI;
            }
            // El molino barre simétricamente por ambos extremos de la línea (PI)
            diff %= PI; 

            if diff < min_angle_diff && diff > 0.0001 {
                min_angle_diff = diff;
                next_pivot_idx = Some(i);
            }
        }

        // Comprobar si en este fotograma el molino cruzó el ángulo del siguiente pivote
        if let Some(idx) = next_pivot_idx {
            let sweep_done = windmill.speed * dt;
            if sweep_done >= min_angle_diff {
                // ¡COLISIÓN DETECTADA! Cambiamos de pivote e incrementamos contador
                windmill.pivot = dots[idx].pos;
                dots[idx].counter += 1;
                // Sincronizamos el ángulo para evitar desfases acumulativos
                windmill.angle = (prev_angle + min_angle_diff) % (2.0 * PI);
            }
        }

        // 3. RENDERIZADO
        canvas.set_draw_color(Color::RGB(15, 15, 25)); // Fondo oscuro
        canvas.clear();

        // Dibujar los Puntos (Dots) y sus números
        for dot in &dots {
            let screen_pos = to_screen(dot.pos);
            
            // Si es el pivote actual lo pintamos Amarillo, si no, Rojo
            let color = if (dot.pos.0 - windmill.pivot.0).abs() < 0.001 && (dot.pos.1 - windmill.pivot.1).abs() < 0.001 {
                Color::RGB(255, 220, 0) // Amarillo
            } else {
                Color::RGB(230, 50, 50)  // Rojo
            };

            draw_filled_circle(&mut canvas, screen_pos, 6, color);
            
            // Renderizar el número entero asignado al punto
            draw_text(
                &mut canvas,
                &texture_creator,
                &font,
                &dot.counter.to_string(),
                screen_pos,
            );
        }

        // Dibujar la Línea del Molino (Windmill Line)
        let start_x = windmill.pivot.0 - (windmill.angle.cos() * windmill.length / 2.0);
        let start_y = windmill.pivot.1 - (windmill.angle.sin() * windmill.length / 2.0);
        let end_x = windmill.pivot.0 + (windmill.angle.cos() * windmill.length / 2.0);
        let end_y = windmill.pivot.1 + (windmill.angle.sin() * windmill.length / 2.0);

        let p1 = to_screen((start_x, start_y));
        let p2 = to_screen((end_x, end_y));

        canvas.set_draw_color(Color::RGB(255, 255, 255)); // Línea blanca
        canvas.draw_line(p1, p2)?;

        canvas.present();
        std::thread::sleep(Duration::from_millis(16)); // Cap a ~60 FPS aproximados
    }

    Ok(())
}