extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    rotation: f64,
}

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

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.18, 0.18, 0.18, 1.0];
        const FOREGROUND: [f32; 4] = [1.0, 0.5, 0.5, 1.0];
        const SQUARE_SIZE: f64 = 10.0;

        let square = rectangle::square(0.0, 0.0, SQUARE_SIZE);
        // let (x, y) = (args.window_size[0] * 0.5,
        //               args.window_size[1] * 0.5);

        let rotation = self.rotation;

        const DISTANCE: f64 = 4.0;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);

            for [x0, y0, z0] in &CUBE {
                let x1 = x0 * f64::cos(rotation) + z0 * f64::sin(rotation);
                let z1 = x0 * f64::sin(rotation) - z0 * f64::cos(rotation);

                let x = x1 / (z1 + DISTANCE);
                let y = y0 / (z1 + DISTANCE);
                let [sx, sy] = to_screen(
                    [x, y],
                    args.window_size[0],
                    args.window_size[1]);

                let transform = c
                    .transform
                    .trans(sx, sy)
                    .trans(-SQUARE_SIZE * 0.5, -SQUARE_SIZE * 0.5);
                rectangle(FOREGROUND, square, transform, gl);
            }
        });
    }


    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window =
        WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        e.render_args().map(|ref args| app.render(args));
        e.update_args().map(|ref args| app.update(args));
    }
}
