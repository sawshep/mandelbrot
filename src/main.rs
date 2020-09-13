extern crate image;
extern crate num_complex;

use image::ImageBuffer;
use num_complex::Complex;
use std::thread;

const RESOLUTION: u32 = 10;
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
    for y_pixel in 0..RESOLUTION {
        for x_pixel in 0..RESOLUTION {
            //let child = thread::spawn(|| {
            println!("({:?}, {:?})", x_pixel, y_pixel);
            let x_real = 4.0 / (x_pixel as f32) - 2.0;
            let y_imag = -4.0 / (y_pixel as f32) + 2.0;
            println!("({:?}, {:?})", x_real, y_imag);

            let color_multiplier = mandelbrot(x_real, y_imag, MAXIMUM_ITERATIONS);
            println!("{:?}", color_multiplier);
            let rgb_value = (255.0 * color_multiplier) as u8;
            let pixel = image_buffer.get_pixel_mut(x_pixel, y_pixel);
            *pixel = image::Rgb([rgb_value, rgb_value, rgb_value]);
            //});
            //let res = child.join();
        }
    }
    image_buffer.save("fractal.png").unwrap();
}
