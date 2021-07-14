use crate::hittable::HitRecord;
use crate::vec3::{random_unit_vector, reflect};
use crate::{
    ray::Ray,
    vec3::{dot, Vec3},
};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { attenuation: Vec3 },
    Metal { attenuation: Vec3 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: HitRecord) -> MaterialReturn {
        match self {
            Material::Lambertian { attenuation } => {
                let mut scatter_direction: Vec3 = rec.normal + random_unit_vector();
                // Catch degenerate scatter direction
                if (scatter_direction.near_zero()) {
                    scatter_direction = rec.normal;
                }
                let scattered = Ray::new(rec.p, scatter_direction);
                return MaterialReturn::new(true, *attenuation, scattered);
            }
            Material::Metal { attenuation } => {
                let unit_direction = r_in.direction().unit_vector();
                let reflected = reflect(unit_direction, rec.normal);
                let scattered = Ray::new(rec.p, reflected);
                let is_scattered = dot(scattered.direction(), rec.normal) > 0.0;
                return MaterialReturn::new(is_scattered, *attenuation, scattered);
            }
        }
    }
}

pub struct MaterialReturn {
    pub is_scatter: bool,
    pub color: Vec3,
    pub scat_ray: Ray,
}

impl MaterialReturn {
    pub fn new(is_scatter: bool, color: Vec3, scat_ray: Ray) -> MaterialReturn {
        MaterialReturn {
            is_scatter,
            color,
            scat_ray,
        }
    }
}

// // LAMBERT
// #[derive(Copy, Clone)]
// pub struct Lambertian {
//     pub color: Vec3,
// }
//
// impl Lambertian {
//     pub fn new(color: Vec3) -> Lambertian {
//         Lambertian { color }
//     }
// }
//
// impl Material for Lambertian {
//     fn scatter(&self, r_in: &Ray, rec: HitRecord, attenuation: Vec3, scattered: &Ray) -> MaterialReturn {
//         let mut scatter_direction: Vec3 = rec.normal + random_unit_vector();
//         // Catch degenerate scatter direction
//         if (scatter_direction.near_zero()) {
//             scatter_direction = rec.normal;
//         }
//         let scattered = Ray::new(rec.p, scatter_direction);
//         return MaterialReturn::new(true, self.color, scattered);
//     }
// }
//
// // METAL
// #[derive(Copy, Clone)]
// pub struct Metal {
//     pub color: Vec3,
// }
//
// impl Metal {
//     pub fn new(color: Vec3) -> Metal {
//         Metal { color }
//     }
// }
//
// impl Material for Metal {
//     fn scatter(&self, r_in: &Ray, rec: HitRecord, attenuation: Vec3, scattered: &Ray) -> MaterialReturn {
//         let unit_direction = r.direction().unit_vector();
//         let reflected = reflect(unit_direction, rec.normal);
//         let scattered = Ray::new(rec.p, reflected);
//         let is_scattered = dot(scattered.direction(), rec.normal) > 0.0;
//         return MaterialReturn::new(is_scatter, self.color, scattered);
//     }
// }
