use crate::{hittable::HitRecord, ray::Ray};
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use scatter_result::ScatterResult;

mod dielectric;
mod lambertian;
mod metal;
mod scatter_result;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}
