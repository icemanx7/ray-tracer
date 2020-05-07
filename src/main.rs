use crate::weekend::random_double;
use hittable::{HitRecord, Hittable, Sphere};
use hittable_list::hittableList;
use vec3::Vec3;
mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod vec3;
mod weekend;
use camera::Camera;
use rand::Rng;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 384;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;

    println!("{}", "P3");
    println!("{} {}", image_width, image_height);
    println!("{}", "255");

    let lower_left_corner: Vec3 = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };

    let horizontal: Vec3 = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };

    let vertical: Vec3 = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };

    let origin: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

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

    for j in (0..image_height).rev() {
        eprint!("{} {}", "\rlines remaining", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

            for s in 0..samples_per_pixel {
                let u = ((i as f64) + random_double()) / ((image_width - 1) as f64);
                let v = ((j as f64) + random_double()) / ((image_height - 1) as f64);
                let r: ray::Ray = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world)
            }

            pixel_color.write_color(samples_per_pixel);
        }
    }
    println!("{}", "Done");
}

fn mult(vect: Vec3, t: f64) -> Vec3 {
    Vec3 {
        x: t * vect.x,
        y: t * vect.y,
        z: t * vect.z,
    }
}

fn hit_sphere(center: Vec3, radius: f64, r: ray::Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir.lenthSquared();
    let half_b = vec3::dot(oc, r.dir);
    let c = oc.lenthSquared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: ray::Ray, world: &hittableList) -> Vec3 {
    let res = world.hit(r, 0.0, std::f64::INFINITY);
    if res.doesHit {
        let n = res.hitRecord.normal;
        return (n + Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }) * 0.5;
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
