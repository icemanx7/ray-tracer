use crate::{
    hittable::{HitRecord, Hittable, Sphere},
    ray::Ray,
    vec3::{dot, Vec3},
};
use std::{ops::Deref, sync::Arc};

pub struct hittableList {
    pub objects: Vec<Arc<Sphere>>,
}

impl Deref for hittableList {
    type Target = Vec<Arc<Sphere>>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl hittableList {
    fn clear(&mut self) {
        self.objects.clear();
    }
    fn add(&mut self, object: Arc<Sphere>) {
        self.objects.push(object);
    }
}

impl Hittable for hittableList {
    fn hit(&mut self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool {
        let temp_rec: HitRecord;
        let hit_anything = false;
        let closest_so_far = t_max;

        for obj in &self.objects {
            if *obj.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
            }
        }
        true
    }
}
