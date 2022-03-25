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
            let mut slope = (line.end.y - line.start.y) / (line.end.x - line.start.x);
            let mut step = line.step;
            if line.end.x < line.start.x {
                slope = -slope;
                step = -step;
            }

            let mut y = line.start.y;
            let mut i = line.start.x;
            buf[y as usize][i as usize] = true;

            while i != line.end.x {
                y += slope;
                buf[y as usize][i as usize] = true;
                i += step;
            }
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
