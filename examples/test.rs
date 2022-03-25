use renderer::{
    render::{Line, Pos},
    shapes::{point::Point, triangle::Triangle},
    ConsoleRender,
};

fn main() {
    let ren = ConsoleRender::new(30, 30);

    // let l1 = Line {
    //     start: Pos { x: 10.0, y: 10.0 },
    //     end: Pos { x: 5.0, y: 5.0 },
    //     step: 1.0,
    // };

    // let l2 = Line {
    //     start: Pos { x: 3.0, y: 6.0 },
    //     end: Pos { x: 15.0, y: 2.0 },
    //     step: 1.0,
    // };

    let tri = Triangle {
        point_a: Pos { x: 0.0, y: 0.0 },
        point_b: Pos { x: 7.0, y: 29.0 },
        point_c: Pos { x: 29.0, y: 29.0 },
        step: 1.0,
    };

    // let p1 = Point(Pos { x: 0.0, y: 0.0 });

    ren.render(tri);
}
