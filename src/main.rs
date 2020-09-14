extern crate image;
extern crate num_complex;

use image::ImageBuffer;
use num_complex::Complex;
//use std::thread;

const RESOLUTION: u32 = 1000;

const MINIMUM_REAL: f32 = -2.0;
const MAXIMUM_REAL: f32 = 2.0;
const MINIMUM_IMAGINARY: f32 = -2.0;
const MAXIMUM_IMAGINARY: f32 = 2.0;

const X_SCALE: f32 = (MAXIMUM_REAL - MINIMUM_REAL) / ((RESOLUTION - 1) as f32);
const Y_SCALE: f32 = (MAXIMUM_IMAGINARY - MINIMUM_IMAGINARY) / ((RESOLUTION - 1) as f32);
const MAXIMUM_ITERATIONS: u32 = 50;
const EXPLOSION_THRESHOLD: u32 = 2;

fn mandelbrot(real: f32, imag: f32, max_iter: u32) -> f32 {
    let c = Complex { re: real, im: imag };
    let mut z = Complex { re: 0.0, im: 0.0 };

    let mut i: u32 = 0;
    while i <= max_iter as u32 {
        z = (z * z) + c;
        if z.norm() > (EXPLOSION_THRESHOLD as f32) {
            return (i as f32) / (max_iter as f32);
        }
        i += 1;
    }
    return 1.0;
}

fn main() {
    let mut image_buffer = ImageBuffer::new(RESOLUTION, RESOLUTION);
    println!("Image buffer created");
    println!("Running...");
    for (pixel_x, pixel_y, pixel) in image_buffer.enumerate_pixels_mut() {
        //let child = thread::spawn(|| {
        let x_real = MINIMUM_REAL + (pixel_x as f32) * X_SCALE;
        let y_imag = -1.0 * (MINIMUM_IMAGINARY + (pixel_y as f32) * Y_SCALE);

        let color_multiplier = mandelbrot(x_real, y_imag, MAXIMUM_ITERATIONS);
        let rgb_value = (255.0 * color_multiplier) as u8;
        *pixel = image::Rgb([rgb_value, rgb_value, rgb_value]);
        //});
    }
    println!("Saving image buffer to fractal.png...");
    image_buffer.save("fractal.png").unwrap();
    println!("Done");
}
