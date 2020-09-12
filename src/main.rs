extern crate kiss3d;
extern crate nalgebra as alg;

use alg::{Complex, Translation2};
use kiss3d::light::Light;
use kiss3d::window::Window;

const RESOLUTION: u32 = 100;
const MAX_ITERATIONS: u32 = 20;
const EXPLOSION_THRESHOLD: u32 = 4;

fn mandelbrot(real: f32, imag: f32, max_iter: u32) -> u32 {
    let c = Complex { re: real, im: imag };

    let mut z = Complex { re: 0.0, im: 0.0 };

    let mut i: u32 = 0;
    while i <= max_iter {
        z = z * z + c;
        if (z.re * z.re + z.im * z.im) >= (EXPLOSION_THRESHOLD as f32) {
            return 0;
        }
        i += 1;
    }
    return 1;
}

fn main() {
    let mut window = Window::new("Mandelbrot");

    let mut pixel_y: u32 = 0;
    while pixel_y < RESOLUTION {
        let mut pixel_x: u32 = 0;
        while pixel_x < RESOLUTION {
            let real_x: f32 = 0.5 * (pixel_x as f32) + 2.0;
            let imag_y: f32 = -0.5 * (pixel_y as f32) - 2.0;
            let color_multiplier =
                (mandelbrot(real_x, imag_y, MAX_ITERATIONS) / MAX_ITERATIONS) as f32;

            let pixel_width = (RESOLUTION / EXPLOSION_THRESHOLD) as f32;
            let mut pixel = window.add_rectangle(pixel_width, pixel_width);
            pixel.set_color(color_multiplier, color_multiplier, color_multiplier);
            pixel.append_translation(&Translation2::new(real_x, imag_y));

            pixel_x += 1;
        }
        pixel_y += 1;
    }

    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);

    while window.render() {}
}
