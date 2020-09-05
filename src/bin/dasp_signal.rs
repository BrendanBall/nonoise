use dasp::sample::Sample;
use dasp::signal::{self as signal, Signal};

fn main() {
    let frames = [1.to_sample::<i8>(), -3, 5, 6];
    let mut signal = signal::from_iter(frames.iter().cloned());
    assert_eq!(signal.next(), 1);
    assert_eq!(signal.next(), -3);
    assert_eq!(signal.next(), 5);
    assert_eq!(signal.next(), 6);
    assert_eq!(signal.next(), 0);
}
