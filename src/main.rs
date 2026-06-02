use std::path::PathBuf;

use glam::DVec3;

use rt2::math::Color;
use rt2::render::BasicCameraConfig;
use rt2::render::PngImageWriter;
use rt2::render::RenderConfig;
use rt2::render::{BasicCamera, Camera};
use rt2::scene::sample_scene;

fn main() {
    let scene = sample_scene();

    let camera_config = BasicCameraConfig {
        look_from: DVec3::ZERO,
        look_at: DVec3::new(0.0, 0.0, -1.0),
        up: DVec3::new(0.0, 1.0, 0.0),
        vfov: 90.0,
        defocus_angle: 1.0,
        focus_distance: 1.4,
    };

    let render_config = RenderConfig {
        width: 1080,
        height: 720,
        samples_per_pixel: 100,
        max_depth: 10,
        background: Color::splat(0.5),
    };

    let frame_pixels = BasicCamera::new(camera_config).render(&render_config, &scene.world);
    PngImageWriter::new(&render_config).write(frame_pixels, PathBuf::from("./output/out.png"));
}
