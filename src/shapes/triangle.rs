use crate::render::{Line, Pos, Render};

pub struct Triangle {
    pub point_a: Pos,
    pub point_b: Pos,
    pub point_c: Pos,
    pub step: f32,
}

impl Render for Triangle {
    fn render(&self) -> Vec<Line> {
        let aline = Line {
            start: self.point_a,
            end: self.point_b,
            step: self.step,
            cool_down: true,
        };

        let bline = Line {
            start: self.point_b,
            end: self.point_c,
            step: self.step,
            cool_down: true,
        };

        let cline = Line {
            start: self.point_c,
            end: self.point_a,
            step: self.step,
            cool_down: true,
        };

        vec![aline, bline, cline]
    }
}
