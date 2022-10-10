use crate::geometry::{Point3, Ray, UnitVec3};

#[derive(Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub surface_normal: UnitVec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
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
                Some(HitRecord { t, surface_normal })
            }
        }
    }
}

pub struct HittableList {
    pub members: Vec<Box<dyn Hittable>>,
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut maybe_nearest: Option<HitRecord> = None;
        for hittable in self.members.iter() {
            if let Some(hit) = hittable.hit(ray) {
                if let Some(nearest) = &maybe_nearest {
                    if hit.t < nearest.t {
                        maybe_nearest = Some(hit);
                    }
                } else {
                    maybe_nearest = Some(hit);
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

    #[test]
    fn sphere_test1() {
        let sphere = Sphere {
            center: Point3 {
                x: 0.,
                y: 0.,
                z: -3.,
            },
            radius: 1.,
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
        assert_eq!(Some(expected_hit), sphere.hit(&ray));
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
        assert_eq!(Some(expected_hit), sphere.hit(&ray));
    }
}
