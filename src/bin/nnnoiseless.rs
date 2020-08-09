use std::fs::File;
use std::io::Write;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

use nnnoiseless::DenoiseState;

const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: rnnoise_demo INPUT_FILE OUTPUT_FILE");
        std::process::exit(1);
    }

    let mut in_file = File::open(&args[1])?;
    let mut out_file = File::create(&args[2])?;
    let mut buf = [0f32; FRAME_SIZE];
    let mut state = DenoiseState::new();
    let mut in_buf = [0.0; FRAME_SIZE];
    let mut out_buf = [0.0; FRAME_SIZE];
    let mut out_bytes = [0u8; FRAME_SIZE * 4];
    let mut first = true;
    while let Ok(_) = in_file.read_f32_into::<LittleEndian>(&mut buf[..]) {
        for (i, x) in buf.iter().enumerate() {
            in_buf[i] = *x * 32767.0;
        }
        state.process_frame(&mut out_buf[..], &in_buf[..]);
        for (i, x) in out_buf.iter().enumerate() {
            buf[i] = *x / 32767.0;
        }
        if !first {
            // Is there a (convenient) way to do the endian conversion without this extra copy?
            LittleEndian::write_f32_into(&buf[..], &mut out_bytes[..]);
            out_file.write_all(&out_bytes[..])?;
        }
        first = false;
    }
    Ok(())
}
