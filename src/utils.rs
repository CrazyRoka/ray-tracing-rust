use rand::Rng;

use crate::constants::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    let distribution = rand::distributions::Uniform::new(0.0, 1.0);

    rng.sample(distribution)
}
