use vec3::Vec3;
mod vec3;

fn main() {
    let image_width: i32 = 200;
    let image_height: i32 = 200;

    println!("{}", "P3");
    println!("{} {}", image_width, image_height);
    println!("{}", "255");

    for j in (0..image_height).rev() {
        eprint!("{} {}", "\rlines remaining", j);
        for i in 0..image_width {
            let vecTest: Vec3 = Vec3 {
                x: (i as f64) / (image_width as f64),
                y: (j as f64) / (image_height as f64),
                z: 0.2,
            };

            vecTest.write_color();
            // let r: f64 = (i as f64) / (image_width as f64);
            // let g: f64 = (j as f64) / (image_height as f64);
            // let b: f64 = 0.2;

            // let ir: i64 = (255.999 * r) as i64;
            // let ig: i64 = (255.999 * g) as i64;
            // let ib: i64 = (255.999 * b) as i64;
            // println!("{} {} {}", ir, ig, ib)
        }
    }
    println!("{}", "Done");
}
