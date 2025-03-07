extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use hound;
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
    }
}
