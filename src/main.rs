extern crate image;
extern crate num_complex;

use num_complex::Complex;
use std::thread;

const RESOLUTION: u32 = 100;
const MAXIMUM_ITERATIONS: u32 = 20;
const EXPLOSION_THRESHOLD: u32 = 4;

fn mandelbrot(real: f32, imag: f32, max_iter: u32) -> u32 {
    let c = Complex { re: real, im: imag };
    let mut z = Complex { re: 0.0, im: 0.0 };

    let mut i: u32 = 0;
    while i <= max_iter as u32 {
        z = (z * z) + c;
        if z.norm() > (EXPLOSION_THRESHOLD as f32) {
            return 1;
        }
    }
    return 0;
}

fn main() {}
