use nnnoiseless::DenoiseState;

const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;
const VAD_GRACE_PERIOD_SAMPLES: i16 = 20;

pub struct Denoise {
    denoise_state: Box<DenoiseState>,
    input_buffer: Vec<f32>,
    output_buffer: Vec<f32>,
    remaining_grace_period: i16,
}

impl Denoise {
    pub fn new() -> Box<Self> {
        Box::new(Denoise {
            denoise_state: DenoiseState::new(),
            input_buffer: vec![0f32; FRAME_SIZE],
            output_buffer: vec![0f32; FRAME_SIZE],
            remaining_grace_period: 0,
        })
    }

    // Port https://github.com/werman/noise-suppression-for-voice/blob/34003bd9ab1509708eab61ef458feaae23327495/src/common/src/RnNoiseCommonPlugin.cpp
    pub fn process(
        &mut self,
        input: &[f32],
        output: &mut [f32],
        sample_frames: usize,
        vad_threshold: f32,
    ) {
        assert!(vad_threshold >= 0f32 && vad_threshold <= 1f32);

        if sample_frames == 0 {
            return;
        }
        if sample_frames <= FRAME_SIZE {
            for i in 0..sample_frames {
                self.input_buffer[i] = input[i] * i16::MAX as f32;
            }
            for i in sample_frames..self.input_buffer.len() {
                self.input_buffer[i] = 0f32;
            }

            let vad_probability = self
                .denoise_state
                .process_frame(output, &self.input_buffer[..]);

            if vad_probability >= vad_threshold {
                self.remaining_grace_period = VAD_GRACE_PERIOD_SAMPLES;
            }

            if self.remaining_grace_period > 0 {
                self.remaining_grace_period -= 1;
                for i in 0..sample_frames {
                    output[i] /= i16::MAX as f32;
                }
            } else {
                for i in 0..sample_frames {
                    output[i] = 0f32;
                }
            }
        } else if sample_frames > FRAME_SIZE {
            for (frame_iteration, frame) in input.chunks(FRAME_SIZE).enumerate() {
                for i in 0..frame.len() {
                    self.input_buffer[i] = frame[i] * i16::MAX as f32;
                }

                for i in frame.len()..self.input_buffer.len() {
                    self.input_buffer[i] = 0f32;
                }

                let vad_probability = self
                    .denoise_state
                    .process_frame(&mut self.output_buffer[..], &self.input_buffer[..]);

                if vad_probability >= vad_threshold {
                    self.remaining_grace_period = VAD_GRACE_PERIOD_SAMPLES;
                }

                if self.remaining_grace_period > 0 {
                    self.remaining_grace_period -= 1;
                    for i in 0..frame.len() {
                        output[frame_iteration * FRAME_SIZE + i] =
                            self.output_buffer[i] / i16::MAX as f32;
                    }
                } else {
                    for i in 0..frame.len() {
                        output[frame_iteration * FRAME_SIZE + i] = 0f32;
                    }
                }
            }
        }
    }
}
