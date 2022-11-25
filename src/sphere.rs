use super::hit::{Hit, HitRecord};
use super::mat::Scatter;
use super::ray::Ray;
use super::vec::{Point3, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat: m,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return Some(rec);
    }
}


pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat: Arc<dyn Scatter>,
}

impl MovingSphere {
    pub fn new(cen0: Point3,cen1: Point3, time0:f64,time1:f64,r: f64, m: Arc<dyn Scatter>) -> MovingSphere {
        MovingSphere {
            center0:cen0,
            center1:cen1,
            time0 : time0,
            time1: time1,
            radius: r,
            mat: m
        }
    }
    pub fn center(&self, time:f64) -> Point3{
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return Some(rec);
    }
}
