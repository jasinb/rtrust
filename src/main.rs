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
use random::*;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = ((image_width as f32) / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let camera = Camera::new(
        image_width,
        image_height, 
        samples_per_pixel,
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        0.6,
        10.0);

    let mut world = HittableList::new();
    let ground_material = Lambertian::new(Vec3(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere{ center: Vec3( 0., -1000.0,  0.0), radius: 1000.0, material: Box::new(ground_material) }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float(0.0, 1.0);
            
            let center = Vec3(a as f32 + 0.9 * random_float(0.0, 1.0), 0.2, b as f32 + 0.9 * random_float(0.0, 1.0));
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material : Box<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    sphere_material = Box::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = random_float(0.0, 0.5);
                    sphere_material = Box::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Box::new(Dielectric::new(1.5));
                }
                world.add(Box::new(Sphere{ center, radius: 0.2, material: sphere_material }));
            }
        }
    }
 
    let material1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere{ center: Vec3(0.0, 1.0, 0.0), radius: 1.0, material: material1 }));


    let material2 = Box::new(Lambertian::new(Vec3(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere{ center: Vec3(-4.0, 1.0, 0.0), radius: 1.0, material: material2 }));

    let material3 = Box::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere{ center: Vec3(4.0, 1.0, 0.0), radius: 1.0, material: material3 }));

    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let mut f = File::create(&args[1]).unwrap();

    write!(&mut f, "P3\n{image_width} {image_height}\n255\n").unwrap();
    camera.render(&world, &mut f, max_depth);
}
