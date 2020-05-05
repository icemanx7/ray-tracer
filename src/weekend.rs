use std::sync::Mutex;
use crate::{
    hittable::{HitRecord, Hittable, Sphere},
    ray::Ray,
    vec3::{dot, Vec3},
};
use std::{ops::Deref, sync::Arc, f64::consts::PI};

// inline double degrees_to_radians(double degrees) {
//     return degrees * pi / 180;
// }

fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0
}