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
    pub fn new(albedo: Vec3) -> Self {
        Self{ albedo }
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

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self{ albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(ray.dir.unit(), rec.n);
        let scattered = Ray{ orig: rec.p, dir: reflected + self.fuzz * Vec3::random_unit_vector() };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self{ ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let refraction_ratio = if rec.front_face { 1. / self.ir } else { self.ir };

        let unit_direction = ray.dir.unit();
        let cos_theta = Vec3::dot(-unit_direction, rec.n).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            Vec3::reflect(unit_direction, rec.n)
        }
        else {
            Vec3::refract(unit_direction, rec.n, refraction_ratio)
        };
        
        let attenuation = Vec3::one();
        let scattered = Ray { orig: rec.p, dir: direction };
        Some((attenuation, scattered))
    }
}
