use super::vec::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn time(&self) -> f32 {
        self.tm
    }
}
