//! A library for visualizing input touches with Piston-Graphics

extern crate graphics;
extern crate input;

use std::collections::HashMap;

use input::{GenericEvent, Touch};
use graphics::{Context, Graphics};

/// Stores touch values.
pub type TouchValues = HashMap<(i64, i64), ([f64; 2], f64)>;

/// Visualizes touch input.
#[derive(Clone, Debug)]
pub struct TouchVisualizer {
    /// Stores touch values.
    pub touch_values: TouchValues,
    window_size: Option<[u32; 2]>,
}

impl TouchVisualizer {
    /// Creates a new touch visualizer.
    pub fn new() -> TouchVisualizer {
        TouchVisualizer {
            touch_values: HashMap::new(),
            window_size: None,
        }
    }

    /// Draws touch visual graphics.
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::ellipse::Ellipse;

        let window_size = if let Some(size) = self.window_size {
            size
        } else {
            return;
        };
        let color = [1.0, 0.0, 0.0, 0.4];
        let radius = 20.0;
        let mut draw = |pos: [f64; 2], pressure: f64| {
            let r = radius * pressure;
            let x = pos[0] * window_size[0] as f64 - r;
            let y = pos[1] * window_size[1] as f64 - r;
            let w = 2.0 * r;
            Ellipse::new(color)
                .resolution(16)
                .draw([x, y, w, w], &c.draw_state, c.transform, g);
        };
        for &(pos, pressure) in self.touch_values.values() {
            draw(pos, pressure);
        }
    }

    /// Handles event.
    pub fn event<E: GenericEvent, S: Into<[u32; 2]>>(&mut self, window_size: S, e: &E) {
        self.window_size = Some(window_size.into());
        if let Some(args) = e.touch_args() {
            match args.touch {
                Touch::Start | Touch::Move => {
                    self.touch_values
                        .insert((args.device, args.id), (args.position(), args.pressure()));
                }
                Touch::End | Touch::Cancel => {
                    self.touch_values.remove(&(args.device, args.id));
                }
            }
        }
    }
}
