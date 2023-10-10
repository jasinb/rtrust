use std::env;
use std::fs::File;
use std::io::Write;

mod random;
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
    let max_depth = 10;

    let camera = Camera::new(
        image_width,
        image_height, 
        samples_per_pixel,
        Vec3(-2.0, 2.0, 1.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        10.0,
        3.4);

    let mat_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Vec3(0.1, 0.2, 0.5));
    let mat_left = Dielectric::new(1.5);
    let mat_right = Metal::new(Vec3(0.8, 0.6, 0.2), 0.0);

    let world = HittableList::new()
        .add(Box::new(Sphere{ center: Vec3( 0., -100.5,  -1.), radius: 100.0, material: &mat_ground }))
        .add(Box::new(Sphere{ center: Vec3( 0.,      0., -1.), radius:   0.5, material: &mat_center }))
        .add(Box::new(Sphere{ center: Vec3(-1.,      0., -1.), radius:   0.5, material: &mat_left }))
        .add(Box::new(Sphere{ center: Vec3(-1.,      0., -1.), radius:  -0.4, material: &mat_left }))
        .add(Box::new(Sphere{ center: Vec3( 1.,      0., -1.), radius:   0.5, material: &mat_right }));
 
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let mut f = File::create(&args[1]).unwrap();

    write!(&mut f, "P3\n{image_width} {image_height}\n255\n").unwrap();
    camera.render(&world, &mut f, max_depth);
}
