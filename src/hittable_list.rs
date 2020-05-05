use std::sync::Mutex;
use crate::{
    hittable::{HitRecord, Hittable, Sphere},
    ray::Ray,
    vec3::{dot, Vec3},
};
use std::{ops::Deref, sync::Arc};


#[derive(Clone, Default)]
pub struct hittableList {
    pub objects: Vec<Arc<Mutex<Sphere>>>,
}

// impl Deref for hittableList {
//     type Target = Vec<Arc<Mutex<Sphere>>>;

//     // fn deref(&self) -> &Self::Target {
//     //     &self.objects
//     // }
// }

impl hittableList {
    fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<Mutex<Sphere>>) {
        self.objects.push(object);
    }
}

impl Hittable for hittableList {
    fn hit(&mut self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            let mut objTemps = obj.lock().unwrap();
            if objTemps.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                temp_rec = rec;
            }
        }
        hit_anything
    }
}
