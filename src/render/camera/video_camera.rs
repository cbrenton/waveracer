use glam::DVec3;

use crate::{
    math::Color,
    render::{DummyRenderer, FrameData, Hittable},
};

pub struct VideoCamera {
    vfov: f64,
    renderer: DummyRenderer,
    pos: DVec3,
    look_at: DVec3,
    up: DVec3,
    pub cur_frame: i32,
    pub total_frames: i32,
}

impl VideoCamera {
    pub fn new(vfov: f64, renderer: DummyRenderer) -> Self {
        let camera_look_from = DVec3::ZERO;
        let camera_look_at = DVec3::new(0.0, 0.0, -1.0);
        let camera_up = DVec3::new(0.0, 1.0, 0.0);
        Self {
            vfov,
            renderer,
            pos: camera_look_from,
            look_at: camera_look_at,
            up: camera_up,
            cur_frame: 0,
            total_frames: 10,
        }
    }

    pub fn capture_frame(&mut self, world: &[Hittable]) -> FrameData {
        self.cur_frame += 1;
        let pixels = self.renderer.render(world);
        FrameData {
            w: 1,
            h: 1,
            pixels,
            frame_number: self.cur_frame - 1,
            t: 0.0,
        }
    }

    /*
    // TODO
    fn roll(&self, renderer: impl Renderer) {
        self.renderer.prebake();
    }
    */

    pub fn is_rolling(&self) -> bool {
        self.cur_frame <= self.total_frames
    }
}
