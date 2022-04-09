use rand::Rng;
use renderer::{
    render::{Line, Pos, Render},
    shapes::{group::Group, point::Point, triangle::Triangle},
    ScopeRender,
};

fn main() {
    let ren = ScopeRender::new(10, 10);

    let mut flap_box = Vec::new();

    for i in 0..2 {
        flap_box.push(next_box());
    }

    ren.render(bounding());
}

fn next_box() -> FlappyBox {
    let mut rng = rand::thread_rng();
    FlappyBox::new(Pos::new(0.0, 0.0), Pos::new(2.0, 4.0))
}

#[inline]
fn bounding() -> Group {
    Group::new()
        .add(Line::new(Pos::new(-5.0, -5.0), Pos::new(5.0, -5.0)))
        .add(Line::new(Pos::new(5.0, -5.0), Pos::new(5.0, 5.0)))
        .add(Line::new(Pos::new(5.0, 5.0), Pos::new(-5.0, 5.0)))
        .add(Line::new(Pos::new(-5.0, 5.0), Pos::new(-5.0, -5.0)))
}

#[derive(Debug, Clone, Copy)]
struct FlappyBox {
    p1: Pos,
    p2: Pos,
}

impl FlappyBox {
    fn new(p1: Pos, p2: Pos) -> Self {
        Self { p1, p2 }
    }
}

impl Render for FlappyBox {
    fn render(&self) -> Vec<Line> {
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;

        Group::new()
            .add(Line::new(Pos::new(x1, y1), Pos::new(x1, y2)))
            .add(Line::new(Pos::new(x1, y2), Pos::new(x2, y2)))
            .add(Line::new(Pos::new(x2, y2), Pos::new(x2, y1)))
            .add(Line::new(Pos::new(x2, y1), Pos::new(x1, y1)))
            .render()
    }
}
