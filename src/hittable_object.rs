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
}
