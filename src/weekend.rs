use crate::{
    hittable::{HitRecord, Hittable, Sphere},
    ray::Ray,
    vec3::{dot, Vec3},
};
use rand::Rng;
use std::sync::Mutex;
use std::{f64::consts::PI, ops::Deref, sync::Arc};
// inline double degrees_to_radians(double degrees) {
//     return degrees * pi / 180;
// }

fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 1.0)
}

fn random_doubleRange(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}
