use glam::DVec3;
use kdam::tqdm;
use mockall::automock;

use crate::{
    math::{ALMOST_ZERO, Color, DInterval, Ray},
    render::{HitRecord, Hittable},
};

#[automock]
pub trait Renderer {
    fn ray_color(&self, ray: &Ray, world: &[Hittable], depth: i32) -> Color;
    // TODO: change this
    fn samples_per_pixel(&self) -> i32;
}

#[derive(Debug)]
pub struct MonteCarloRenderer {
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

impl Renderer for MonteCarloRenderer {
    fn ray_color(&self, ray: &Ray, world: &[Hittable], depth: i32) -> Color {
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
            // Color::new(1.0, 0.0, 0.0)
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

    fn samples_per_pixel(&self) -> i32 {
        self.samples_per_pixel
    }
}
