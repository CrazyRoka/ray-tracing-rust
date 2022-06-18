use crate::{color::Color, ray::Ray};

pub struct ScatterResult {
    color: Color,
    ray: Ray,
}

impl ScatterResult {
    pub fn new(color: Color, ray: Ray) -> Self {
        Self { color, ray }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_ray(&self) -> &Ray {
        &self.ray
    }
}
