mod monte_carlo_renderer;

pub use monte_carlo_renderer::MonteCarloRenderer;

use mockall::automock;

use crate::{
    math::{Color, Ray},
    render::SomeHittable,
};

#[automock]
pub trait Renderer {
    fn ray_color(&self, ray: &Ray, world: &[SomeHittable], depth: i32) -> Color;
}
