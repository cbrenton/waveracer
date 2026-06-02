use crate::math::Color;

pub struct FrameData {
    pub w: i32,
    pub h: i32,
    pub pixels: Vec<Color>,
    pub frame_number: i32,
    pub t: f64,
    // TODO: camera settings (for debugging)
}
