use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        const FOCAL_LENGTH: f64 = 1.0;
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta/2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let orig = Point3::new(0.0, 0.0, 0.0);
        let h = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, viewport_height, 0.0);
        let llc = orig - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            origin: orig,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
