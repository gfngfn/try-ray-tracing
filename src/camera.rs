use crate::geometry::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
}
impl Camera {
    pub fn new(aspect_ratio: f64, origin: Point3, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width: f64 = viewport_height * aspect_ratio;
        Self {
            origin,
            viewport_width,
            viewport_height,
            focal_length,
        }
    }

    pub fn horizontal(&self) -> Vec3 {
        Vec3 {
            x: self.viewport_width,
            y: 0.,
            z: 0.,
        }
    }

    pub fn vertical(&self) -> Vec3 {
        Vec3 {
            x: 0.,
            y: self.viewport_height,
            z: 0.,
        }
    }

    pub fn forward(&self) -> Vec3 {
        Vec3 {
            x: 0.,
            y: 0.,
            z: -self.focal_length,
        }
    }

    pub fn lower_left_corner(&self) -> Point3 {
        self.origin
            .add(&self.horizontal().scale(-0.5))
            .add(&self.vertical().scale(-0.5))
            .add(&self.forward())
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = self.origin.clone();
        let direction = self
            .lower_left_corner()
            .add(&self.horizontal().scale(u))
            .add(&self.vertical().scale(v))
            .subtract(&origin)
            .unit_vector();
        Ray { origin, direction }
    }
}
