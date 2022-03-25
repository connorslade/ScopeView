use crate::render::{Line, Render};

pub struct Group {
    lines: Vec<Line>,
}

impl Render for Group {
    fn render(&self) -> Vec<Line> {
        self.lines.to_owned()
    }
}

impl Group {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn add(self, shape: impl Render) -> Self {
        let mut lines = self.lines;
        lines.extend(shape.render());

        Self { lines }
    }
}
