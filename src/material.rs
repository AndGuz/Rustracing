use super::Vec3::Color;
use super::ray::Ray;
use super::hit::HitRecord;

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color,Ray)>;
}