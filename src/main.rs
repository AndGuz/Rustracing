mod vec;
use std::{
    io::{stderr, Write},
    rc::Rc,
};
use vec::{Color, Vec3};
mod hit;
mod ray;
mod sphere;
use ray::Ray;
mod camera;
mod mat;
use rand::{self, Rng};

use crate::vec::Point3;
use camera::Camera;
use hit::{Hit, World};
use mat::*;
use sphere::Sphere;

#[inline]
fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    //Imagen por si aca
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 64;
    const MAX_DEPTH: u64 = 10;

    //el mundito
    let mut world = World::new();
    let r: f64 = (std::f64::consts::PI / 4.0).cos();

    let mat_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_right = Rc::new(Lambertian::new(Color::new(1.0,0.0,0.0)));

    let sphere_left = Sphere::new(Point3::new(-r, 0.0, -1.0), r, mat_left);
    let sphere_right = Sphere::new(Point3::new(r, 0.0, -1.0), r, mat_right);

    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));
    //Camara
    let cam = Camera::new(90.0,ASPECT_RATIO);

    //Salida de ppm
    println!("P3\n{} {}\n 256", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rLineas procesadas: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("\nTerminado cabros");
}
