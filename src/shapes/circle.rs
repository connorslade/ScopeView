use std::f32::consts::PI;

use crate::render::{Line, Pos, Render};

#[derive(Debug)]
pub struct Circle {
    radius: f32,
    center: Pos,
    step: f32,
    lines: Vec<Line>,
}
impl Circle {
    pub fn new(radius: f32, center: Pos) -> Circle {
        Circle {
            radius,
            center,
            step: 1.0,
            lines: Default::default(),
        }
    }

    pub fn generate(&mut self) {
        let steps = (2.0 * self.radius * PI / self.step) + 1.0;
        let angle = 2.0 * PI / steps;

        let mut a: Pos = self.center;
        let mut b: Pos = self.center;

        for itr in 0..steps as i32 {
            b = Pos::new(
                self.center.x + (angle * itr as f32).cos() * self.radius,
                self.center.y + (angle * itr as f32).sin() * self.radius,
            );
            self.lines.push(Line::new(a, b));
            a = b;
        }
    }
}

impl Render for Circle {
    fn render(&self) -> Vec<Line> {
        // self.points
        self.lines.to_owned()
    }
}
