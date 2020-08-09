extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use nnnoiseless::DenoiseState;

fn main() {
    let sine: Vec<_> = (0..48_000)
        .map(|x| (x as f32 * 440.0 * 2.0 * std::f32::consts::PI / 48_000.0).sin() * i16::MAX as f32)
        .collect();
    let mut output = Vec::new();
    let mut out_buf = [0.0; DenoiseState::FRAME_SIZE];
    let mut denoise = DenoiseState::new();
    let mut first = true;
    for chunk in sine.chunks_exact(DenoiseState::FRAME_SIZE) {
        denoise.process_frame(&mut out_buf[..], chunk);

        // We throw away the first output, as discussed in the documentation for
        //`DenoiseState::process_frame`.
        if !first {
            output.extend_from_slice(&out_buf[..]);
        }
        first = false;
    }
}
