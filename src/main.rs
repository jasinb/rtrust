use std::env;
use std::fs::File;
use std::io::Write;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod interval;
mod camera;
mod material;

use hittable::HittableList;
use vec3::Vec3;
use sphere::Sphere;
use camera::Camera;
use material::*;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f32) / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(image_width, image_height, samples_per_pixel, Vec3::zero());

    let mat_a = Lambertian::new(Vec3(1.0, 0.4, 0.1));
    let mat_b = Lambertian::new(Vec3(0.3, 0.7, 1.0));

    let world = HittableList::new()
        .add(Box::new(Sphere{ center: Vec3(0., 0., -1.), radius: 0.5, material: &mat_a }))
        .add(Box::new(Sphere{ center: Vec3(0., -100.5, -1.), radius: 100.0, material: &mat_b }));

    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let mut f = File::create(&args[1]).unwrap();

    write!(&mut f, "P3\n{image_width} {image_height}\n255\n").unwrap();
    camera.render(&world, &mut f, max_depth);
}
