extern crate rand;

use rand::Rng;

mod camera;
mod color;
mod geometry;
mod hittable_object;

use camera::Camera;
use color::Color;
use geometry::{Point3, Ray};
use hittable_object::{Hittable, HittableList, Sphere};

fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-0.5..0.5)
}

fn ray_background_color(ray: &Ray) -> Color {
    let u = &ray.direction;
    let t = 0.5 * (u.inject().y + 1.);
    let white = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };
    let sky = Color {
        r: 0.5,
        g: 0.7,
        b: 1.,
    };
    white.blend(t, &sky)
}

fn ray_color(ray: &Ray) -> Color {
    let sphere1 = Sphere {
        center: Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: Point3 {
            x: 0.,
            y: -100.5,
            z: -1.,
        },
        radius: 100.,
    };
    let hittable_list = HittableList {
        members: vec![Box::new(sphere1), Box::new(sphere2)],
    };
    if let Some(hit) = hittable_list.hit(ray) {
        let u = hit.surface_normal.inject();
        Color {
            r: 0.5 * (u.x + 1.),
            g: 0.5 * (u.y + 1.),
            b: 0.5 * (u.z + 1.),
        }
    } else {
        ray_background_color(ray)
    }
}

fn main() {
    // Constants for the image:
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;

    // Constants for the camera:
    let origin = Point3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let viewport_height: f64 = 2.0;
    let focal_length: f64 = 1.0;

    let camera = Camera::new(aspect_ratio, origin, viewport_height, focal_length);

    // Constants for antialiasing:
    let num_samples_per_pixel = 10;

    // Rendering operations:
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining: {}", j + 1);
        for i in 0..image_width {
            let mut colors: Vec<Color> = vec![];
            for _ in 0..num_samples_per_pixel {
                let u: f64 = (i as f64 + random_double()) / ((image_width - 1) as f64);
                let v: f64 = (j as f64 + random_double()) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);
                let color = ray_color(&ray);
                colors.push(color);
            }
            let color = Color::average(&colors);
            color.write();
        }
    }
    eprintln!("Done.");
}
