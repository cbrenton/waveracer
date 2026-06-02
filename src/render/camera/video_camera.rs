use std::{collections::LinkedList, mem::take};

use crate::render::{CameraState, DummyRenderer, FrameData, Hittable, LerpTransition};

pub struct VideoCamera {
    vfov: f64,
    renderer: DummyRenderer,
    transitions: LinkedList<LerpTransition>,
    pub cur_frame: i32,
    pub total_frames: i32,
    // TODO: this is gnarly. I should ask somebody if there's a better way
    trans_iterator: Option<Box<dyn Iterator<Item = CameraState>>>,
}

impl VideoCamera {
    pub fn new(vfov: f64, renderer: DummyRenderer) -> Self {
        Self {
            vfov,
            renderer,
            transitions: LinkedList::new(),
            cur_frame: 0,
            total_frames: 20,
            trans_iterator: None,
        }
    }

    pub fn add_transition(&mut self, transition: LerpTransition) {
        self.transitions.push_back(transition);
    }

    pub fn capture_frame(&mut self, world: &[Hittable]) -> Option<FrameData> {
        if !self.is_rolling() {
            // TODO: improve this
            panic!("camera isn't rolling");
        }
        let pixels = self.renderer.render(world);
        let state = self.trans_iterator.as_mut().and_then(Iterator::next);
        // TODO: so gross. make this better
        if state.is_none() {
            self.trans_iterator = None;
            return None;
        }
        dbg!(state);
        let result = FrameData {
            w: 1,
            h: 1,
            pixels,
            frame_number: self.cur_frame,
            t: 0.0,
        };
        self.cur_frame += 1;
        Some(result)
    }

    pub fn roll(&mut self) {
        // self.renderer.prebake();

        // use std::mem::take to take ownership of the transitions field of a mutable struct
        let transitions = take(&mut self.transitions);
        // sigh...OPTION of FAT POINTER of something that IMPLEMENTS iterator OVER camerastate
        self.trans_iterator = Some(Box::new(transitions.into_iter().flatten()));
    }

    pub fn is_rolling(&self) -> bool {
        // TODO: I don't think this is correct
        self.trans_iterator.is_some() // self.cur_frame <= self.total_frames
    }
}
