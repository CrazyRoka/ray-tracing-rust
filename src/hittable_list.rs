use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub const fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects.iter().fold(None, |res, cur| {
            let closest = if let Some(hitted_res) = &res {
                hitted_res.get_t()
            } else {
                t_max
            };

            if let Some(hitted_res) = cur.hit(ray, t_min, closest) {
                Some(hitted_res)
            } else {
                res
            }
        })
    }
}
