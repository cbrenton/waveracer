use crate::render::CameraState;

#[derive(Debug, Clone)]
pub struct LerpTransition {
    start_state: CameraState,
    pub ticks: usize,
    cur_tick: usize,
    delta: CameraState,
    pub name: String,
}

impl LerpTransition {
    pub fn new(start: &CameraState, end: &CameraState, ticks: usize) -> Self {
        Self {
            start_state: *start,
            ticks,
            cur_tick: 0,
            // TODO: this currently creates one too many frames. I'm too foggy to fix it yet
            delta: (*end - *start) / ticks as f64,
            name: format!("{} frame lerp", ticks),
        }
    }

    pub fn hold(hold_state: &CameraState, ticks: usize) -> Self {
        Self::new(hold_state, hold_state, ticks)
    }

    fn tick(&self, tick: usize) -> CameraState {
        self.start_state + self.delta * tick as f64
    }

    pub fn ticks_left(&self) -> usize {
        self.ticks - self.cur_tick
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
