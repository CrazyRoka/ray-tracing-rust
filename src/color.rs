use crate::vec3::Vec3;
use std::fmt::Display;

pub type Color = Vec3;

pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const BLUE: Color = Color::new(0.5, 0.7, 1.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0);

fn color_to_i(color: f64) -> usize {
    let transformed = color * 255.999;
    transformed as usize
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            color_to_i(self.get_x()),
            color_to_i(self.get_y()),
            color_to_i(self.get_z())
        )
    }
}
