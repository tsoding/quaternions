use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::time::Duration;

const CUBE: [[f64; 3]; 8] = [
    [-1.0,  1.0,  1.0],
    [ 1.0,  1.0,  1.0],
    [ 1.0,  1.0, -1.0],
    [-1.0,  1.0, -1.0],
    [-1.0, -1.0,  1.0],
    [ 1.0, -1.0,  1.0],
    [ 1.0, -1.0, -1.0],
    [-1.0, -1.0, -1.0],
];

fn to_screen([x0, y0]: [f64; 2], w: f64, h: f64) -> [f64; 2] {
    let half_w = w * 0.5;
    let half_h = h * 0.5;
    let x = x0 * half_w + half_w;
    let y = y0 * half_h + half_h;
    return [x, y];
}

const DISTANCE: f64 = 4.0;
const BACKGROUND: Color = Color::RGB(18, 18, 18);
const FOREGROUND: Color = Color::RGB(255, 150, 150);
const SQUARE_SIZE: f64 = 10.0;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Quaternions", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut rotation: f64 = 0.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }

        }

        canvas.set_draw_color(BACKGROUND);
        canvas.clear();

        canvas.set_draw_color(FOREGROUND);
        for [x0, y0, z0] in &CUBE {
            let x1 = x0 * f64::cos(rotation) + z0 * f64::sin(rotation);
            let z1 = x0 * f64::sin(rotation) - z0 * f64::cos(rotation);

            let x = x1 / (z1 + DISTANCE);
            let y = y0 / (z1 + DISTANCE);
            let (w, h) = canvas.window().size();
            let [sx, sy] = to_screen([x, y], w as f64, h as f64);

            canvas.fill_rect(Rect::new(
                (sx - SQUARE_SIZE * 0.5) as i32,
                (sy - SQUARE_SIZE * 0.5) as i32,
                SQUARE_SIZE as u32,
                SQUARE_SIZE as u32))?;
        }

        canvas.present();

        const FPS: u32 = 30;
        let dt = 1.0 / FPS as f64;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        rotation += 2.0 * dt;

    }

    Ok(())
}
