mod basic_camera;
mod render_config;

pub use basic_camera::*;
pub use render_config::RenderConfig;

use glam::DVec3;
use image::{Rgb, RgbImage};
use kdam::tqdm;
use std::{
    fs,
    ops::{Index, IndexMut},
};

use crate::{math::Color, render::Hittable};

pub trait Camera {
    fn render(&self, render_config: &RenderConfig, world: &[Hittable]) -> Vec<Color>;
}
