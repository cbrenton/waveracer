use derive_more::{Add, Div, Mul, Sub};

use glam::DVec3;

#[derive(Default, Copy, Clone, Debug, Add, Sub, Mul, Div)]
// TODO: probably rename this
pub struct CameraState {
    pub pos: DVec3,
    pub look_at: DVec3,
    pub up: DVec3,
}
