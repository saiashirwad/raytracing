use std::ops::{Index, IndexMut};

use crate::{vec3::Color, Debug};

pub struct PPM {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl PPM {
    pub fn new(width: u32, height: u32) -> PPM {
        PPM {
            width,
            height,
            pixels: vec![Color::new(255., 0., 0.); (width * height) as usize],
        }
    }

    fn num_pixels(&self) -> u32 {
        self.pixels.len() as u32
    }

    pub fn set_pixels(&mut self, pixels: Vec<Color>) {
        if (self.width * self.height) as usize != pixels.len() {
            panic!("Invalid number of pixels");
        }
        self.pixels = pixels;
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            panic!("Invalid pixel coordinates");
        }
        self.pixels[(y * self.width + x) as usize] = color;
    }

    pub fn write_to_file(&self, filename: &str) {
        std::fs::write(filename, self.to_string()).unwrap();
    }
}

impl Index<usize> for PPM {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index as usize]
    }
}

impl IndexMut<usize> for PPM {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index as usize]
    }
}

impl ToString for PPM {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(&format!("{} {}\n", self.width, self.height));
        s.push_str("255\n");
        self.pixels.iter().enumerate().for_each(|(idx, p)| {
            s.push_str(&(p.to_string()));
            s.push_str(" ");
            if idx % self.width as usize == self.width as usize - 1 {
                s.push_str("\n");
            }
        });
        s
    }
}

impl Debug for PPM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
