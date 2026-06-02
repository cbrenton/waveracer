use std::ops::{Add, Div, Mul, Sub};

use glam::DVec3;

#[derive(Clone, Debug)]
pub struct CameraState {
    pub pos: DVec3,
    pub look_at: DVec3,
    pub up: DVec3,
}

impl Sub for CameraState {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            pos: self.pos - other.pos,
            look_at: self.look_at - other.look_at,
            up: self.up - other.up,
        }
    }
}

impl Add for CameraState {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            pos: self.pos + other.pos,
            look_at: self.look_at + other.look_at,
            up: self.up + other.up,
        }
    }
}

impl Div<f64> for CameraState {
    type Output = Self;

    // Required method
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            pos: self.pos / rhs,
            look_at: self.look_at / rhs,
            up: self.up / rhs,
        }
    }
}

impl Mul<f64> for CameraState {
    type Output = Self;

    // Required method
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            pos: self.pos * rhs,
            look_at: self.look_at * rhs,
            up: self.up * rhs,
        }
    }
}
