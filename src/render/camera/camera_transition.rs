use glam::DVec3;

use crate::{math::TransformFunc, render::CameraState};

#[derive(Clone)]
pub struct CameraTransition {
    pos_trans: Box<dyn TransformFunc>,
    look_at_trans: Box<dyn TransformFunc>,
    up_trans: Box<dyn TransformFunc>,
    pub ticks: usize,
    cur_tick: usize,
}

impl CameraTransition {
    pub fn new(
        pos_trans: impl TransformFunc + 'static,
        look_at_trans: impl TransformFunc + 'static,
        up_trans: impl TransformFunc + 'static,
        ticks: usize,
    ) -> Self {
        Self {
            pos_trans: Box::new(pos_trans),
            look_at_trans: Box::new(look_at_trans),
            up_trans: Box::new(up_trans),
            ticks,
            cur_tick: 0,
        }
    }

    pub fn tick(&mut self) -> Option<CameraState> {
        match self.cur_tick {
            _max if self.cur_tick == self.ticks => None,
            _ => {
                let t = self.cur_tick as f64 / self.ticks as f64;
                let pos = self.pos_trans.at(t)?;
                let look_at = self.look_at_trans.at(t)?;
                let up = self.up_trans.at(t)?;
                self.cur_tick += 1;

                Some(CameraState { pos, look_at, up })
            }
        }
    }
}

impl Iterator for CameraTransition {
    type Item = CameraState;

    fn next(&mut self) -> Option<Self::Item> {
        self.tick()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::MockTransformFunc;

    use super::*;

    #[test]
    fn test_tick_advances_frame_until_reaching_last_tick() {
        let mut pos = MockTransformFunc::new();
        pos.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut look_at = MockTransformFunc::new();
        look_at.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut up = MockTransformFunc::new();
        up.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut trans = CameraTransition::new(pos, look_at, up, 3);

        assert_eq!(trans.cur_tick, 0);
        trans.tick();
        assert_eq!(trans.cur_tick, 1);
        trans.tick();
        assert_eq!(trans.cur_tick, 2);
        trans.tick();
        assert_eq!(trans.cur_tick, 3);
        trans.tick();
        assert_eq!(trans.cur_tick, 3);
    }

    #[test]
    fn test_tick_returns_some_next_camera_state_until_reaching_last_tick() {
        let mut pos = MockTransformFunc::new();
        pos.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut look_at = MockTransformFunc::new();
        look_at.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut up = MockTransformFunc::new();
        up.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut trans = CameraTransition::new(pos, look_at, up, 3);

        assert!(trans.tick().is_some());
        assert!(trans.tick().is_some());
        assert!(trans.tick().is_some());
        assert!(trans.tick().is_none());
    }
}
