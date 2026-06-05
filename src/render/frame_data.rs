use crate::math::Color;

pub struct FrameData {
    pub w: usize,
    pub h: usize,
    pub pixels: Vec<Color>,
    pub frame_number: usize,
    pub t: f64,
    // TODO: camera settings (for debugging)
}
