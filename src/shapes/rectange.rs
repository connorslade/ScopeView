use crate::render::{Line, Pos, Render};

pub struct Rectangle {}

impl Render for Rectangle {
    fn render(&self) -> Vec<Line> {
        Vec::new()
    }
}

impl Rectangle {
    pub fn new(p1: Pos, p2: Pos) -> Self {
        Self {}
    }
}
