use std::ops::{Add, Sub, Mul, DivAssign, AddAssign, BitAnd, MulAssign};
pub struct Vec3 {
   pub x: f64,
   pub y: f64,
   pub z: f64
}

trait vec3H {

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    fn lenthSquared();


}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl MulAssign<f64> for Vec3 {

    fn mul_assign(&mut self, rhs: f64) {
       self.x *= rhs;
       self.y *= rhs;
       self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, rhs: f64){
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
