mod color;
mod geometry;

use color::Color;
use geometry::{Point3, Ray, Vec3};

fn get_point_hitting_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<Point3> {
    // (O, d) := ray
    // C := center
    // r := radius
    //
    // v := O - C
    // (v^T d)^2 - |v|^2 + r^2 >= 0

    let origin = &ray.origin;
    let dir = &ray.direction.inject();
    let v = origin.subtract(center);

    let b_half = v.inner_product(dir);
    let c = v.inner_product(&v) - radius * radius;
    let discriminant_quarter = b_half * b_half - c;
    if discriminant_quarter <= 0. {
        None
    } else {
        let t = -b_half + discriminant_quarter.sqrt();
        Some(ray.at(t))
    }
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
    let center = Point3 {
        x: 0.,
        y: 0.,
        z: -1.,
    };
    let radius: f64 = 0.5;
    if let Some(pt) = get_point_hitting_sphere(&center, radius, ray) {
        let u = pt.subtract(&center).unit_vector().inject();
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
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * aspect_ratio;
    let focal_length: f64 = 1.0;

    let origin = Point3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.,
        z: 0.,
    };
    let vertical = Vec3 {
        x: 0.,
        y: viewport_height,
        z: 0.,
    };
    let forward = Vec3 {
        x: 0.,
        y: 0.,
        z: -focal_length,
    };
    let lower_left_corner = origin
        .add(&horizontal.scale(-0.5))
        .add(&vertical.scale(-0.5))
        .add(&forward);

    // Rendering operations:
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining: {}", j + 1);
        for i in 0..image_width {
            let u: f64 = (i as f64) / ((image_width - 1) as f64);
            let v: f64 = (j as f64) / ((image_height - 1) as f64);
            let direction = lower_left_corner
                .add(&horizontal.scale(u))
                .add(&vertical.scale(v))
                .subtract(&origin)
                .unit_vector();
            let ray = Ray {
                origin: origin.clone(),
                direction,
            };
            let color = ray_color(&ray);
            color.write();
        }
    }
    eprintln!("Done.");
}
