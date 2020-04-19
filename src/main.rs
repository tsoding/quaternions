use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
use std::time::Duration;

#[derive(Clone, Copy)]
struct Quaternion {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Quaternion {
    fn from_v3([x, y, z]: [f64; 3]) -> Quaternion {
        Self {
            a: 0.0,
            b: x,
            c: y,
            d: z,
        }
    }

    fn to_v3(self) -> [f64; 3] {
        let Quaternion { a, b, c, d } = self;
        assert!(f64::abs(a) < 1e-6);
        [b, c, d]
    }

    fn rot([x, y, z]: [f64; 3], theta: f64) -> Quaternion {
        let l = f64::sqrt(x * x + y * y + z * z);
        let c = f64::cos(theta * 0.5);
        let s = f64::sin(theta * 0.5);
        Self {
            a: c,
            b: x / l * s,
            c: y / l * s,
            d: z / l * s
        }
    }

    fn product(self,
               Quaternion { a:a2, b: b2, c: c2, d: d2 }: Quaternion) -> Quaternion {
        let Quaternion { a:a1, b: b1, c: c1, d: d1 } = self;
        Self {
            a: a1 * a2 - b1 * b2 - c1 * c2 - d1 * d2,
            b: a1 * b2 + b1 * a2 + c1 * d2 - d1 * c2,
            c: a1 * c2 - b1 * d2 + c1 * a2 + d1 * b2,
            d: a1 * d2 + b1 * c2 - c1 * b2 + d1 * a2,
        }
    }

    fn recip(self) -> Quaternion {
        let Quaternion { a, b, c, d } = self;
        let q = a * a + b * b + c * c + d * d;
        Self {
            a:  a / q,
            b: -b / q,
            c: -c / q,
            d: -d / q,
        }
    }
}

const VS: [[f64; 3]; 8] = [
    [-1.0,  1.0,  1.0],  // 0
    [ 1.0,  1.0,  1.0],  // 1
    [ 1.0,  1.0, -1.0],  // 2
    [-1.0,  1.0, -1.0],  // 3

    [-1.0, -1.0,  1.0],  // 4
    [ 1.0, -1.0,  1.0],  // 5
    [ 1.0, -1.0, -1.0],  // 6
    [-1.0, -1.0, -1.0],  // 7
];

const LS: [[usize; 2]; 12] = [
    // Top Side
    [0, 1],
    [1, 2],
    [2, 3],
    [3, 0],

    // Bottom side
    [4, 5],
    [5, 6],
    [6, 7],
    [7, 4],

    // Vertical
    [0, 4],
    [1, 5],
    [2, 6],
    [3, 7]
];

fn project([x, y, z]: [f64; 3], r: f64) -> [f64; 2] {
    return [x * r / z, y / z];
}

fn to_screen([x0, y0]: [f64; 2], w: f64, h: f64) -> [f64; 2] {
    let half_w = w * 0.5;
    let half_h = h * 0.5;
    let x = x0 * half_w + half_w;
    let y = y0 * half_h + half_h;
    [x, y]
}

fn epic_rotate(p: [f64; 3], theta: f64) -> [f64; 3] {
    let pq = Quaternion::from_v3(p);
    let rotq = Quaternion::rot([1.0, 1.0, 1.0], theta);
    rotq.product(pq).product(rotq.recip()).to_v3()
}

#[allow(dead_code)]
fn rotate_y([x0, y0, z0]: [f64; 3], theta: f64) -> [f64; 3] {
    let x1 = x0 * f64::cos(theta) + z0 * f64::sin(theta);
    let z1 = x0 * f64::sin(theta) - z0 * f64::cos(theta);
    return [x1, y0, z1];
}

fn translate([x0, y0, z0]: [f64; 3], [x1, y1, z1]: [f64; 3]) -> [f64; 3] {
    return [x0 + x1, y0 + y1, z0 + z1];
}

const DISTANCE: f64 = 3.0;
const BACKGROUND: Color = Color::RGB(18, 18, 18);
const FOREGROUND: Color = Color::RGB(255, 150, 150);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Quaternions", 800, 600)
        .position_centered()
        .resizable()
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
        let (w, h) = canvas.window().size();
        for [l1, l2] in &LS {
            let r = h as f64 / w as f64;
            let [sx0, sy0] = to_screen(project(translate(epic_rotate(VS[*l1], rotation), [0.0, 0.0, DISTANCE]), r),
                                       w as f64, h as f64);
            let [sx1, sy1] = to_screen(project(translate(epic_rotate(VS[*l2], rotation), [0.0, 0.0, DISTANCE]), r),
                                       w as f64, h as f64);
            canvas.draw_line(Point::new(sx0 as i32, sy0 as i32),
                             Point::new(sx1 as i32, sy1 as i32))?;
        }

        canvas.present();

        const FPS: u32 = 30;
        let dt = 1.0 / FPS as f64;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        rotation += 2.0 * dt;

    }

    Ok(())
}
