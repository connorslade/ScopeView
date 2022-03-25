#[derive(Debug, Clone, Copy)]
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
