use std::fs::File;

use crate::vec3::*;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::random::*;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
            image_width: i32,
            image_height: i32,
            samples_per_pixel: i32,
            lookfrom: Vec3,
            lookat: Vec3,
            vup: Vec3,
            vfov: f32,
            defocus_angle: f32,
            focus_dist: f32) -> Self {
        let center = lookfrom;
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32) / image_height as f32;
        
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
    
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
    
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let offs = pixel_delta_u + pixel_delta_v;
        let half_offs = 0.5 * offs;
        let pixel00_loc = viewport_upper_left +  half_offs;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        
        Self {
            image_width,
            image_height,
            samples_per_pixel,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        if let Some(rec) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }
            return Vec3::zero();
        }

        let unit_dir = Vec3::unit(ray.dir);
        let a = 0.5 * unit_dir.1 + 1.0;
        (1.0 - a) * Vec3::one() + a * Vec3(0.5, 0.7, 1.0)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = random_float(-0.5, 0.5);
        let py = random_float(-0.5, 0.5);

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.0 * self.defocus_disk_u) + (p.1 * self.defocus_disk_v)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + ((i as f32) * self.pixel_delta_u) + ((j as f32) * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_dir = pixel_sample - ray_origin;
        Ray { orig: ray_origin, dir: ray_dir }
    }

    pub fn render(&self, world: &dyn Hittable, f: &mut File, max_depth: i32) {
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                
                let mut color = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += self.ray_color(&ray, world, max_depth);
                }
                color.write_color(f, self.samples_per_pixel);
            }
        }
    
    }
}