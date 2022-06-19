use crate::{
    color::Color,
    ray::Ray,
    utils::{random_in_unit_sphere, reflect},
    vec3::Vec3,
};

use super::{Material, ScatterResult};

pub struct Metal {
    color: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            color,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        hit_record: &crate::hittable::HitRecord,
    ) -> Option<super::ScatterResult> {
        let reflected = reflect(&ray.get_direction(), &hit_record.get_normal());
        let ray = Ray::new(
            hit_record.get_point(),
            reflected.add(&random_in_unit_sphere().multiply_constant(self.fuzz)),
        );
        let color = self.color;

        if ray.get_direction().dot(&hit_record.get_normal()) > 0.0 {
            Some(ScatterResult::new(color, ray))
        } else {
            None
        }
    }
}
