use glam::DVec3;
use kdam::{BarExt, tqdm};
use rt2::render::CameraState;
use rt2::render::LerpTransition;
use rt2::render::MonteCarloRenderer;
use rt2::render::MultiFilePngWriter;
use rt2::render::VideoCamera;
use rt2::scene::sample_scene;

fn main() {
    let scene = sample_scene();

    let renderer = MonteCarloRenderer {
        samples_per_pixel: 20,
        max_depth: 10,
    };

    let width = 1000;
    let height = 600;
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
    // TODO: improve this interface so that we're only passing end camera state
    camera.add_transition(LerpTransition::hold(&start, 10));
    camera.add_transition(LerpTransition::new(&start, &end, 100));
    camera.add_transition(LerpTransition::new(&end, &start, 100));

    camera.roll();

    let image_writer = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");

    let mut bar = tqdm!(
        total = camera.total_frames,
        position = 0,
        desc = "OVERALL",
        colour = "green"
    );
    bar.refresh().unwrap();
    while camera.is_rolling() {
        let result = camera.capture_frame(&scene.world);
        match result {
            Some(frame) => image_writer.write(frame),
            None => println!("done"),
        }
        bar.update(1).unwrap();
    }
}
