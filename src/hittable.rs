use crate::vec3::*;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, t: f32, r: &Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = dot(r.dir, outward_normal) < 0.0;
        let n = if front_face { outward_normal } else { -outward_normal };
        HitRecord{ p, n, t, front_face }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin : f32, ray_tmax : f32) -> Option<HitRecord>;
}
