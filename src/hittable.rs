use crate::{
    ray::Ray,
    vec3::{dot, Vec3},
};
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> ReturnHitRecord;
}

#[derive(Copy, Clone, Default)]
pub struct ReturnHitRecord {
    pub doesHit: bool,
    pub hitRecord: HitRecord,
}

impl ReturnHitRecord {
    pub fn new(doesHit: bool, hitRecord: HitRecord) -> ReturnHitRecord {
        ReturnHitRecord { doesHit, hitRecord }
    }
}

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl HitRecord {
    fn set_p(&mut self, new_p: Vec3) {
        self.p = new_p;
    }
    fn set_normal(&mut self, new_normal: Vec3) {
        self.normal = new_normal;
    }
    fn set_t(&mut self, new_t: f64) {
        self.t = new_t;
    }
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { p, normal, t }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> ReturnHitRecord {
        let mut rec = HitRecord::new(t_max, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let oc = r.orig - self.center;
        let a = r.dir.lenthSquared();
        let half_b = dot(oc, r.dir);
        let c = oc.lenthSquared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if (temp < t_max && temp > t_min) {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t));
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_normal(outward_normal);
                // rec.set_face_normal(r, outward_normal);
                return ReturnHitRecord::new(true, rec);
            }
            let temp = (-half_b + root) / a;
            if (temp < t_max && temp > t_min) {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t));
                let outward_normal = (rec.p - self.center) / self.radius;
                // rec.set_face_normal(r, outward_normal);
                rec.set_normal(outward_normal);
                return ReturnHitRecord::new(true, rec);
            }
        }
        return ReturnHitRecord::new(false, rec);
    }
}
