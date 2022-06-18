use rand::Rng;

use crate::{constants::PI, vec3::Vec3};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    random_in_range(0.0, 1.0)
}

pub fn random_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let distribution = rand::distributions::Uniform::new(min, max);

    rng.sample(distribution)
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        in_unit_sphere.negative()
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let vec3 = Vec3::new(
            random_in_range(-1.0, 1.0),
            random_in_range(-1.0, 1.0),
            random_in_range(-1.0, 1.0),
        );

        if vec3.length() < 1.0 {
            return vec3;
        }
    }
}
