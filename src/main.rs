use glam::DVec3;
use rt2::render::CameraState;
use rt2::render::Film;
use rt2::render::LerpTransition;
use rt2::render::MonteCarloRenderer;
use rt2::render::MultiFilePngWriter;
use rt2::render::VideoCamera;
use rt2::scene::sample_scene;

fn main() {
    let scene = sample_scene();

    let renderer = MonteCarloRenderer { max_depth: 10 };
    let film = Film {
        width: 1000,
        height: 600,
        samples_per_pixel: 10,
    };

    let mut camera = VideoCamera::new(90.0, renderer, film);
    let image_writer = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");

    let camera_pos = DVec3::new(0.0, 0.0, 3.0);
    let camera_look_at = DVec3::new(0.0, 0.0, -1.0);
    let camera_up = DVec3::new(0.0, 1.0, 0.0);
    let state = CameraState {
        pos: camera_pos,
        look_at: camera_look_at,
        up: camera_up,
    };

    camera.add_transition(LerpTransition::new(&state, &state, 2));

    for frame in camera.render_frames(&scene.world) {
        image_writer.write(frame);
    }
}
