use crate::{ray::Ray, vec3};
use vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
    pub fn new(origin: Vec3, horizontal: Vec3, vertical: Vec3, lower_left_corner: Vec3) -> Camera {
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
