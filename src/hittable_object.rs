extern crate dyn_clone;

use dyn_clone::DynClone;

use crate::color::Attenuation;
use crate::geometry::{random_double, random_unit_vector, reflect_vector, Point3, Ray, UnitVec3};

/// The type for intersection points; see `Hittable` for the usage of this type.
#[derive(Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub surface_normal: UnitVec3,
}

/// The trait for surface materials.
pub trait Material: DynClone {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> (Attenuation, Ray);
}

/// The type for materials that perform Lambertian reflectance.
#[derive(Clone, Debug, PartialEq)]
pub struct Lambertian {
    pub albedo: Attenuation,
}
impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> (Attenuation, Ray) {
        let surface_normal = hit.surface_normal.inject();
        let scattered_direction = surface_normal.add(&random_unit_vector().inject());
        let child_ray = Ray {
            origin: ray_in.at(hit.t),
            direction: scattered_direction.unit_vector(),
            // TODO: make this work even when `scattered_direction` is close to the zero vector
        };
        (self.albedo.clone(), child_ray)
    }
}

/// The type for metals, i.e., materials that perform the regular reflection.
#[derive(Clone)]
pub struct Metal {
    pub albedo: Attenuation,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> (Attenuation, Ray) {
        let direction_raw = reflect_vector(&ray_in.direction, &hit.surface_normal);
        let direction = direction_raw
            .inject()
            .add(&random_unit_vector().inject().scale(self.fuzz))
            .unit_vector();
        let child_ray = Ray {
            origin: ray_in.at(hit.t),
            direction,
        };
        (self.albedo.clone(), child_ray)
    }
}

pub type BoxedMaterial = Box<dyn Material>;
impl Clone for BoxedMaterial {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1. - refraction_index) / (1. + refraction_index);
    let r1 = r0 * r0;
    r1 + (1. - r1) * (1. - cosine).powi(5)
}

/// The type for glasses, i.e., materials that perform refraction.
/// The parameter `eta` is the refractive index and should >= 1.
#[derive(Clone)]
pub struct Glass {
    pub eta: f64,
    pub albedo: Attenuation,
}
impl Material for Glass {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> (Attenuation, Ray) {
        let normal_raw = hit.surface_normal.inject();
        let direction_in = ray_in.direction.inject();
        let inprod_raw = normal_raw.inner_product(&direction_in);

        // TODO: generalize the refractive index of external spaces.
        let (normal, inprod, eta_in, eta_out) = {
            if inprod_raw < 0. {
                // If `ray_in` is coming into the object from the outside:
                (normal_raw, inprod_raw, 1., self.eta)
            } else {
                // If `ray_in` is going out of the object from the inside:
                (normal_raw.scale(-1.), -inprod_raw, self.eta, 1.)
            }
        };

        // v := d - (n^T d) n
        let vp_in = direction_in.subtract(&normal.scale(inprod));

        // v' := (eta / eta') v
        let vp_out = vp_in.scale(eta_in / eta_out);

        // c := 1 - |v'|^2
        let coeff_normal = 1. - vp_out.length_squared();

        let direction_out = {
            if coeff_normal >= 0. {
                // If the light can refract:

                if reflectance(-inprod, eta_in / eta_out) > random_double() {
                    reflect_vector(&ray_in.direction, &normal.unit_vector())
                } else {
                    // d' = v' - sqrt(c) n
                    vp_out
                        .subtract(&normal.scale(coeff_normal.sqrt()))
                        .unit_vector()
                }
            } else {
                // If the light cannot refract and performs regular reflection:
                reflect_vector(&ray_in.direction, &normal.unit_vector())
            }
        };
        let ray = Ray {
            origin: ray_in.at(hit.t),
            direction: direction_out,
        };
        (self.albedo.clone(), ray)
    }
}

/// The trait for objects hittable by rays.
pub trait Hittable {
    /// Checks that `ray` intersects with the object.
    /// Returns `Some((hit, material))` if it does
    /// where `hit` is the information about the intersection point
    /// and `material` is the surface material of that point,
    /// or returns `None` otherwise.
    fn hit(&self, ray: &Ray) -> Option<(HitRecord, Box<dyn Material>)>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: BoxedMaterial,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<(HitRecord, Box<dyn Material>)> {
        let t_min = 0.01; // This should be set in order for rays after reflection not to hit the sphere itself.

        let center = &self.center;
        let radius = &self.radius;

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
        let c = v.length_squared() - radius * radius;
        let discriminant_quarter = b_half * b_half - c;
        let t_opt = {
            if discriminant_quarter < 0. {
                // If the ray does not hit the object at any point:
                None
            } else {
                let sqrt_of_discriminant_quarter = discriminant_quarter.sqrt();
                let t_minus = -b_half - sqrt_of_discriminant_quarter;
                if t_minus >= t_min {
                    // If the ray hits the surface from the outside:
                    Some(t_minus)
                } else {
                    let t_plus = -b_half + sqrt_of_discriminant_quarter;
                    if t_plus >= t_min {
                        // If the ray hits the surface from the inside:
                        Some(t_plus)
                    } else {
                        None
                    }
                }
            }
        };
        match t_opt {
            None => None,
            Some(t) => {
                let intersection_point = ray.at(t);
                let surface_normal = intersection_point.subtract(&center).unit_vector();
                Some((HitRecord { t, surface_normal }, self.material.clone()))
            }
        }
    }
}

pub struct HittableList {
    pub members: Vec<Box<dyn Hittable>>,
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray) -> Option<(HitRecord, BoxedMaterial)> {
        let mut maybe_nearest: Option<(HitRecord, BoxedMaterial)> = None;
        for hittable in self.members.iter() {
            if let Some(pair) = hittable.hit(ray) {
                let (hit, _material) = &pair;
                if let Some(nearest) = &maybe_nearest {
                    let (nearest_hit, _) = &nearest;
                    if hit.t < nearest_hit.t {
                        maybe_nearest = Some(pair);
                    }
                } else {
                    maybe_nearest = Some(pair);
                }
            }
        }
        maybe_nearest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vec3;

    fn create_dummy_material() -> BoxedMaterial {
        let albedo = Attenuation {
            r: 0.5,
            g: 0.5,
            b: 0.5,
        };
        let material = Lambertian { albedo };
        Box::new(material)
    }

    #[test]
    fn sphere_test1() {
        let sphere = Sphere {
            center: Point3 {
                x: 0.,
                y: 0.,
                z: -3.,
            },
            radius: 1.,
            material: create_dummy_material(),
        };
        let ray = Ray {
            origin: Point3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            direction: Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            }
            .unit_vector(),
        };
        let expected_hit = HitRecord {
            t: 2.,
            surface_normal: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            }
            .unit_vector(),
        };
        match sphere.hit(&ray) {
            Some((got_hit, _)) => {
                assert_eq!(expected_hit, got_hit);
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn sphere_test2() {
        let sphere = Sphere {
            center: Point3 {
                x: 0.,
                y: 0.,
                z: -8.,
            },
            radius: 5.,
            material: create_dummy_material(),
        };
        let ray = Ray {
            origin: Point3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            direction: Vec3 {
                x: -0.6,
                y: 0.,
                z: -0.8,
            }
            .unit_vector(),
        };
        let expected_hit = HitRecord {
            t: 4.999999999999997, // Ideally `5.`
            surface_normal: Vec3 {
                x: -0.5999999999999996, // Ideally `-0.6`
                y: 0.,
                z: 0.8000000000000004, // Ideally `0.8`
            }
            .unit_vector(),
        };
        match sphere.hit(&ray) {
            Some((got_hit, _)) => {
                assert_eq!(expected_hit, got_hit);
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn sphere_test3() {
        let sphere = Sphere {
            center: Point3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 5.,
            material: create_dummy_material(),
        };
        let ray = Ray {
            origin: Point3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            direction: Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            }
            .unit_vector(),
        };
        let expected_hit = HitRecord {
            t: 5.,
            surface_normal: Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            }
            .unit_vector(),
        };
        match sphere.hit(&ray) {
            Some((got_hit, _)) => {
                assert_eq!(expected_hit, got_hit);
            }
            None => {
                assert!(false);
            }
        }
    }

    fn make_dummy_attenuation() -> Attenuation {
        Attenuation {
            r: 0.8,
            g: 0.8,
            b: 0.8,
        }
    }

    #[test]
    fn glass_scatter_test1() {
        let glass = Glass {
            eta: 1.0,
            albedo: make_dummy_attenuation(),
        };
        let ray_in = Ray {
            origin: Point3 {
                x: -3.,
                y: 4.,
                z: 0.,
            },
            direction: Vec3 {
                x: 0.6,
                y: -0.8,
                z: 0.,
            }
            .unit_vector(),
        };
        let hit = HitRecord {
            t: 5.,
            surface_normal: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            }
            .unit_vector(),
        };
        let expected_ray_out = Ray {
            origin: Point3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            direction: Vec3 {
                x: 0.6,
                y: -0.8,
                z: 0.,
            }
            .unit_vector(),
        };
        let (_attenuation, ray_out) = glass.scatter(&ray_in, &hit);
        assert_eq!(expected_ray_out, ray_out);
    }

    #[test]
    fn glass_scatter_test2() {
        let glass = Glass {
            eta: 3f64.sqrt(),
            albedo: make_dummy_attenuation(),
        };
        let ray_in = Ray {
            origin: Point3 {
                x: -3f64.sqrt(),
                y: 1.,
                z: 0.,
            },
            direction: Vec3 {
                x: 3f64.sqrt(),
                y: -1.,
                z: 0.,
            }
            .unit_vector(),
        };
        let hit = HitRecord {
            t: 2.,
            surface_normal: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            }
            .unit_vector(),
        };
        let expected_ray_out = Ray {
            origin: Point3 {
                x: 2.220446049250313e-16,  // Ideally `0.`
                y: -2.220446049250313e-16, // Ideally `0.`
                z: 0.,
            },
            direction: Vec3 {
                x: 0.5000000000000001,  // Ideally `0.5`
                y: -0.8660254037844386, // Ideally `-3f64.sqrt() / 2.`
                z: 0.,
            }
            .unit_vector(),
        };
        let (_attenuation, ray_out) = glass.scatter(&ray_in, &hit);
        assert_eq!(expected_ray_out, ray_out);
    }

    #[test]
    fn glass_scatter_test3() {
        let glass = Glass {
            eta: 3f64.sqrt(),
            albedo: make_dummy_attenuation(),
        };
        let ray_in = Ray {
            origin: Point3 {
                x: -1.,
                y: -3f64.sqrt(),
                z: 0.,
            },
            direction: Vec3 {
                x: 1.,
                y: 3f64.sqrt(),
                z: 0.,
            }
            .unit_vector(),
        };
        let hit = HitRecord {
            t: 2.,
            surface_normal: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            }
            .unit_vector(),
        };
        let expected_ray_out = Ray {
            origin: Point3 {
                x: 2.220446049250313e-16, // Ideally `0.`
                y: 2.220446049250313e-16, // Ideally `0.`
                z: 0.,
            },
            direction: Vec3 {
                x: 0.8660254037844388,  // Ideally `3f64.sqrt() / 2.`
                y: 0.49999999999999967, // Ideally `0.5`
                z: 0.,
            }
            .unit_vector(),
        };
        let (_attenuation, ray_out) = glass.scatter(&ray_in, &hit);
        assert_eq!(expected_ray_out, ray_out);
    }
}
