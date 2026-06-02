use crate::math::Color;

pub struct RenderConfig {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub background: Color,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 450,
            samples_per_pixel: 100,
            max_depth: 10,
            background: Color::ZERO,
        }
    }
}
