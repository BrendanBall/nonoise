use dasp::frame::{Frame, Mono, Stereo};
use dasp::sample::Sample;
use dasp_graph::{Buffer, Input, Node};
use nnnoiseless::DenoiseState;

pub const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;
const VAD_GRACE_PERIOD_SAMPLES: i16 = 20;

pub struct Denoise {
    denoise_state: Box<DenoiseState>,
    input_buffer: Vec<f32>,
    output_buffer: Vec<f32>,
    remaining_grace_period: i16,
    vad_threshold: f32,
}

impl Denoise {
    pub fn new() -> Box<Self> {
        Box::new(Denoise {
            denoise_state: DenoiseState::new(),
            input_buffer: vec![0f32; FRAME_SIZE],
            output_buffer: vec![0f32; FRAME_SIZE],
            remaining_grace_period: 0,
            vad_threshold: 0.95,
        })
    }
}

impl Node for Denoise {
    // Port https://github.com/werman/noise-suppression-for-voice/blob/34003bd9ab1509708eab61ef458feaae23327495/src/common/src/RnNoiseCommonPlugin.cpp
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        let input = match inputs.get(0) {
            Some(input) => input,
            None => return,
        };

        // Apply the delay across each channel.
        for ((ring_buf, in_buf), out_buf) in self.0.iter_mut().zip(input.buffers()).zip(output) {
            for (i, out) in out_buf.iter_mut().enumerate() {
                *out = ring_buf.push(in_buf[i]);
            }
        }

        for (frame_iteration, frame) in input.chunks(FRAME_SIZE).enumerate() {
            for i in 0..frame.len() {
                self.input_buffer[i] =
                    frame[i].to_float_sample().to_sample::<f32>() * i16::MAX as f32;
            }

            for i in frame.len()..self.input_buffer.len() {
                self.input_buffer[i] = 0f32;
            }

            let vad_probability = self
                .denoise_state
                .process_frame(&mut self.output_buffer[..], &self.input_buffer[..]);

            if vad_probability >= self.vad_threshold {
                self.remaining_grace_period = VAD_GRACE_PERIOD_SAMPLES;
            }

            if self.remaining_grace_period > 0 {
                self.remaining_grace_period -= 1;
                for i in 0..frame.len() {
                    output[frame_iteration * FRAME_SIZE + i] =
                        (self.output_buffer[i] / i16::MAX as f32).to_sample::<f32>();
                }
            } else {
                for i in 0..frame.len() {
                    output[frame_iteration * FRAME_SIZE + i] = (0f32).to_sample::<f32>();
                }
            }
        }
    }
}
