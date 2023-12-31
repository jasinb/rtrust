use std::fs::File;
use std::io::Write;
use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg};
use crate::random::*;


#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
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

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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

fn linear_to_gamma(linear: f32) -> f32 {
    f32::sqrt(linear)
}

impl Vec3 {
    pub fn write_color(self, f: &mut File, samples_per_pixel: i32)  {
        let Self(r, g, b)  = self / samples_per_pixel as f32;
        
        let r = (linear_to_gamma(r).clamp(0., 0.999) * 256.0) as i32;
        let g = (linear_to_gamma(g).clamp(0., 0.999) * 256.0) as i32;
        let b = (linear_to_gamma(b).clamp(0., 0.999) * 256.0) as i32;

        write!(f, "{r} {g} {b}\n").unwrap();
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn zero() -> Self {
        Self(0., 0., 0.)
    }

    pub fn one() -> Self {
        Self(1., 1., 1.)
    }

    pub fn near_zero(self) -> bool {
        const S: f32 = 1e-8;
        f32::abs(self.0) < S && f32::abs(self.0) < S && f32::abs(self.0) < S
    }

    pub fn random(min: f32, max: f32) -> Self {
        Self(random_float(min, max), random_float(min, max), random_float(min, max))
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let v = Self(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
            if v.length_squared() < 1.0 {
                return v;
            }

        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random(-1.0, 1.0);
            assert!(v.0 >= -1.0 && v.0 < 1.0);
            assert!(v.1 >= -1.0 && v.1 < 1.0);
            assert!(v.2 >= -1.0 && v.2 < 1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 { on_unit_sphere } else { -on_unit_sphere }
    }

    pub fn reflect(self, n: Self) -> Self {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Self, etai_over_etat: f32) -> Self {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn dot(self, b: Vec3) -> f32 {
        self.0*b.0 + self.1*b.1 + self.2*b.2
    }

    pub fn cross(self, v: Self) -> Self {
        Self(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0)
    }
}


