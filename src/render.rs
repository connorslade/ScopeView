#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Pos,
    pub end: Pos,
    pub step: f32,
}

impl Render for Line {
    fn render(&self) -> Vec<Line> {
        vec![*self]
    }
}

impl Render for Vec<Line> {
    fn render(&self) -> Vec<Line> {
        self.to_owned()
    }
}

pub trait Render {
    fn render(&self) -> Vec<Line>;
}

impl Line {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self {
            start,
            end,
            step: 0.2,
        }
    }

    pub fn step(self, step: f32) -> Self {
        Self { step, ..self }
    }

    pub fn distance(&self) -> f32 {
        ((self.end.x - self.start.x).powf(2.0) + (self.end.y - self.start.y).powf(2.0)).sqrt()
    }
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
