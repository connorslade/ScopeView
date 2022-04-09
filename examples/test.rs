use renderer::{
    render::{Line, Pos},
    shapes::{group::Group, point::Point, text::Text},
    ConsoleRender, ScopeRender,
};

fn main() {
    let ren2 = ScopeRender::new(10, 10);
    ren2.render(
        Group::new()
            .add(Line::new(Pos::new(-5.0, -5.0), Pos::new(5.0, -5.0)))
            .add(Line::new(Pos::new(5.0, -5.0), Pos::new(5.0, 5.0)))
            .add(Line::new(Pos::new(5.0, 5.0), Pos::new(-5.0, 5.0)))
            .add(Line::new(Pos::new(-5.0, 5.0), Pos::new(-5.0, -5.0))),
    );

    // let smile = Group::new()
    //     .add(Line::new(Pos::new(3.0, 3.0), Pos::new(3.0, 5.0)))
    //     .add(Line::new(Pos::new(6.0, 3.0), Pos::new(6.0, 5.0)))
    //     .add(Line::new(Pos::new(3.0, 7.0), Pos::new(6.0, 7.0)));
    //
    // ren.render(smile);
}
