use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

const duration: i32 = 2;
const sample_rate: i32 = 44100;
const frequency: i32 = 440;

fn main() {
    generate()
}

fn generate() {
    let mut start = 1.0;
    let end = 1.0e-4;
    let tau = PI * 2f64;
    let nsamps = duration * sample_rate;
    let decayfac = ((end / start) as f64).powf(1.0 / nsamps as f64);
    let angle = tau / nsamps as f64;
    let mut file = File::create("out.bin").unwrap();
    for i in 0..nsamps {
        let sample = ((angle * frequency as f64 * i as f64).sin() * start) as f32;
        start *= decayfac;
        let empty = [0u8; 4];
        let buffer = sample.to_le_bytes();
        file.write_all(&buffer).unwrap();
        file.write_all(&empty).unwrap();
    }
}
