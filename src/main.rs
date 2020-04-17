use vec3::Vec3;
mod hittable;
mod ray;
mod vec3;

fn main() {
    let image_width: i32 = 200;
    let image_height: i32 = 100;

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

    for j in (0..image_height).rev() {
        eprint!("{} {}", "\rlines remaining", j);
        for i in 0..image_width {
            let u = (i as f64) / (image_width as f64);
            let v = (j as f64) / (image_height as f64);

            let r: ray::Ray = ray::Ray {
                orig: origin,
                dir: lower_left_corner + mult(horizontal, u) + mult(vertical, v),
            };

            let color = ray_color(r);

            color.write_color();
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
    // let b = 2.0 * vec3::dot(oc, r.dir);
    let c = oc.lenthSquared() - radius * radius;
    // let discriminant = half_b * half_b - (4.0 * a * c);
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: ray::Ray) -> Vec3 {
    let sp = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let t = hit_sphere(sp, 0.5, r);
    if t > 0.0 {
        let temp: Vec3 = r.at(t)
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            };
        let N = temp.unit_vector();
        let fVec = Vec3 {
            x: N.x + 1.0,
            y: N.y + 1.0,
            z: N.z + 1.0,
        };
        return fVec * 0.5;
    }
    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - t)
        + Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * t;
}
