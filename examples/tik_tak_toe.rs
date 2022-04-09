use renderer::{
    render::{Line, Pos},
    shapes::{group::Group, point::Point, text::Text, triangle::Triangle},
    ConsoleRender, ScopeRender,
};

fn main() {
    let ren2 = ScopeRender::new(10, 10);
    ren2.render(Group::new().add(bounding()));
}

#[inline]
fn gridcoord(x: i8, y: i8) -> Pos {
    return Pos::new((-5 + 10 * x) as f32 / 3.0, (-5 + 10 * y) as f32 / 3.0);
}

#[inline]
fn bounding() -> Group {
    Group::new()
        .add(Line::new(gridcoord(1, 0), gridcoord(1, 3)))
        .add(Line::new(gridcoord(2, 3), gridcoord(2, 0)))
        .add(Line::new(gridcoord(3, 1), gridcoord(0, 1)))
        .add(Line::new(gridcoord(0, 2), gridcoord(3, 2)))
}
