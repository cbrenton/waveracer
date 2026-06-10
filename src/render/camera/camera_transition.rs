use glam::DVec3;

use crate::{math::TransformFunc, render::CameraState};

#[derive(Clone)]
pub struct CameraTransition {
    pos_xform: Box<dyn TransformFunc>,
    look_at_xform: Box<dyn TransformFunc>,
    up_xform: Box<dyn TransformFunc>,
    frames: usize,
    cur_frame: usize,
}

impl CameraTransition {
    pub fn new(
        pos_trans: impl TransformFunc + 'static,
        look_at_trans: impl TransformFunc + 'static,
        up_trans: impl TransformFunc + 'static,
        frames: usize,
    ) -> Self {
        Self {
            pos_xform: Box::new(pos_trans),
            look_at_xform: Box::new(look_at_trans),
            up_xform: Box::new(up_trans),
            frames,
            cur_frame: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.frames
    }
}

impl Iterator for CameraTransition {
    type Item = CameraState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_frame > self.frames {
            None
        } else {
            let t = self.cur_frame as f64 / self.frames as f64;
            let pos = self.pos_xform.at(t)?;
            let look_at = self.look_at_xform.at(t)?;
            let up = self.up_xform.at(t)?;
            self.cur_frame += 1;

            Some(CameraState { pos, look_at, up })
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::{Sequence, predicate::eq};

    use crate::math::{Lerp, MockTransformFunc, almost_eq};

    use super::*;

    #[test]
    fn test_next_advances_frame_until_passing_last_frame_then_does_not_advance_further() {
        let mut pos = MockTransformFunc::new();
        pos.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut look_at = MockTransformFunc::new();
        look_at.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut up = MockTransformFunc::new();
        up.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut trans = CameraTransition::new(pos, look_at, up, 3);

        assert_eq!(trans.cur_frame, 1);
        trans.next();
        assert_eq!(trans.cur_frame, 2);
        trans.next();
        assert_eq!(trans.cur_frame, 3);
        trans.next();
        assert_eq!(trans.cur_frame, 4);
        trans.next();
        assert_eq!(trans.cur_frame, 4);
    }

    #[test]
    fn test_next_returns_some_next_camera_state_until_reaching_last_frame() {
        let mut pos = MockTransformFunc::new();
        pos.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut look_at = MockTransformFunc::new();
        look_at.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut up = MockTransformFunc::new();
        up.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut trans = CameraTransition::new(pos, look_at, up, 3);

        assert!(trans.next().is_some());
        assert!(trans.next().is_some());
        assert!(trans.next().is_some());
        assert!(trans.next().is_none());
    }

    #[test]
    fn test_iterator_has_correct_length_and_ends_at_end() {
        let pos = Lerp::new(DVec3::ZERO, DVec3::ONE, false);
        let mut look_at = MockTransformFunc::new();
        look_at.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut up = MockTransformFunc::new();
        up.expect_at().returning(|_| Some(DVec3::ZERO));
        let mut t = CameraTransition::new(pos, look_at, up, 3);

        assert!(almost_eq(t.next().unwrap().pos, DVec3::splat(1.0 / 3.0)));
        assert!(almost_eq(t.next().unwrap().pos, DVec3::splat(2.0 / 3.0)));
        assert!(almost_eq(t.next().unwrap().pos, DVec3::ONE));
    }

    #[test]
    fn test_iterator_passes_t_values_starting_past_0_and_ending_at_1() {
        fn dummy_transform_func() -> MockTransformFunc {
            let mut f = MockTransformFunc::new();
            let mut seq = Sequence::new();
            f.expect_at()
                .with(eq(1.0 / 3.0))
                .times(1)
                .in_sequence(&mut seq)
                .returning(|_| Some(DVec3::ZERO));
            f.expect_at()
                .with(eq(2.0 / 3.0))
                .times(1)
                .in_sequence(&mut seq)
                .returning(|_| Some(DVec3::ZERO));
            f.expect_at()
                .with(eq(1.0))
                .times(1)
                .in_sequence(&mut seq)
                .returning(|_| Some(DVec3::ZERO));
            f
        }
        let pos = dummy_transform_func();
        let look_at = dummy_transform_func();
        let up = dummy_transform_func();
        let mut trans = CameraTransition::new(pos, look_at, up, 3);

        trans.next().unwrap();
        trans.next().unwrap();
        trans.next().unwrap();
    }
}
