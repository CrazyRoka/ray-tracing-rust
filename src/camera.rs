use crate::{
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3, Vec3},
};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, vfow: f64) -> Self {
        let theta = degrees_to_radians(vfow);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = ASPECT_RATIO * viewport_height;

        let w = look_from.subtract(&look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u.multiply_constant(viewport_width);
        let vertical = v.multiply_constant(viewport_height);
        let lower_left_corner = origin
            .subtract(&horizontal.divide_constant(2.0))
            .subtract(&vertical.divide_constant(2.0))
            .subtract(&w);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner
                .add(&self.horizontal.multiply_constant(u))
                .add(&self.vertical.multiply_constant(v))
                .subtract(&self.origin),
        )
    }
}
