use glam::DVec3;
use kdam::tqdm;

use crate::{
    math::{ALMOST_ZERO, Color, DInterval, Ray},
    render::{HitRecord, Renderer, SomeHittable},
};

#[derive(Debug)]
pub struct MonteCarloRenderer {
    pub max_depth: i32,
}

impl Renderer for MonteCarloRenderer {
    fn ray_color(&self, ray: &Ray, world: &[SomeHittable], depth: i32) -> Color {
        if depth >= self.max_depth {
            return DVec3::ZERO;
        }

        let ray_t = DInterval::new(ALMOST_ZERO, f64::INFINITY);
        let mut closest_so_far = ray_t.max;
        let mut result: Option<HitRecord> = None;

        for object in world {
            if let Some(rec) = object.hit(ray, DInterval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        if let Some(rec) = result {
            let color_from_emission = rec.mat.emitted(rec.u, rec.v, rec.point);
            if let Some(scatter) = rec.mat.scatter(ray, &rec) {
                let color_from_scatter =
                    scatter.attenuation * self.ray_color(&scatter.scattered, world, depth + 1);
                color_from_emission + color_from_scatter
            } else {
                color_from_emission
            }
        } else {
            // TODO: fix
            // camera_render_config.background
            Color::splat(0.0)
        }
    }
}
