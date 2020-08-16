extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use psimple::Simple;
use pulse::stream::Direction;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = get_simple();
    let mut buffer = [0; 4096];
    let mut out_file = File::create("out_raw.bin")?;

    for _ in 0..200 {
        s.read(&mut buffer).unwrap();
        out_file.write_all(&buffer)?;
    }
    Ok(())
}

fn get_simple() -> Simple {
    let spec = pulse::sample::Spec {
        format: pulse::sample::SAMPLE_FLOAT32NE,
        channels: 1,
        rate: 48000,
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
    );
    match s {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            panic!("failed getting simple")
        }
    }
}
