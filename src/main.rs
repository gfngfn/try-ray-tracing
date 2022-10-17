mod camera;
mod color;
mod geometry;
mod hittable_object;

use camera::Camera;
use color::{Attenuation, Color};
use geometry::{random_double, Point3, Ray, Vec3};
use hittable_object::{Glass, Hittable, HittableList, Lambertian, Metal, Sphere};

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
        if let Some((hit, material)) = world.hit(ray) {
            let (attenuation, child_ray) = material.scatter(ray, &hit);
            let color = ray_color(&child_ray, world, diffusion_depth - 1);
            color.attenuate(&attenuation)
        } else {
            ray_background_color(ray)
        }
    }
}

/// Performs Gamma Correction.
fn filter_color(color: &Color) -> Color {
    Color {
        r: color.r.sqrt(),
        g: color.g.sqrt(),
        b: color.b.sqrt(),
    }
}

fn oxygen(x: f64, y: f64, z: f64) -> Box<dyn Hittable> {
    Box::new(Sphere {
        center: Point3 { x, y, z },
        radius: 0.3,
        material: Box::new(Glass {
            eta: 1.5,
            albedo: Attenuation {
                r: 0.9,
                g: 0.5,
                b: 0.5,
            },
        }),
    })
}

fn carbon(x: f64, y: f64, z: f64) -> Box<dyn Hittable> {
    Box::new(Sphere {
        center: Point3 { x, y, z },
        radius: 0.35,
        material: Box::new(Metal {
            albedo: Attenuation {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            },
            fuzz: 0.1,
        }),
    })
}

fn hydrogen(x: f64, y: f64, z: f64) -> Box<dyn Hittable> {
    Box::new(Sphere {
        center: Point3 { x, y, z },
        radius: 0.25,
        material: Box::new(Lambertian {
            albedo: Attenuation {
                r: 0.8,
                g: 0.8,
                b: 0.9,
            },
        }),
    })
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
        z: 0.5,
    };
    let look_in = Vec3 {
        x: 0.,
        y: 0.,
        z: -1.,
    }
    .unit_vector();
    let view_up = Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    };

    let vertical_fov_radian = std::f64::consts::PI / 1.5;

    let camera = Camera::new(origin, look_in, view_up, vertical_fov_radian, aspect_ratio);

    // Constants for antialiasing:
    let num_samples_per_pixel = 100;

    // Constants for diffusion:
    let max_diffusion_depth = 10;

    // Hittable objects:
    /*
        let sphere1 = Sphere {
            center: Point3 {
                x: -1.,
                y: 0.,
                z: -1.,
            },
            radius: 0.5,
            material: Box::new(Lambertian {
                albedo: Attenuation {
                    r: 0.8,
                    g: 0.5,
                    b: 0.5,
                },
            }),
        };
        let sphere2 = Sphere {
            center: Point3 {
                x: 1.,
                y: 0.,
                z: -1.,
            },
            radius: 0.5,
            material: Box::new(Metal {
                albedo: Attenuation {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                },
                fuzz: 0.3,
            }),
        };
        let sphere3 = Sphere {
            center: Point3 {
                x: 0.,
                y: 0.,
                z: -1.,
            },
            radius: 0.5,
            material: Box::new(Glass {
                eta: 1.5,
                albedo: Attenuation {
                    r: 0.9,
                    g: 0.9,
                    b: 0.9,
                },
            }),
        };
    */
    let ground = Sphere {
        center: Point3 {
            x: 0.,
            y: -100.5,
            z: -1.,
        },
        radius: 100.,
        material: Box::new(Lambertian {
            albedo: Attenuation {
                r: 0.2,
                g: 0.4,
                b: 0.2,
            },
        }),
    };
    let (x1, y1, z1) = (0f64, 0f64, -1f64);
    let len_oh = 0.11;
    let len_ch = 0.14;
    let len_co = 0.2;
    let hittable_list = HittableList {
        members: vec![
            carbon(x1, y1, z1),
            oxygen(x1 + len_co, y1 + len_co, z1 + len_co),
            hydrogen(
                x1 + len_co + len_oh,
                y1 + len_co - len_oh,
                z1 + len_co + len_oh,
            ),
            hydrogen(x1 + len_ch, y1 - len_ch, z1 - len_ch),
            hydrogen(x1 - len_ch, y1 - len_ch, z1 + len_ch),
            hydrogen(x1 - len_ch, y1 + len_ch, z1 - len_ch),
            Box::new(ground),
        ],
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
            filter_color(&color).write();
        }
    }
    eprintln!("Done.");
}
