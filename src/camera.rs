use crate::{
    ray::Ray,
    utils::{degrees_to_radians, random_in_unit_disk},
    vec3::{Point3, Vec3},
};

pub const ASPECT_RATIO: f64 = 3.0 / 2.0;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfow: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfow);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = ASPECT_RATIO * viewport_height;

        let w = look_from.subtract(&look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u
            .multiply_constant(viewport_width)
            .multiply_constant(focus_dist);
        let vertical = v
            .multiply_constant(viewport_height)
            .multiply_constant(focus_dist);
        let lower_left_corner = origin
            .subtract(&horizontal.divide_constant(2.0))
            .subtract(&vertical.divide_constant(2.0))
            .subtract(&w.multiply_constant(focus_dist));
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            w,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = random_in_unit_disk().multiply_constant(self.lens_radius);
        let offset = self
            .u
            .multiply_constant(rd.get_x())
            .add(&self.v.multiply_constant(rd.get_y()));

        Ray::new(
            self.origin.add(&offset),
            self.lower_left_corner
                .add(&self.horizontal.multiply_constant(u))
                .add(&self.vertical.multiply_constant(v))
                .subtract(&self.origin)
                .subtract(&offset),
        )
    }
}
