use crate::vec3::Vec3;
use std::fmt::Display;

pub type Color = Vec3;

pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const BLUE: Color = Color::new(0.5, 0.7, 1.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

pub struct MultipleSamplesColor {
    color_sum: Color,
    samples_count: usize,
}

impl MultipleSamplesColor {
    pub fn new() -> Self {
        Self {
            color_sum: BLACK,
            samples_count: 0,
        }
    }

    pub fn add(&mut self, color: &Color) {
        self.color_sum = self.color_sum.add(color);
        self.samples_count += 1;
    }
}

fn color_to_i(color: f64) -> usize {
    let transformed = color.clamp(0.0, 0.999) * 256.0;
    transformed as usize
}

impl Display for MultipleSamplesColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scale = 1.0 / self.samples_count as f64;
        let r = (self.color_sum.get_x() * scale).sqrt();
        let g = (self.color_sum.get_y() * scale).sqrt();
        let b = (self.color_sum.get_z() * scale).sqrt();

        write!(f, "{} {} {}", color_to_i(r), color_to_i(g), color_to_i(b))
    }
}
