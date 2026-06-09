use crate::{
    math::{Color, Ray},
    render::{Hittable, Renderer},
};

pub struct DummyRenderer {}

impl Renderer for DummyRenderer {
    fn ray_color(&self, _ray: &Ray, _world: &[Hittable], _depth: i32) -> Color {
        Color::default()
    }
}
