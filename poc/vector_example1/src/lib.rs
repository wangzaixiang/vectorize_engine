#![feature(portable_simd)]
use std::time::Duration;

pub mod sort2;

pub fn timeit<F, T>(_: &str, mut f: F) -> (T, Duration) where F: FnMut()-> T {
    let start = std::time::Instant::now();
    let t = f();
    let elapsed = start.elapsed();
    (t, elapsed)
}