use crate::render::{Line, Pos, Render};

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

    pub fn offset(self, pos: Pos) -> Self {
        Self {
            lines: self
                .lines
                .into_iter()
                .map(|x| {
                    Line::new(
                        Pos::new(x.start.x + pos.x, x.start.y + pos.y),
                        Pos::new(x.end.x + pos.x, x.end.y + pos.y),
                    )
                    .cool_down(x.cool_down)
                })
                .collect::<Vec<_>>(),
        }
    }
}
