use crate::render::{Line, Pos, Render};

pub struct Point(pub Pos);

impl Render for Point {
    fn render(&self) -> Vec<Line> {
        vec![Line {
            start: self.0,
            end: self.0,
            step: 1.0,
        }]
    }
}
