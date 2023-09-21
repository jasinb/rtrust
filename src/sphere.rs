use crate::vec3::*;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin : f32, ray_tmax : f32) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }
    
        let p = r.at(root);
        return Some(HitRecord::new(p, root, r, (p - self.center) / self.radius));
    }
}

