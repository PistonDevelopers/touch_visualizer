extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate touch_visualizer;

use sdl2_window::Sdl2Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::window::{Window, WindowSettings};
use piston::input::RenderEvent;
use touch_visualizer::TouchVisualizer;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("touch", [512; 2])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut touch_visualizer = TouchVisualizer::new();
    let ref mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        touch_visualizer.event(window.size(), &e);
        if let Some(args) = e.render_args() {
            // println!("Render {}", args.ext_dt);
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([1.0; 4], g);
                touch_visualizer.draw(&c, g);
            });
        }
    }
}
