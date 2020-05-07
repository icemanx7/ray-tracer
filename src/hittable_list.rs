use crate::{
    hittable::{HitRecord, Hittable, ReturnHitRecord, Sphere},
    ray::Ray,
    vec3::{dot, Vec3},
};

#[derive(Clone, Default)]
pub struct hittableList {
    pub objects: Vec<Box<Sphere>>,
}

impl hittableList {
    pub fn new(objects: Vec<Box<Sphere>>) -> hittableList {
        hittableList { objects }
    }
}

impl Hittable for hittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> ReturnHitRecord {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = HitRecord::new(
            closest_so_far,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        );

        for obj in self.objects.iter() {
            let objTemps = obj.hit(r, t_min, closest_so_far);
            if objTemps.doesHit {
                hit_anything = true;
                closest_so_far = objTemps.hitRecord.t;
                rec = objTemps.hitRecord;
            }
        }
        ReturnHitRecord::new(hit_anything, rec)
    }
}
