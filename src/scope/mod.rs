use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat};

use crate::render::Render;

pub struct ScopeRender {
    size: (usize, usize),
}

impl ScopeRender {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            size: (x_size, y_size),
        }
    }

    pub fn render(&self, ren: impl Render) {
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

        let stream = device
            .build_output_stream(
                &supported_config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        *sample = Sample::from(&1.0);
                    }
                },
                move |err| {
                    dbg!(err);
                },
            )
            .unwrap();
        stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
