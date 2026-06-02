use crate::{math::Color, render::Hittable};

pub struct DummyRenderer {}

impl DummyRenderer {
    pub fn render(&self, _world: &[Hittable]) -> Vec<Color> {
        vec![Color::default()]
    }
}
