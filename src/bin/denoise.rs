extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use hound::WavReader;
use nnnoiseless::DenoiseState;

fn main() {
    let mut reader = WavReader::open("raw.wav").unwrap();
    let samples = reader
        .samples::<f32>()
        .collect::<Result<Vec<f32>, hound::Error>>()
        .unwrap();
    let mut output = Vec::new();
    let mut out_buf = [0.0; DenoiseState::FRAME_SIZE];
    let mut denoise = DenoiseState::new();
    let mut first = true;
    for chunk in samples.chunks_exact(DenoiseState::FRAME_SIZE) {
        denoise.process_frame(&mut out_buf[..], chunk);

        // We throw away the first output, as discussed in the documentation for
        //`DenoiseState::process_frame`.
        if !first {
            output.extend_from_slice(&out_buf[..]);
        }
        first = false;
    }

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create("raw_denoised.wav", spec).unwrap();
    for sample in output {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
}
