extern crate kiss3d;
extern crate nalgebra as alg;

use alg::{Complex, Point2, Point3, Translation2};
use kiss3d::light::Light;
use kiss3d::window::Window;

// The resolution in "pixels" to render at.
const RESOLUTION: u32 = 3;
// The maximum number of times to check if a square is going to zero.
const MAX_ITERATIONS: u32 = 20;
// If a square goes over this value, it is considered to be going to infinity.
const EXPLOSION_THRESHOLD: u32 = 4;

const fn mandelbrot(real: f32, imag: f32, max_iter: u32) -> u32 {
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
            let ratio: f32 = (EXPLOSION_THRESHOLD / RESOLUTION) as f32;
            let real_x: f32 = ratio * (pixel_x as f32) - 2.0;
            let imag_y: f32 = -1.0 * ratio * (pixel_y as f32) + 2.0;
            println!("{:?} {:?}", real_x, imag_y);
            let color_multiplier =
                (mandelbrot(real_x, imag_y, MAX_ITERATIONS) / MAX_ITERATIONS) as f32;
            println!("{:?}", color_multiplier);

            let pixel_width = (1 / RESOLUTION) as f32;
            //let mut pixel = window.add_rectangle(pixel_width, pixel_width);
            //pixel.set_color(color_multiplier, color_multiplier, color_multiplier);
            //pixel.append_translation(&Translation2::new(real_x, imag_y));
            // pixel.append_translation(&Translation2::new());

            pixel_x += 1;
        }
        pixel_y += 1;
    }

    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);

    while window.render() {
        let top_left = Point2::new(-1000.0, 1000.0);
        let top_right = Point2::new(1000.0, 1000.0);
        let bot_right = Point2::new(1000.0, -1000.0);
        let bot_left = Point2::new(-1000.0, -1000.0);
        window.draw_planar_line(&top_left, &top_right, &Point3::new(0.0, 0.0, 0.0));
    }
}
