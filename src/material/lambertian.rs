use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::random_unit_vector};

use super::{Material, ScatterResult};

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let scatter_direction = hit_record.get_normal().add(&random_unit_vector());

        let direction = if scatter_direction.near_zero() {
            hit_record.get_normal()
        } else {
            scatter_direction
        };

        let ray = Ray::new(hit_record.get_point(), direction);
        let color = self.color;

        Some(ScatterResult::new(color, ray))
    }
}
