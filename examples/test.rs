use renderer::{
    render::{Line, Pos},
    shapes::{group::Group, point::Point, triangle::Triangle},
    ConsoleRender, ScopeRender,
};

fn main() {
    let ren = ConsoleRender::new(10, 10);
    let ren2 = ScopeRender::new(10, 10);

    // let l1 = Line {
    //     start: Pos { x: 10.0, y: 10.0 },
    //     end: Pos { x: 5.0, y: 5.0 },
    //     step: 1.0,
    // };
    //
    // let l2 = Line {
    //     start: Pos { x: 3.0, y: 6.0 },
    //     end: Pos { x: 15.0, y: 2.0 },
    //     step: 1.0,
    // };

    // let tri = Triangle {
    //     point_a: Pos { x: 0.0, y: 0.0 },
    //     point_b: Pos { x: 7.0, y: 29.0 },
    //     point_c: Pos { x: 29.0, y: 29.0 },
    //     step: 1.0,
    // };
    // ren.render(
    //     Group::new()
    //         .add(renderer::shapes::text::Text::new(
    //             "Hello",
    //             Pos { x: 5.0, y: 5.0 },
    //         ))
    //         .add(renderer::shapes::text::Text::new(
    //             "World",
    //             Pos { x: 5.0, y: 20.0 },
    //         )),
    // );

    let p1 = Point(Pos { x: 5.0, y: 5.0 });
    ren.render(p1);
    ren2.render(p1);

    // ren.render(Group::new().add(l1).add(l2).add(tri).add(p1));

    // let smile = Group::new()
    //     .add(Line::new(Pos::new(3.0, 3.0), Pos::new(3.0, 5.0)))
    //     .add(Line::new(Pos::new(6.0, 3.0), Pos::new(6.0, 5.0)))
    //     .add(Line::new(Pos::new(3.0, 7.0), Pos::new(6.0, 7.0)));
    //
    // ren.render(smile);
}
