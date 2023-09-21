use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};


#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self * (1.0 / rhs)
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        self + (-rhs)
    }
}

// Implementing this trait this way only makes sense because we
// know we're only going to print Vec3s into PPM files as colors
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(r, g, b)  = *self * 255.999 as f32;
        
        let r = r as i32;
        let g = g as i32;
        let b = b as i32;

        write!(f, "{r} {g} {b}\n")
    }
}

impl Vec3 {

    pub fn length_squared(self) -> f32 {
        dot(self, self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.0*b.0 + a.1*b.1 + a.2*b.2
}


