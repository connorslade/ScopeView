use renderer::{
    render::{Line, Pos, Render},
    shapes::{circle::Circle, group::Group, point::Point, text::Text},
    ConsoleRender, ScopeRender,
};
use std::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    None,
    Ex,
    Oh,
}

fn main() {
    let mspf = (10.0_f32.recip() * 1000.0) as u64;
    let mut board = vec![vec![Tile::None; 3]; 3];
    // let mut board = vec![vec![Tile::Ex, Tile:Ex, Tile:Oh]; 3];
    let (tx, rx) = mpsc::sync_channel(10);

    // Pregen circle for um well
    // im not really sure why brendon did this but its here so :\
    let mut circle = Circle::new(1.0);
    circle.generate();

    let ren = ScopeRender::new(10, 10, rx);
    std::thread::spawn(|| ren.render());

    loop {
        let start = std::time::Instant::now();
        let render = Group::new().add(bounding()).add(peices(&board, &circle));
        tx.send(render.render()).unwrap();
        let end_ms = start.elapsed().as_millis();

        std::thread::sleep(std::time::Duration::from_millis(mspf - end_ms as u64));
    }
}

fn peices(peices: &[Vec<Tile>], circle: &Circle) -> Group {
    let mut g = Group::new();

    for (xi, x) in peices.iter().enumerate() {
        for (yi, y) in x.iter().enumerate() {
            g = g.add(tile_draw(*y, xi as i8, yi as i8, &circle));
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

enum WinType {
    None,
    Horizontal(u8),
    Vertical(u8),
    Diagonal(u8),
}

// stuff with wins? idpokjihgv
fn do_win(win: WinType) {
    match win {
        WinType::None => {}
        WinType::Horizontal(_) => {}
        WinType::Vertical(_) => {}
        WinType::Diagonal(_) => {}
    }
}

fn detect_win(board: &[Vec<Tile>]) -> (WinType, Tile) {
    // check horizontals
    for n in 0..3 {
        if board[n][0] == board[n][1] && board[n][1] == board[n][2] {
            return (WinType::Horizontal(n as u8), board[n][0]);
        }
    }

    // check verticals
    for n in 0..3 {
        if board[0][n] == board[1][n] && board[1][n] == board[2][n] {
            return (WinType::Vertical(n as u8), board[0][n]);
        }
    }

    // check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return (WinType::Diagonal(0), board[1][1]);
    }

    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return (WinType::Diagonal(1), board[1][1]);
    }

    (WinType::None, Tile::None)
}

#[inline]
fn tile_draw(tile: Tile, x: i8, y: i8, circle: &Circle) -> Group {
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
            .add(circle.to_owned())
            .offset(Pos::new(c.x, c.y)),
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
