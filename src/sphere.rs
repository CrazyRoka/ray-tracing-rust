use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.get_origin().subtract(&self.center);
        let a = ray.get_direction().square();
        let half_b = oc.dot(&ray.get_direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t1 = (-half_b - discriminant.sqrt()) / a;
            let t2 = (-half_b + discriminant.sqrt()) / a;

            let range = t_min..=t_max;
            let solution = if range.contains(&t1) {
                Some(t1)
            } else if range.contains(&t2) {
                Some(t2)
            } else {
                None
            };

            solution.map(|t| {
                let point = ray.at(t);
                let outward_normal = point.subtract(&self.center).divide_constant(self.radius);
                HitRecord::new(point, outward_normal, t, ray, Rc::clone(&self.material))
            })
        }
    }
}
