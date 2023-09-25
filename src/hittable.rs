use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HitRecord {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, t: f32, r: &Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        let n = if front_face { outward_normal } else { -outward_normal };
        HitRecord{ p, n, t, front_face }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}


pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { hittables: Default::default() }
    }
    pub fn add(mut self, hittable: Box<dyn Hittable>) -> Self {
        self.hittables.push(hittable);
        self
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_anything = false;
        // dummy initialization to silence compiler warning
        let mut record = HitRecord {
            p: Vec3(0., 0., 0.),
            n:  Vec3(0., 0., 0.),
            t: f32::INFINITY,
            front_face: false,
        };

        for hittable in self.hittables.iter() {
            if let Some(rec) = hittable.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                hit_anything = true;
                closest_so_far = rec.t;
                record = rec;
            }
        }
        if !hit_anything {
            return None;
        }
        Some(record)
    }    
}