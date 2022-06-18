use crate::{color::Color, ray::Ray, vec3::Vec3};

use super::{Material, ScatterResult};

pub struct Metal {
    color: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v.subtract(&n.multiply_constant(v.dot(&n) * 2.0))
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        hit_record: &crate::hittable::HitRecord,
    ) -> Option<super::ScatterResult> {
        let reflected = Self::reflect(&ray.get_direction(), &hit_record.get_normal());
        let ray = Ray::new(hit_record.get_point(), reflected);
        let color = self.color;

        if ray.get_direction().dot(&hit_record.get_normal()) > 0.0 {
            Some(ScatterResult::new(color, ray))
        } else {
            None
        }
    }
}
