use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{ albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.n + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.n;
        }
        let scattered = Ray{ orig: rec.p, dir: scatter_direction };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}