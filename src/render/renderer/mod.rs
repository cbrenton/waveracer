mod dummy_renderer;
mod monte_carlo_renderer;

pub use dummy_renderer::DummyRenderer;
pub use monte_carlo_renderer::MonteCarloRenderer;

use mockall::automock;

use crate::{
    math::{Color, Ray},
    render::Hittable,
};

#[automock]
pub trait Renderer {
    fn ray_color(&self, ray: &Ray, world: &[Hittable], depth: i32) -> Color;
}
