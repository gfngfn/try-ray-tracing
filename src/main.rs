extern crate rand;

use rand::Rng;

mod camera;
mod color;
mod geometry;
mod hittable_object;

use camera::Camera;
use color::Color;
use geometry::{Point3, Ray, UnitVec3, Vec3};
use hittable_object::{Hittable, HittableList, Sphere};

/// Returns a random double in [-0.5, 0.5).
fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-0.5..0.5)
}

fn random_unit_vector() -> UnitVec3 {
    let v = Vec3 {
        x: random_double(),
        y: random_double(),
        z: random_double(),
    };
    v.unit_vector()
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

fn ray_color(ray: &Ray, world: &dyn Hittable, diffusion_depth: i32) -> Color {
    if diffusion_depth <= 0 {
        Color {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    } else {
        if let Some(hit) = world.hit(ray) {
            let surface_normal = hit.surface_normal.inject();
            let child_ray = Ray {
                origin: ray.at(hit.t),
                direction: surface_normal
                    .add(&random_unit_vector().inject())
                    .unit_vector(),
            };
            let color = ray_color(&child_ray, world, diffusion_depth - 1);
            color.scale(0.5)
        } else {
            ray_background_color(ray)
        }
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

    // Constants for diffusion:
    let max_diffusion_depth = 3;

    // Hittable objects:
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
                let color = ray_color(&ray, &hittable_list, max_diffusion_depth);
                colors.push(color);
            }
            let color = Color::average(&colors);
            color.write();
        }
    }
    eprintln!("Done.");
}
