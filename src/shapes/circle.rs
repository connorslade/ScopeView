use std::f32::consts::PI;

use crate::render::{Line, Pos, Render};

#[derive(Debug, Clone)]
pub struct Circle {
    radius: f32,
    step: f32,
    lines: Vec<Line>,
}
impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle {
            radius,
            step: 1.0,
            lines: Vec::new(),
        }
    }

    pub fn generate(&mut self) {
        let steps = (2.0 * self.radius * PI / self.step) + 1.0;
        let angle = 2.0 * PI / steps;

        let save = Pos::new(
            (angle * 0 as f32).cos() * self.radius,
            (angle * 0 as f32).sin() * self.radius,
        );
        let mut a = save;
        let mut b;
        self.lines.push(Line::new(Pos::new(0.0, 0.0), a));

        for itr in 1..=steps as i32 {
            b = Pos::new(
                (angle * itr as f32).cos() * self.radius,
                (angle * itr as f32).sin() * self.radius,
            );
            self.lines.push(Line::new(a, b).cool_down(false));
            a = b;
        }
        self.lines.push(Line::new(a, save).cool_down(false));
    }
}

impl Render for Circle {
    fn render(&self) -> Vec<Line> {
        // self.points
        self.lines.to_owned()
    }
}
