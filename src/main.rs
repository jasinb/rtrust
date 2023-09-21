use std::env;
use std::fs::File;
use std::io::Write;


mod vec3;
mod ray;
mod hittable;
mod sphere;

use hittable::Hittable;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;

fn ray_color(ray: &Ray) -> Vec3 {
    let sphere = Sphere{ center: Vec3(0.0, 0.0, -1.0), radius: 0.5 };
    if let Some(rec) = sphere.hit(ray, 0.001, 1000.0) {
        return 0.5 * (rec.n + Vec3(1.0, 1.0, 1.0));
    }

    let unit_dir = vec3::unit_vector(ray.dir);
    let a = 0.5 * unit_dir.1 + 1.0;
    (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * Vec3(0.5, 0.7, 1.0)
}


fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = ((image_width as f32) / aspect_ratio) as i32;
    
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32) / image_height as f32;
    let camera_center = Vec3(0.0, 0.0, 0.0);
    let viewport_u = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let offs = pixel_delta_u + pixel_delta_v;
    let half_offs = 0.5 * offs;
    let pixel00_loc = viewport_upper_left +  half_offs;
    
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let mut f = File::create(&args[1]).unwrap();

    write!(&mut f, "P3\n{image_width} {image_height}\n255\n").unwrap();
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + ((i as f32) * pixel_delta_u) + ((j as f32) * pixel_delta_v);
            let ray_dir = pixel_center - camera_center;
            let ray = Ray{orig: camera_center, dir: ray_dir};

            write!(&mut f, "{}", ray_color(&ray)).unwrap();
        }
    }
}
