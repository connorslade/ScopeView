use std::io::{stdout, Write};

use crate::render::Render;

pub struct ConsoleRender {
    size: (usize, usize),
}

impl ConsoleRender {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            size: (x_size, y_size),
        }
    }

    pub fn render(&self, ren: impl Render) {
        let mut buf = vec![vec![false; self.size.0]; self.size.1];

        for line in ren.render() {
            render_line(
                &mut buf,
                line.start.x as i32,
                line.start.y as i32,
                line.end.x as i32,
                line.end.y as i32,
            );
        }

        let mut out = String::new();
        for y in buf {
            for x in y {
                if x {
                    out.push('#');
                    out.push('#');
                    continue;
                }
                out.push('.');
                out.push('.');
            }
            out.push('\n');
        }

        stdout().write(out.as_bytes()).unwrap();
        stdout().flush().unwrap();
    }
}

pub fn render_line(buf: &mut Vec<Vec<bool>>, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    if (end_y - start_y).abs() < (end_x - start_x).abs() {
        if start_x > end_x {
            line_low(buf, end_x, end_y, start_x, start_y);
            return;
        }
        line_low(buf, start_x, start_y, end_x, end_y);
        return;
    }

    if start_y > end_y {
        line_high(buf, end_x, end_y, start_x, start_y);
        return;
    }
    line_high(buf, start_x, start_y, end_x, end_y);
}

fn line_low(buf: &mut Vec<Vec<bool>>, x0: i32, y0: i32, x1: i32, y1: i32) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = 2 * dy - dx;
    let mut y = y0;

    for x in x0..x1 + 1 {
        buf[y as usize][x as usize] = true;
        if d > 0 {
            y += yi;
            d -= 2 * dx;
        }
        d += 2 * dy;
    }
}

fn line_high(buf: &mut Vec<Vec<bool>>, x0: i32, y0: i32, x1: i32, y1: i32) {
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = 2 * dx - dy;
    let mut x = x0;

    for y in y0..y1 + 1 {
        buf[y as usize][x as usize] = true;
        if d > 0 {
            x += xi;
            d -= 2 * dy;
        }
        d += 2 * dx;
    }
}
