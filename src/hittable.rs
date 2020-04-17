use crate::{
    ray::Ray,
    vec3::{dot, Vec3},
};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
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

    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.dir, outward_normal) < 0.0;
        if (self.front_face) {
            self.normal = outward_normal
        } else {
            self.normal = -outward_normal;
        }
    }
}

impl Sphere {
    fn hit(&mut self, r: Ray, t_min: f64, t_max: f64, mut rec: HitRecord) -> bool {
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
                rec.set_face_normal(r, outward_normal);
                // rec.set_normal((rec.p - self.center) / self.radius);
                return true;
            }
            let temp = (-half_b + root) / a;
            if (temp < t_max && temp > t_min) {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t));
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                // rec.set_normal((rec.p - self.center) / self.radius);
                return true;
            }
        }
        return false;
    }
}
