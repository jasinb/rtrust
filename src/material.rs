use rand::Rng;
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
        let reflected = ray.dir.unit().reflect(rec.n);
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
        let cos_theta = (-unit_direction).dot(rec.n).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let rnd = rand::thread_rng().gen::<f32>();
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rnd {
            unit_direction.reflect(rec.n)
        }
        else {
            unit_direction.refract(rec.n, refraction_ratio)
        };
        
        let attenuation = Vec3::one();
        let scattered = Ray { orig: rec.p, dir: direction };
        Some((attenuation, scattered))
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}