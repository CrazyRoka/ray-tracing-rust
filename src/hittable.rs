use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(point: Point3, outward_normal: Vec3, t: f64, ray: &Ray) -> Self {
        let front_face = ray.get_direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal.negative()
        };
        Self { point, normal, t }
    }

    pub fn get_point(&self) -> Point3 {
        self.point
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}