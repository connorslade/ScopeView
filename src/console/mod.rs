use std::io::{stdout, Write};

use crate::render::Render;

mod common;

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
        // Init buffer
        let mut buf = vec![vec![false; self.size.0]; self.size.1];
        dbg!(buf.len(), buf[0].len());

        for line in ren.render() {
            // Render lines to buffer
            common::render_line(&mut buf, line);
        }

        // Genarate a string from the pixel buffer
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

        // Write to stdout
        stdout().write_all(out.as_bytes()).unwrap();
        stdout().flush().unwrap();
    }
}
