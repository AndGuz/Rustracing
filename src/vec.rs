use std::collections::btree_map::Range;
use std::fmt;
use std::fmt::Display;
use std::ops;
use rand::{self, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Color = Vec3;
pub type Point3 = Vec3;

#[allow(dead_code)]

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn random(r: ops::Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 { 
            e: [rng.gen_range(r.clone()),rng.gen_range(r.clone()),rng.gen_range(r.clone())] 
        }
    }
    #[inline]
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0{
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else{
            (-1.0) * in_unit_sphere
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    #[inline]
    pub fn format_color(&self, samples_per_pixel: u64) -> String {
        let ir = (256.0 * (self[0] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self[1] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self[2] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl ops::Add for Vec3 {
    fn add(self, rhs: Self) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
    type Output = Vec3;
}

impl ops::Sub for Vec3 {
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
    type Output = Vec3;
}

impl ops::Mul for Vec3 {
    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
    type Output = Vec3;
}

impl ops::Div for Vec3 {
    fn div(self, rhs: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] / rhs.e[0],
                self.e[1] / rhs.e[1],
                self.e[2] / rhs.e[2],
            ],
        }
    }
    type Output = Vec3;
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        };
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        };
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self.e[0] / rhs.e[0],
                self.e[1] / rhs.e[1],
                self.e[2] / rhs.e[2],
            ],
        };
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        };
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other.e[0], self * other.e[1], self * other.e[2]],
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [
                self.e[0] * (1.0_f64 / rhs),
                self.e[1] * (1.0_f64 / rhs),
                self.e[2] * (1.0_f64 / rhs),
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
    type Output = Vec3;
}

impl ops::Div<f64> for Vec3 {
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0_f64 / rhs)
    }
    type Output = Vec3;
}

impl ops::Neg for Vec3 {
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
    type Output = Vec3;
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self[0], self[1], self[2])
    }
}
