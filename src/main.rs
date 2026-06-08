use std::f64::consts::PI;

use glam::DVec3;
use kdam::{BarExt, tqdm};
use rt2::math::{Lerp, Rotate, Slerp};
use rt2::render::{
    CameraState, CameraTransition, MonteCarloRenderer, MultiFilePngWriter, VideoCamera,
};
use rt2::scene::sample_scene;

fn main() {
    let scene = sample_scene();

    let renderer = MonteCarloRenderer {
        samples_per_pixel: 100,
        max_depth: 10,
    };

    // 100 for 1000x600 image
    let scale = 20;

    let width = 10 * scale;
    let height = 6 * scale;

    let mut camera = VideoCamera::new(90.0, renderer, width, height);

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

    // TODO: maybe improve this interface so that we're only passing end camera state
    let look_at_hold = Lerp::hold(start.look_at);
    let up_hold = Lerp::hold(start.up);

    let zoom_in_slerp = Slerp::end_early(start.pos, end.pos, 0.8);
    camera.add_transition(CameraTransition::new(
        zoom_in_slerp,
        look_at_hold.clone(),
        up_hold.clone(),
        100,
    ));

    let zoom_out_slerp = Slerp::new(end.pos, start.pos);
    camera.add_transition(CameraTransition::new(
        zoom_out_slerp,
        look_at_hold.clone(),
        up_hold.clone(),
        100,
    ));

    let rot = Rotate::end_early(start.pos, start.look_at, 2.0 * PI, 0.5);
    camera.add_transition(CameraTransition::new(rot, look_at_hold, up_hold, 100));

    let image_writer = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");

    let mut progress = tqdm!(total = camera.total_frames);
    progress.refresh().unwrap();
    for frame in camera.render_frames(&scene.world) {
        image_writer.write(frame);
        progress.update(1).unwrap();
    }
}
