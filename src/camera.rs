use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Point3::empty();
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin
            .subtract(&horizontal.divide_constant(2.0))
            .subtract(&vertical.divide_constant(2.0))
            .subtract(&Vec3::new(0.0, 0.0, FOCAL_LENGTH));

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
