use crate::weekend::random_doubleRange;
use crate::weekend::{clamp, random_double};
use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

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

    pub fn write_color(self, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f64;

        let r = scale * self.x;
        let g = scale * self.y;
        let b = scale * self.z;

        let ir: i64 = (255.999 * clamp(r, 0.0, 0.999)) as i64;
        let ig: i64 = (255.999 * clamp(g, 0.0, 0.999)) as i64;
        let ib: i64 = (255.999 * clamp(b, 0.0, 0.999)) as i64;

        println!("{} {} {}", ir, ig, ib)
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.getLength()
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn randomRange(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_doubleRange(min, max),
            random_doubleRange(min, max),
            random_doubleRange(min, max),
        )
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::randomRange(-1.0, 1.0);
    while p.lenthSquared() >= 1.0 {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
        continue;
    }
    return p;
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
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
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

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}
