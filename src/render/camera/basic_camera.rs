use crate::math::random::{random_double, random_in_unit_disk};
use crate::math::{ALMOST_ZERO, Color, DInterval, IInterval, Ray, linear_to_gamma};
use crate::render::camera::{Camera, RenderConfig};
use crate::render::{HitRecord, Hittable};

use glam::DVec3;
use image::{Rgb, RgbImage};
use kdam::{BarExt, tqdm};
use std::fs;
use std::ops::{Index, IndexMut};

pub struct BasicCameraConfig {
    pub vfov: f64,
    pub look_from: DVec3,
    pub look_at: DVec3,
    pub up: DVec3,
    pub defocus_angle: f64,
    pub focus_distance: f64, // distance from camera look_from point to plane of perfect focus
}

impl Default for BasicCameraConfig {
    fn default() -> Self {
        Self {
            vfov: 90.0,
            look_from: DVec3::ZERO,
            look_at: DVec3::new(0.0, 0.0, -1.0),
            up: DVec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
        }
    }
}

#[derive(Default)]
pub struct BasicCamera {
    config: BasicCameraConfig,
}

impl Camera for BasicCamera {
    fn render(&self, render_config: &RenderConfig, world: &[Hittable]) -> Vec<Color> {
        let camera_render_config = BasicCameraRenderConfig::new(self, render_config);

        let mut result: Vec<Color> = vec![];

        let mut bar = tqdm!(total = render_config.width * render_config.height);
        for y in 0..render_config.height {
            for x in 0..render_config.width {
                let mut pixel_color = Color::ZERO;

                // cast SAMPLES_PER_PIXEL random-ish rays and then divide total color by
                // SAMPLES_PER_PIXEL for simple antialiasing
                for _ in 0..render_config.samples_per_pixel {
                    let ray = self.get_ray(x, y, &camera_render_config);

                    pixel_color += self.ray_color(&ray, world, 0, &camera_render_config);
                }
                // NOTE: I chose to not include samples in my progress bar for the sake of having a
                // semi-readable number
                bar.update(1).unwrap();
                result.push(pixel_color / render_config.samples_per_pixel as f64);
            }
        }

        result
    }
}

impl BasicCamera {
    pub fn new(config: BasicCameraConfig) -> Self {
        Self { config }
    }

    /// Spawns a single ray at pixel [x, y] using the CameraRenderConfig's settings. For this
    /// camera the ray is offset by the defocus amount to add depth of field
    fn get_ray(&self, x: usize, y: usize, camera_render_config: &BasicCameraRenderConfig) -> Ray {
        let offset = self.sample_square();

        let pixel_center = camera_render_config.pixel00_loc
            + ((x as f64 + offset.x) * camera_render_config.pixel_delta_u)
            + ((y as f64 + offset.y) * camera_render_config.pixel_delta_v);

        let ray_origin = if self.config.defocus_angle <= 0.0 {
            camera_render_config.camera_center
        } else {
            self.defocus_disk_sample(camera_render_config)
        };
        let ray_direction = pixel_center - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self, camera_render_config: &BasicCameraRenderConfig) -> DVec3 {
        let p = random_in_unit_disk();
        camera_render_config.camera_center
            + (p.x * camera_render_config.defocus_disk_u)
            + (p.y * camera_render_config.defocus_disk_v)
    }

    fn sample_square(&self) -> DVec3 {
        DVec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(
        &self,
        ray: &Ray,
        world: &[Hittable],
        depth: i32,
        camera_render_config: &BasicCameraRenderConfig,
    ) -> Color {
        if depth >= camera_render_config.max_depth {
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
                let color_from_scatter = scatter.attenuation
                    * self.ray_color(&scatter.scattered, world, depth + 1, camera_render_config);
                color_from_emission + color_from_scatter
            } else {
                color_from_emission
            }
        } else {
            camera_render_config.background
        }
    }
}

// TODO: refine this
/// Camera-specific config that is dependent on render configuration. When a render is requested, a
/// BasicCameraRenderConfig will be created on the fly, to allow for multiple renders using the
/// same camera.
pub struct BasicCameraRenderConfig {
    camera_center: DVec3,
    pixel00_loc: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
    // TODO: delete?
    width: usize,
    height: usize,
    samples_per_pixel: i32,
    max_depth: i32,
    background: Color,
}

impl BasicCameraRenderConfig {
    pub fn new(camera: &BasicCamera, render_config: &RenderConfig) -> Self {
        let camera_center = camera.config.look_from;

        let theta = camera.config.vfov.to_radians();
        let h = (theta / 2.0).tan();
        // TODO: is this still necessary?
        let viewport_height = 2.0 * h * camera.config.focus_distance;
        // recalculate aspect ratio because image_h might not be what we intended
        let viewport_width =
            viewport_height * (render_config.width as f64 / render_config.height as f64);

        // calculate u,v,w unit basis vectors for camera
        let w = (camera.config.look_from - camera.config.look_at).normalize();
        let u = camera.config.up.cross(w).normalize();
        let v = w.cross(u);

        // calculate the vectors along the horizontal and vertical edges of the viewport
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / render_config.width as f64;
        let pixel_delta_v = viewport_v / render_config.height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left = camera_center
            - (camera.config.focus_distance * w)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            camera.config.focus_distance * (camera.config.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
            width: render_config.width,
            height: render_config.height,
            samples_per_pixel: render_config.samples_per_pixel,
            max_depth: render_config.max_depth,
            background: render_config.background,
        }
    }
}
