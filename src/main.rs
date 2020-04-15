use vec3::Vec3;
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

fn ray_color(r: ray::Ray) -> Vec3 {
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
