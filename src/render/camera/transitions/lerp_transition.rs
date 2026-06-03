use crate::render::CameraState;

#[derive(Debug)]
pub struct LerpTransition {
    start_state: CameraState,
    ticks: i32,
    cur_tick: i32,
    delta: CameraState,
}

impl LerpTransition {
    pub fn new(start: &CameraState, end: &CameraState, ticks: i32) -> Self {
        Self {
            start_state: *start,
            ticks,
            cur_tick: 0,
            // TODO: this currently creates one too many frames. I'm too foggy to fix it yet
            delta: (*end - *start) / ticks as f64,
        }
    }

    pub fn hold(hold_state: &CameraState, ticks: i32) -> Self {
        Self::new(hold_state, hold_state, ticks)
    }

    fn tick(&self, tick: i32) -> CameraState {
        self.start_state + self.delta * tick as f64
    }
}

impl Iterator for LerpTransition {
    type Item = CameraState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_tick == self.ticks {
            None
        } else {
            self.cur_tick += 1;
            Some(self.tick(self.cur_tick))
        }
    }
}
