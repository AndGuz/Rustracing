mod vec;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{sync::Arc, cell::RefCell};
use vec::{Color, Vec3};
mod hit;
mod ray;
mod sphere;
use ray::Ray;
extern crate mini_gl_fb;
mod camera;
mod mat;
use rand::{self, Rng};


use crate::vec::Point3;
use camera::Camera;
use hit::{Hit, World, HitRecord};
use mat::*;
use sphere::{MovingSphere, Sphere};

/*
TODO! dielectricos con tintado
*/
fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f32 = rng.gen();
            let center = Point3::new(
                (a as f32) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f32) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let center2 = center + Vec3::new(0.0, Vec3::random(0.0..0.5).x(), 0.0);
                let sphere = MovingSphere::new(center, center2, 0.0, 1.0, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5, center.x()));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5, 0.0));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let mat4 = Arc::new(DielectricTint::new(1.33, 0.1, Color::random(0.0..1.0)));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);
    let sphere4 = Sphere::new(Point3::new(8.0, 1.0, 0.0), 1.0, mat4);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));
    world.push(Box::new(sphere4));

    world
}

fn front_spheres() -> World {
    let mat1 = Arc::new(DielectricTint::new(1.33, 0.2, Color::new(1.0, 0.5, 0.8)));
    let sphere1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 1.0, mat1);
    let mat2 = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.5)));
    let sphere2 = Sphere::new(Point3::new(-2.0, 0.0, -1.0), 1.0, mat2);
    let mat3 = Arc::new(Metal::new(Color::new(0.4, 0.8, 0.8), 0.0));
    let sphere3 = Sphere::new(Point3::new(-3.0, 0.0, -1.0), 1.0, mat3);
    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1002.0, 0.0), 1000.0, ground_mat);
    let mut world = World::new();
    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(ground_sphere));
    world.push(Box::new(sphere3));
    world
}
fn ray_color(r: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
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
    const ASPECT_RATIO: f32 = 4.0 / 3.0;
    const IMAGE_WIDTH: u32 = 500;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 5;
    const CHUNKS:u32 = 20;

    let (mut event_loop, mut fb) = mini_gl_fb::gotta_go_fast("RTXBROS", IMAGE_WIDTH as f64, IMAGE_HEIGHT as f64);
    let mut buffer = Arc::new(std::sync::Mutex::new(vec![[128u8,0,0,255];(IMAGE_WIDTH*IMAGE_HEIGHT) as usize]));

    let world = random_scene();
    //Camara
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;


    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        35.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    //Salida de ppm
    println!("P3\n{} {}\n 256", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut refresh_count = 0;

    for j in (0..IMAGE_HEIGHT).rev() {
        //eprintln!("\r{} ", j + 1);

        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f32 = rng.gen();
                    let random_v: f32 = rng.gen();

                    let u = ((i as f32) + random_u) / ((IMAGE_WIDTH - 1) as f32);
                    let v = ((j as f32) + random_v) / ((IMAGE_HEIGHT - 1) as f32);

                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                buffer.as_ref().lock().unwrap()[j as usize * IMAGE_WIDTH as usize + i as usize] = pixel_color.format_color_to_array(SAMPLES_PER_PIXEL, (0.0,1.0));
                //buffer[j as usize * IMAGE_WIDTH as usize + i as usize] = pixel_color.format_color_to_array(SAMPLES_PER_PIXEL, (0.0,1.0));
                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL,(0.0,1.0)));
        }

        refresh_count += 1;
        if refresh_count > IMAGE_HEIGHT/CHUNKS{
            refresh_count = 0;
            fb.update_buffer(&buffer.try_lock().unwrap());
        }
    }
    fb.update_buffer(&buffer.try_lock().unwrap());
    fb.persist(&mut event_loop);
    eprintln!("\nTerminado cabros");
}
