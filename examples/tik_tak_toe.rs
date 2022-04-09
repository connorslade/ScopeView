use renderer::{
    render::{Line, Pos, Render},
    shapes::{circle::Circle, group::Group, point::Point, text::Text},
    ConsoleRender, ScopeRender,
};

#[derive(Debug, Clone, Copy)]
enum Tile {
    None,
    Ex,
    Oh,
}

fn main() {
    let ren = ScopeRender::new(10, 10);
    // let mut circle: Circle = Circle::new(5.0, Pos::new(0.0, 0.0));
    // circle.generate();
    // ren.render(circle);

    let mut board = vec![vec![Tile::Ex; 3]; 3];
    ren.render(Group::new().add(bounding()).add(peices(board)));
}

fn peices(peices: Vec<Vec<Tile>>) -> Group {
    let mut g = Group::new();

    //let index: [u8; 9] = [6, 7, 8, 5, 4, 3, 0, 1, 2];

    for (xi, x) in peices.iter().enumerate() {
        for (yi, y) in x.iter().enumerate() {
            g = g.add(tile_draw(*y, xi as i8, yi as i8));
        }
    }

    g
}

#[inline]
fn gridcoord_c(x: i8, y: i8) -> Pos {
    Pos::new(
        ((-5 + 10 * x) as f32 / 3.0) + 5.0 / 3.0,
        ((-5 + 10 * y) as f32 / 3.0) + 5.0 / 3.0,
    )
}

#[inline]
fn tile_draw(tile: Tile, x: i8, y: i8) -> Group {
    let c = gridcoord_c(x, y);

    match tile {
        Tile::None => Group::new(),
        Tile::Ex => Group::new()
            .add(Line::new(
                Pos::new(c.x - 0.5, c.y - 0.5),
                Pos::new(c.x + 0.5, c.y + 0.5),
            ))
            .add(Line::new(
                Pos::new(c.x - 0.5, c.y + 0.5),
                Pos::new(c.x + 0.5, c.y - 0.5),
            )),
        Tile::Oh => Group::new()
            .add(Line::new(
                Pos::new(c.x - 0.5, c.y - 0.5),
                Pos::new(c.x - 0.5, c.y + 0.5),
            ))
            .add(Line::new(
                Pos::new(c.x - 0.5, c.y + 0.5),
                Pos::new(c.x + 0.5, c.y + 0.5),
            ))
            .add(Line::new(
                Pos::new(c.x + 0.5, c.y + 0.5),
                Pos::new(c.x + 0.5, c.y - 0.5),
            ))
            .add(Line::new(
                Pos::new(c.x + 0.5, c.y - 0.5),
                Pos::new(c.x - 0.5, c.y - 0.5),
            )),
    }
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

// let positions: [Pos; 11] = [
//     gridcoord(0, 2),
//     gridcoord_c(0, 0),
//     gridcoord_c(1, 0),
//     gridcoord_c(2, 0),
//     gridcoord_c(2, 1),
//     gridcoord_c(1, 1),
//     gridcoord_c(0, 1),
//     gridcoord_c(0, 2),
//     gridcoord_c(1, 2),
//     gridcoord_c(2, 2),
//     gridcoord(3, 2),
// ];
//
// let mut group = Group::new();
//
// let index: [u8; 9] = [6, 7, 8, 5, 4, 3, 0, 1, 2];
//
// for (n, i) in index.iter().enumerate() {
//     // n for index of tic tac toe square
//     group = group // .add(Point(positions[*i as usize]));
//         .add(Point(positions[n]));
// }
// group.add(Line::new(positions[9], positions[10]).cool_down(false))
