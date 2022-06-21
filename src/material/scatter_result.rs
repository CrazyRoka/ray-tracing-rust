use crate::{color::Color, ray::Ray};

pub struct ScatterResult {
    color: Color,
    ray: Ray,
}

impl ScatterResult {
    pub const fn new(color: Color, ray: Ray) -> Self {
        Self { color, ray }
    }

    pub const fn get_color(&self) -> Color {
        self.color
    }

    pub const fn get_ray(&self) -> &Ray {
        &self.ray
    }
}
