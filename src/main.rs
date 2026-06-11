use std::f64::consts::PI;

use glam::DVec3;
use kdam::{BarExt, tqdm};
use rt2::math::Lerp;
use rt2::math::Rotate;
use rt2::render::CameraState;
use rt2::render::CameraTransition;
use rt2::render::Film;
use rt2::render::MonteCarloRenderer;
use rt2::render::MultiFilePngWriter;
use rt2::render::VideoCamera;
use rt2::scene::sample_scene;

fn main() {
    // 100 for 1000x600 image
    let scale = 100;

    let renderer = MonteCarloRenderer { max_depth: 10 };
    let film = Film {
        width: 10 * scale,
        height: 6 * scale,
        samples_per_pixel: 10,
    };

    let start = CameraState {
        pos: DVec3::new(0.0, 0.0, 3.0),
        look_at: DVec3::new(0.0, 0.0, -1.0),
        up: DVec3::new(0.0, 1.0, 0.0),
    };
    let end = CameraState {
        pos: DVec3::new(0.0, 0.0, 0.5),
        look_at: DVec3::new(0.0, 0.0, -1.0),
        up: DVec3::new(0.0, 1.0, 0.0),
    };
    let look_at_hold = Lerp::hold(start.look_at);
    let up_hold = Lerp::hold(start.up);

    let mut camera = VideoCamera::new(&start, 90.0, renderer, film);

    let zoom_in_slerp = Lerp::end_early(start.pos, end.pos, 0.8, true);
    camera.add_transition(CameraTransition::new(
        zoom_in_slerp,
        look_at_hold.clone(),
        up_hold.clone(),
        100,
    ));

    let zoom_out_slerp = Lerp::new(end.pos, start.pos, true);
    camera.add_transition(CameraTransition::new(
        zoom_out_slerp,
        look_at_hold.clone(),
        up_hold.clone(),
        100,
    ));

    let rot = Rotate::end_early(start.pos, start.look_at, 2.0 * PI, 0.5, true);
    camera.add_transition(CameraTransition::new(rot, look_at_hold, up_hold, 100));

    let image_writer = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");

    let scene = sample_scene();

    let mut progress = tqdm!(total = camera.total_frames);
    progress.refresh().unwrap();

    let mut errs = vec![];

    for frame in camera.render_frames(&scene.world) {
        if let Err(val) = image_writer.write(&frame) {
            errs.push(format!("frame {}: {}", frame.frame_number, val));
        }
        progress.update(1).unwrap();
    }

    if !errs.is_empty() {
        dbg!(errs);
    }
}
