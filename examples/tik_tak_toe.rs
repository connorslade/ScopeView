use std::fmt;
use std::sync::mpsc;

use getch::Getch;
use renderer::{
    render::{Line, Pos, Render},
    shapes::{circle::Circle, group::Group, point::Point, text::Text},
    ConsoleRender, ScopeRender,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    None,
    Ex,
    Oh,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Tile::None => "None",
                Tile::Ex => "X",
                Tile::Oh => "O",
            }
        );

        Ok(())
    }
}

fn main() {
    print!("\x1B[2J");
    let mspf = (10.0_f32.recip() * 1000.0) as u64;
    let mut board = vec![vec![Tile::None; 3]; 3];
    let mut player = Tile::Ex;
    let mut lose_line = None;
    let mut flag: bool = true;

    let (tx, rx) = mpsc::sync_channel(10);
    let (tx_chr, rx_chr) = mpsc::sync_channel(50);

    // Keyboard Interface Thread :|||
    std::thread::spawn(move || loop {
        let chr = Getch::new().getch().unwrap();
        tx_chr.send(chr).unwrap();
    });

    // Pregen circle for um well
    // im not really sure why brendon did this but its here so :\
    let mut circle = Circle::new(1.0);
    circle.generate();

    let ren = ScopeRender::new(10, 10, rx);
    std::thread::spawn(|| ren.render());

    loop {
        let start = std::time::Instant::now();
        if flag && lose_line.is_none() {
            println!("GO {}", player);
            flag = false;
        }

        if let Ok(i) = rx_chr.try_recv() {
            if let Some(i) = char::from_u32(i as u32) {
                let l = lose_line.is_none();
                let e = match i {
                    'q' => safe_set_player(&mut board, &mut player, l, (2, 0)),
                    'w' => safe_set_player(&mut board, &mut player, l, (2, 1)),
                    'e' => safe_set_player(&mut board, &mut player, l, (2, 2)),
                    'a' => safe_set_player(&mut board, &mut player, l, (1, 0)),
                    's' => safe_set_player(&mut board, &mut player, l, (1, 1)),
                    'd' => safe_set_player(&mut board, &mut player, l, (1, 2)),
                    'z' => safe_set_player(&mut board, &mut player, l, (0, 0)),
                    'x' => safe_set_player(&mut board, &mut player, l, (0, 1)),
                    'c' => safe_set_player(&mut board, &mut player, l, (0, 2)),
                    ' ' => {
                        if lose_line.is_some() {
                            lose_line = None;
                            board = vec![vec![Tile::None; 3]; 3];
                            player = Tile::Ex;
                            lose_line = None;
                            flag = true;
                            print!("\x1B[2J");
                        }
                        false
                    }
                    _ => {
                        println!("Invalid Char: `{}`", i);
                        false
                    }
                };

                if e {
                    if board.iter().all(|x| x.iter().all(|y| *y != Tile::None)) {
                        println!("Tie -_-");
                        lose_line = Some(Group::new());
                    }

                    flag = true;
                    let ret = detect_win(&board);
                    if ret.1 != Tile::None {
                        println!("{} Wins", ret.1);
                        lose_line = Some(do_win(ret.0, ret.1));
                    }
                }
            }
        }

        let mut render = Group::new().add(bounding()).add(peices(&board, &circle));
        if let Some(i) = &lose_line {
            render = render.add(i.render());
        }

        tx.send(render.render()).unwrap();
        let end_ms = start.elapsed().as_millis();

        std::thread::sleep(std::time::Duration::from_millis(mspf - end_ms as u64));
    }
}

fn safe_set_player(
    board: &mut Vec<Vec<Tile>>,
    player: &mut Tile,
    run: bool,
    pos: (u8, u8),
) -> bool {
    if !run {
        return false;
    }

    let prev = (*board)[pos.0 as usize][pos.1 as usize];

    if prev != Tile::None {
        println!("Spot Taken!");
        return false;
    }

    (*board)[pos.0 as usize][pos.1 as usize] = *player;
    *player = match *player {
        Tile::Ex => Tile::Oh,
        Tile::Oh => Tile::Ex,
        Tile::None => unreachable!(),
    };

    true
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
fn do_win(win: WinType, ti: Tile) -> Group {
    match win {
        WinType::None => Group::new(),
        WinType::Horizontal(n) => {
            Group::new().add(Line::new(gridcoord_c(n as i8, -1), gridcoord_c(n as i8, 3)))
        }
        WinType::Vertical(n) => {
            Group::new().add(Line::new(gridcoord_c(-1, n as i8), gridcoord_c(3, n as i8)))
        }
        WinType::Diagonal(0) => Group::new().add(Line::new(gridcoord_c(-1, -1), gridcoord_c(3, 3))),
        WinType::Diagonal(1) => Group::new().add(Line::new(gridcoord_c(-1, 3), gridcoord_c(3, -1))),
        _ => unimplemented!(),
    }
}

fn detect_win(board: &[Vec<Tile>]) -> (WinType, Tile) {
    // check horizontals
    for n in 0..3 {
        if board[n][0] == board[n][1] && board[n][1] == board[n][2] && board[n][0] != Tile::None {
            return (WinType::Horizontal(n as u8), board[n][0]);
        }
    }

    // check verticals
    for n in 0..3 {
        if board[0][n] == board[1][n] && board[1][n] == board[2][n] && board[0][n] != Tile::None {
            return (WinType::Vertical(n as u8), board[0][n]);
        }
    }

    // check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[1][1] != Tile::None {
        return (WinType::Diagonal(0), board[1][1]);
    }

    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[1][1] != Tile::None {
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
