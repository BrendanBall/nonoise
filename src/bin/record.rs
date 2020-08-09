extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use hound;
use nnnoiseless::DenoiseState;
use psimple::Simple;
use pulse::stream::Direction;
use std::convert::TryInto;

fn main() {
    let spec = pulse::sample::Spec {
        format: pulse::sample::SAMPLE_FLOAT32NE,
        channels: 2,
        rate: 44100,
    };
    assert!(spec.is_valid());

    let s = Simple::new(
        None,              // Use the default server
        "FooApp",          // Our application’s name
        Direction::Record, // We want a record stream
        None,              // Use the default device
        "Test record",     // Description of our stream
        &spec,             // Our sample format
        None,              // Use default channel map
        None,              // Use default buffering attributes
    )
    .unwrap();

    let mut buffer = [0; 4096];

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create("raw.wav", spec).unwrap();

    // let mut output = Vec::new();
    // let mut out_buf = [0.0; DenoiseState::FRAME_SIZE];
    // let mut denoise = DenoiseState::new();

    for _ in 0..200 {
        s.read(&mut buffer).unwrap();
        let buffer_f32: Vec<f32> = buffer
            .chunks_exact(4)
            .into_iter()
            .map(|x| f32::from_le_bytes(x.try_into().unwrap()))
            .collect();

        for sample in buffer_f32 {
            writer.write_sample(sample).unwrap();
        }
        // for chunk in buffer_f32.chunks_exact(DenoiseState::FRAME_SIZE) {
        //     denoise.process_frame(&mut out_buf[..], chunk);
        //     if !first {
        //         for sample in &out_buf {
        //             writer.write_sample(*sample).unwrap();
        //         }
        //     }
        //     first = false;
        //     // output.extend_from_slice(&out_buf[..]);
        // }
        // writer.finalize().unwrap();
    }
}
