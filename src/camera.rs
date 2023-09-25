use std::fs::File;
use std::io::Write;

use crate::vec3::*;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::hittable::Hittable;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_width: i32, image_height: i32, center: Vec3) -> Self {
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32) / image_height as f32;
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);
    
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
    
        let viewport_upper_left = center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let offs = pixel_delta_u + pixel_delta_v;
        let half_offs = 0.5 * offs;
        let pixel00_loc = viewport_upper_left +  half_offs;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Vec3 {
        if let Some(rec) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            return 0.5 * (rec.n + Vec3(1., 1., 1.));
        }

        let unit_dir = unit_vector(ray.dir);
        let a = 0.5 * unit_dir.1 + 1.0;
        (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * Vec3(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &dyn Hittable, f: &mut File) {
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + ((i as f32) * self.pixel_delta_u) + ((j as f32) * self.pixel_delta_v);
                let ray_dir = pixel_center - self.center;
                let ray = Ray{orig: self.center, dir: ray_dir};
    
                write!(f, "{}", self.ray_color(&ray, world)).unwrap();
            }
        }
    
    }
}