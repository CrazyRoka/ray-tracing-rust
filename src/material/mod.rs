use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::random_unit_vector};
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use scatter_result::ScatterResult;

mod lambertian;
mod metal;
mod scatter_result;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}
