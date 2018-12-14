#![feature(concat_idents)]
#![feature(type_ascription)]

extern crate homotopy;
extern crate underscore_args;
extern crate vecmath;
extern crate image;

pub mod utils;

use homotopy::*;
use underscore_args::args;
use utils::render::*;
use utils::math::*;

fn main() {
    let n = 10;
    for i in 0..n {
        let fx = i as f64 / n as f64;
        let a = Circle {center: [0.0, 0.0], radius: 0.9};
        let b = Circle {center: args!(rotate2d(pos: [0.0, 0.5], rounds: fx)), radius: 0.0};
        let c = sweep(a, b);
        let e = c.map(|a| {
            args!(rotate2d(pos: args!(displace_circle2d(
                pos: a,
                amplitude: 0.01,
                rounds: a[0] * 100.0
            )), rounds: vec2_len(a) * 0.25))
        });
        let file = file_name("exports/test", i);
        args!(clear(file: &file, size: 512)).unwrap();
        resolution(200, |s| {
            let f = e.left_right(s);
            args!(overlay2d(
                file: &file,
                function: |fx| f.hu(fx),
                aabb: ([-1.0; 2], [1.0; 2]),
                resolution: 20000,
            )).unwrap();
        });
    }
}
