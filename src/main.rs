use crate::weekend::random_double;
use hittable::{HitRecord, Hittable, Sphere};
use hittable_list::hittableList;
use vec3::{random_in_unit_sphere, Vec3};
mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod vec3;
mod weekend;
use camera::Camera;
use rand::Rng;
use ray::Ray;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 384;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;

    println!("{}", "P3");
    println!("{} {}", image_width, image_height);
    println!("{}", "255");

    let hitable1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let hitable2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);

    let world = hittableList::new(vec![Box::new(hitable1), Box::new(hitable2)]);
    let samples_per_pixel = 100;
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
    );
    let max_depth = 50;

    for j in (0..image_height).rev() {
        eprint!("{} {}", "\rlines remaining", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

            for s in 0..samples_per_pixel {
                let u = ((i as f64) + random_double()) / ((image_width - 1) as f64);
                let v = ((j as f64) + random_double()) / ((image_height - 1) as f64);
                let r: ray::Ray = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            pixel_color.write_color(samples_per_pixel);
        }
    }
    println!("{}", "Done");
}

fn ray_color(r: &ray::Ray, world: &hittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let res = world.hit(r, 0.0, std::f64::INFINITY);
    if res.doesHit {
        let target = res.hitRecord.p + res.hitRecord.normal + random_in_unit_sphere();
        return ray_color(
            &Ray::new(res.hitRecord.p, target - res.hitRecord.p),
            world,
            depth - 1,
        );
    }
    let unit_direction = r.direction().unit_vector();
    let t = (unit_direction.y + 1.0) * 0.5;
    return (Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - t))
        + Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * t;
}
