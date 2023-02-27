use std::ops::{Add, Div, Index, Mul, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn to_color(self) -> Color {
        let ir = (255.999 * self.x()) as u8;
        let ig = (255.999 * self.y()) as u8;
        let ib = (255.999 * self.z()) as u8;
        Color::new(ir as f32, ig as f32, ib as f32)
    }

    fn length_squared(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl ToString for Vec3 {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{} {} {}", self.x(), self.y(), self.z()));
        s
    }
}

pub enum Axis {
    X = 0,
    Y,
    Z,
}

impl Index<Axis> for Vec3 {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.e[0],
            Axis::Y => &self.e[1],
            Axis::Z => &self.e[2],
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, val: f32) -> Self {
        Self {
            e: [self.e[0] * val, self.e[1] * val, self.e[2] * val],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, val: f32) -> Self {
        self * (1.0 / val)
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;
