extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use nnnoiseless_plugin::{Denoise, FRAME_SIZE};
use psimple::Simple;
use pulse::stream::Direction;
use std::convert::TryInto;
use std::mem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    type SampleType = f32;
    const SAMPLE_TYPE_SIZE: usize = mem::size_of::<SampleType>();
    let input = get_input_simple();
    let output = get_output_simple();
    let mut input_buffer_bytes = [0; FRAME_SIZE * SAMPLE_TYPE_SIZE];
    let mut input_buffer = [0f32; FRAME_SIZE];
    let mut output_buffer_bytes = [0; FRAME_SIZE * SAMPLE_TYPE_SIZE];
    let mut output_buffer = [0f32; FRAME_SIZE];

    let mut denoise = Denoise::new();

    loop {
        input.read(&mut input_buffer_bytes).unwrap();

        for (i, sample) in input_buffer_bytes
            .chunks_exact(SAMPLE_TYPE_SIZE)
            .enumerate()
        {
            input_buffer[i] = f32::from_le_bytes(sample.try_into().unwrap());
        }

        denoise.process(&input_buffer[..], &mut output_buffer[..], FRAME_SIZE, 0.95);

        for (i, sample) in output_buffer.iter().enumerate() {
            for (j, byte) in f32::to_le_bytes(*sample).iter().enumerate() {
                output_buffer_bytes[i * SAMPLE_TYPE_SIZE + j] = *byte;
            }
        }

        output.write(&output_buffer_bytes).unwrap();
    }
}

fn get_input_simple() -> Simple {
    let spec = pulse::sample::Spec {
        format: pulse::sample::SAMPLE_FLOAT32NE,
        channels: 1,
        rate: 48000,
    };
    assert!(spec.is_valid());
    let s = Simple::new(
        None,                // Use the default server
        "FooAppInput",       // Our application’s name
        Direction::Record,   // We want a record stream
        None,                // Use the default device
        "Test record input", // Description of our stream
        &spec,               // Our sample format
        None,                // Use default channel map
        None,                // Use default buffering attributes
    );
    match s {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            panic!("failed getting simple")
        }
    }
}

fn get_output_simple() -> Simple {
    let spec = pulse::sample::Spec {
        format: pulse::sample::SAMPLE_FLOAT32NE,
        channels: 1,
        rate: 48000,
    };
    assert!(spec.is_valid());
    let s = Simple::new(
        None,                             // Use the default server
        "FooAppOutput",                   // Our application’s name
        Direction::Playback,              // We want a record stream
        Some("nonoise_mic_denoised_out"), // Use the default device
        "Test record output",             // Description of our stream
        &spec,                            // Our sample format
        None,                             // Use default channel map
        None,                             // Use default buffering attributes
    );
    match s {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            panic!("failed getting simple")
        }
    }
}
