extern crate dyn_clone;

use dyn_clone::DynClone;

use crate::color::Attenuation;
use crate::geometry::{random_unit_vector, reflect_vector, Point3, Ray, UnitVec3};

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
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> (Attenuation, Ray) {
        let direction = reflect_vector(&ray_in.direction, &hit.surface_normal);
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

/// The type for glasses, i.e., materials that perform refraction.
/// The parameter `eta` is the refractive index and should >= 1.
#[derive(Clone)]
pub struct Glass {
    pub eta: f64,
}
impl Material for Glass {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> (Attenuation, Ray) {
        let normal_raw = hit.surface_normal.inject();
        let direction_in = ray_in.direction.inject();
        let inprod = normal_raw.inner_product(&direction_in);

        // TODO: generalize the refractive index of external spaces.
        let (normal, eta_in, eta_out) = {
            if inprod < 0. {
                // If `ray_in` is coming into the object from the outside:
                (normal_raw, 1., self.eta)
            } else {
                // If `ray_in` is going out of the object from the inside:
                (normal_raw.scale(-1.), self.eta, 1.)
            }
        };

        // v := d - (n^T d) n
        let vp_in = direction_in.subtract(&normal.scale(inprod));

        // v' := (eta / eta') v
        let vp_out = vp_in.scale(eta_in / eta_out);

        // d' = v' - sqrt(1 - |v'|^2) n
        let direction_out = vp_out
            .subtract(&normal.scale((1. - vp_out.length_squared()).sqrt()))
            .unit_vector();

        let ray = Ray {
            origin: ray_in.at(hit.t),
            direction: direction_out,
        };
        let attenuation = Attenuation {
            r: 1.,
            g: 1.,
            b: 1.,
        }; // TODO: generalize this
        (attenuation, ray)
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
        if discriminant_quarter <= 0. {
            None
        } else {
            let t = -b_half - discriminant_quarter.sqrt();
            if t < 0. {
                None
            } else {
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
    fn glass_scatter_test1() {
        let glass = Glass { eta: 1.0 };
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
        let glass = Glass { eta: 3f64.sqrt() };
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
}
