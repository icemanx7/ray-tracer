use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    fn origin(self) -> Vec3 {
        self.orig
    }

    fn direction(self) -> Vec3 {
        self.dir
    }

    fn mult(vect: Vec3, t: f64) -> Vec3 {
        Vec3 {
            x: t * vect.x,
            y: t * vect.y,
            z: t * vect.z,
        }
    }

    //Refactor this
    fn at(self, t: f64) -> Vec3 {
        let pVec = Vec3 {
            x: t * self.dir.x,
            y: t * self.dir.y,
            z: t * self.dir.z,
        };
        return self.orig + pVec;
    }
}
