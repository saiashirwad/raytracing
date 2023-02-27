#![allow(dead_code)]
mod color;
mod point;
mod ppm;
mod vec3;

use crate::{ppm::PPM, vec3::Vec3};
use std::fmt::Debug;

static WIDTH: u32 = 300;
static HEIGHT: u32 = 300;

fn main() {
    let mut img = PPM::new(WIDTH, HEIGHT);

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let color = Vec3::new(
                i as f32 / (WIDTH - 1) as f32,
                j as f32 / (HEIGHT - 1) as f32,
                0.25,
            )
            .to_color();
            img[(j * WIDTH + i) as usize] = color;
        }
    }

    img.write_to_file("test.ppm");
}
