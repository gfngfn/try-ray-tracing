use crate::geometry::{Point3, Ray, UnitVec3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn new(
        origin: Point3,
        look_in: UnitVec3,
        view_up: Vec3,
        vertical_fov_radian: f64,
        aspect_ratio: f64,
    ) -> Self {
        let viewport_height: f64 = (vertical_fov_radian / 2.).tan();
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let w = look_in.inject().scale(-1.).unit_vector();
        let u = view_up.cross_product(&w.inject()).unit_vector();
        let v = w.inject().cross_product(&u.inject()).unit_vector();

        let horizontal = u.inject().scale(viewport_width);
        let vertical = v.inject().scale(viewport_height);

        let lower_left_corner = origin
            .add(&horizontal.scale(-0.5))
            .add(&vertical.scale(-0.5))
            .add(&look_in.inject());

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = self.origin.clone();
        let direction = self
            .lower_left_corner
            .add(&self.horizontal.scale(u))
            .add(&self.vertical.scale(v))
            .subtract(&origin)
            .unit_vector();
        Ray { origin, direction }
    }
}
