use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material + 'a>,
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
    
        let p = ray.at(root);
        Some(HitRecord::new(p, root, ray, (p - self.center) / self.radius, &(*self.material)))
    }
}

