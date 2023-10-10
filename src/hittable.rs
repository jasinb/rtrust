use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, t: f32, r: &Ray, outward_normal: Vec3, material: &'a dyn Material) -> HitRecord<'a> {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let n = if front_face { outward_normal } else { -outward_normal };
        HitRecord{ p, n, t, front_face, material }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}


pub struct HittableList<'a> {
    hittables: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self { hittables: Default::default() }
    }
    pub fn add(&mut self, hittable: Box<dyn Hittable + 'a>) -> &Self {
        self.hittables.push(hittable);
        self
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Why no mut????
        let mut closest_so_far = ray_t.max;
        let mut result: Option<HitRecord> = None;

        for hittable in self.hittables.iter() {
            if let Some(rec) = hittable.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }
        result
    }    
}