use crate::render::Line;

pub fn render_line(buf: &mut Vec<Vec<bool>>, line: Line) {
    if (line.end.y - line.start.y).abs() < (line.end.x - line.start.x).abs() {
        if line.start.x > line.end.x {
            line_low(
                buf,
                line.end.x as i32,
                line.end.y as i32,
                line.start.x as i32,
                line.start.y as i32,
            );
            return;
        }
        line_low(
            buf,
            line.start.x as i32,
            line.start.y as i32,
            line.end.x as i32,
            line.end.y as i32,
        );
        return;
    }

    if line.start.y > line.end.y {
        line_high(
            buf,
            line.end.x as i32,
            line.end.y as i32,
            line.start.x as i32,
            line.start.y as i32,
        );
        return;
    }
    line_high(
        buf,
        line.start.x as i32,
        line.start.y as i32,
        line.end.x as i32,
        line.end.y as i32,
    );
}

// line_low and line_high are 'boreowed' from console_engine
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
