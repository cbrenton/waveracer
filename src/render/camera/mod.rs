mod frame_data;
mod render_config;

pub use frame_data::FrameData;
pub use render_config::RenderConfig;

use glam::DVec3;
use image::{Rgb, RgbImage};
use kdam::tqdm;
use std::{
    fs,
    ops::{Index, IndexMut},
};

use crate::{math::Color, render::Hittable};

pub struct DummyRenderer {}

impl DummyRenderer {
    fn render(&self, _world: &[Hittable]) -> Vec<Color> {
        vec![Color::default()]
    }
}
