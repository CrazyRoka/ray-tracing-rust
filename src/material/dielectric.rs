use crate::{
    color::WHITE,
    hittable::HitRecord,
    ray::Ray,
    utils::{random_double, reflect, refract},
};

use super::{Material, ScatterResult};

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio = if hit_record.get_front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.get_direction().unit_vector();
        let cos = hit_record
            .get_normal()
            .dot(&unit_direction.negative())
            .min(1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let direction = if refraction_ratio * sin > 1.0
            || reflectance(cos, refraction_ratio) > random_double()
        {
            reflect(&unit_direction, &hit_record.get_normal())
        } else {
            refract(&unit_direction, &hit_record.get_normal(), refraction_ratio)
        };
        let ray = Ray::new(hit_record.get_point(), direction);
        let color = WHITE;

        Some(ScatterResult::new(color, ray))
    }
}

fn reflectance(cos: f64, refraction_ratio: f64) -> f64 {
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}
