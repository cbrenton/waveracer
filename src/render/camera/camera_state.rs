use derive_more::{Add, Div, Mul, Sub};

use glam::DVec3;

#[derive(Copy, Clone, Debug, Add, Sub, Mul, Div)]
pub struct CameraState {
    pub pos: DVec3,
    pub look_at: DVec3,
    pub up: DVec3,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            pos: DVec3::ZERO,
            look_at: DVec3::new(0.0, 0.0, -1.0),
            up: DVec3::new(0.0, 1.0, 0.0),
        }
    }
}
