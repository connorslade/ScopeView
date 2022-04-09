use crate::render::{Line, Pos, Render};

pub struct Circle {
    radius: f32,
    center: Pos,
}

impl Render for Circle {
    fn render(&self) -> Vec<Line> {
        unimplemented!()
    }
}
