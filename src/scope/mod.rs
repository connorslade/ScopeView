use std::ops::Add;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat};

use crate::render::{Line, Pos, Render};

pub struct ScopeRender {
    size: (usize, usize),
}

impl ScopeRender {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            size: (x_size, y_size),
        }
    }

    fn get_next_frame(&self, lines: &[Line], line_i: &mut usize, x: &mut f32) -> Pos {
        // TODO: Calc sloap beforehand
        let mut line = lines.get(*line_i).unwrap();

        if *x > line.end.x {
            *x = 0.0;
            *line_i += 1;

            if *line_i > lines.len() - 1 {
                *line_i = 0;
            }
            line = lines.get(0).unwrap();
        }

        let sloap = (line.end.y - line.start.y) / (line.end.x - line.start.x);
        let y = sloap * x.to_owned() as f32;

        *x += line.step / 100.0;
        scale_pos(Pos::new(x.to_owned() as f32, y), self.size)
    }

    pub fn render(self, ren: impl Render) {
        let lines = ren.render();

        // Init audio output
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        if supported_config.channels() != 2 {
            panic!("Too many chanels! [{}]", supported_config.channels());
        }

        // RENDER
        let mut frame = Pos::new(0., 0.);
        let mut line = 0;
        let mut x = 0.0;
        let stream = device
            .build_output_stream(
                &supported_config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for (i, e) in data.iter_mut().enumerate() {
                        if i % 2 == 0 {
                            frame = Self::get_next_frame(&self, &lines, &mut line, &mut x);
                            *e = Sample::from(&frame.x);
                            continue;
                        }
                        *e = Sample::from(&frame.y);
                    }
                },
                move |err| {
                    dbg!(err);
                },
            )
            .unwrap();

        // Start stream outputing
        stream.play().unwrap();

        // wait,,,
        std::thread::sleep(std::time::Duration::from_millis(10000));
    }
}

#[inline]
fn scale_pos(inp: Pos, size: (usize, usize)) -> Pos {
    Pos::new(inp.x / size.0 as f32, inp.y / size.1 as f32)
}
