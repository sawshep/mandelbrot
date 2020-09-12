extern crate kiss3d;
extern crate nalgebra as alg;

use alg::{Complex, Translation2};
use kiss3d::light::Light;
use kiss3d::window::Window;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const MAX_ITERATIONS: u32 = 20;
const EXPLOSION_THRESHOLD: f64 = 4.0;

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

    let coordinates: [[[f32; 2]; WIDTH as usize]; HEIGHT as usize];
    /* Something like this:
     * [[[],[],[]],
     *  [[],[],[]],
     *  [[],[],[]]]
     */

    let mut 

    let mut y: f32 = 0.0;
    while y <= (HEIGHT as f32) {
        let mut x: f32 = 0.0;
        while x <= (WIDTH as f32) {
            let mut square = window.add_rectangle(1.0, 1.0);
            let color_multiplier = (mandelbrot(x, y, MAX_ITERATIONS)) as f32;
            square.set_color(color_multiplier, color_multiplier, color_multiplier);
            square.append_translation(&Translation2::new(x, y));
            x += 1.0;
        }
        y += 1.0;
    }
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);

    while window.render() {}
}
