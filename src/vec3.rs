use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Sub};
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// trait vec3H {
//     fn x(&self) -> f64;
//     fn y(&self) -> f64;
//     fn z(&self) -> f64;

//     fn lenthSquared(self) -> f64;
//     fn getLength(self) -> f64;
//     fn write_color(self);
// }

impl Vec3 {
    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn lenthSquared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn getLength(self) -> f64 {
        self.lenthSquared().sqrt()
    }

    pub fn write_color(self) {
        let ir: i64 = (255.999 * self.x) as i64;
        let ig: i64 = (255.999 * self.y) as i64;
        let ib: i64 = (255.999 * self.z) as i64;
        println!("{} {} {}", ir, ig, ib)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
