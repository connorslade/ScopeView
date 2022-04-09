use std::sync::mpsc::Receiver;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat};

use crate::render::{Line, Pos, Render};

type Cooldown = (u32, u32, Option<Pos>, Option<Pos>);

pub struct ScopeRender {
    size: (usize, usize),
    rx: Receiver<Vec<Line>>,
}

impl ScopeRender {
    pub fn new(x_size: usize, y_size: usize, rx: Receiver<Vec<Line>>) -> Self {
        Self {
            size: (x_size, y_size),
            rx,
        }
    }

    fn get_next_frame(
        &self,
        lines: &[Line],
        line_i: &mut usize,
        inc: &mut u32,
        cooldown: &mut Cooldown,
    ) -> Pos {
        let line = lines.get(*line_i).unwrap();

        let start = scale_pos(line.start, self.size);
        let end = scale_pos(line.end, self.size);

        // Get the length of the line
        let distance = line.distance();
        let ratio = (line.step * (*inc as f32)) / distance;

        // Calc beam pos
        let p = Pos::new(
            ratio * end.x + (1.0 - ratio) * start.x,
            ratio * end.y + (1.0 - ratio) * start.y,
        );

        if line.cool_down {
            // End cooldown
            if (*cooldown).0 > 0 {
                (*cooldown).0 -= 1;
                return cooldown.2.unwrap();
            }

            // Start cooldown
            if (*cooldown).1 > 0 {
                (*cooldown).1 -= 1;
                if let Some(i) = (*cooldown).3 {
                    return i;
                }
                (*cooldown).3 = Some(p);
            }
        }

        // check if we need to terminate
        if ratio > 1.0 {
            *line_i += 1;
            *inc = 0;
            *line_i %= lines.len();
            (*cooldown) = (20, 35, Some(p), None);

            return self.get_next_frame(lines, line_i, inc, cooldown);
        }
        *inc += 1;

        p
    }

    pub fn render(self) {
        // Init audio output
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range
            .nth(1)
            .expect("no supported config?!")
            .with_sample_rate(cpal::SampleRate(44100));
        let channels = supported_config.channels() as usize;
        dbg!(channels);

        // RENDER
        let mut lines = self.rx.recv().unwrap();
        let mut frame = Pos::new(0., 0.);
        let mut line = 0;
        let mut x = 0;
        let mut line_cooldown = (0, 0, None, None);
        let stream = device
            .build_output_stream(
                &supported_config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for (i, e) in data.iter_mut().enumerate() {
                        if let Ok(i) = self.rx.try_recv() {
                            lines = i;
                        }

                        match i % channels {
                            0 => {
                                frame = Self::get_next_frame(
                                    &self,
                                    &lines,
                                    &mut line,
                                    &mut x,
                                    &mut line_cooldown,
                                );
                                *e = Sample::from(&frame.x);
                            }
                            1 => *e = Sample::from(&frame.y),
                            _ => {}
                        }
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
        std::thread::park();
    }
}

#[inline]
fn scale_pos(inp: Pos, size: (usize, usize)) -> Pos {
    Pos::new(inp.x / size.0 as f32, inp.y / size.1 as f32)
}
