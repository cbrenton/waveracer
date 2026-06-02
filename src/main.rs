use glam::DVec3;
use rt2::render::CameraState;
use rt2::render::DummyRenderer;
use rt2::render::LerpTransition;
use rt2::render::MultiFilePngWriter;
use rt2::render::VideoCamera;
use rt2::scene::sample_scene;

fn main() {
    let scene = sample_scene();

    //defocus_angle: 1.0,
    //focus_distance: 1.4,

    let renderer = DummyRenderer {};

    let mut camera = VideoCamera::new(90.0, renderer);
    let start = CameraState {
        pos: DVec3::ZERO,
        look_at: DVec3::new(0.0, 0.0, -1.0),
        up: DVec3::new(0.0, 1.0, 0.0),
    };
    let end = CameraState {
        pos: DVec3::new(3.0, 0.0, 0.0),
        look_at: DVec3::new(0.0, 0.0, -1.0),
        up: DVec3::new(0.0, 1.0, 0.0),
    };
    // TODO: improve this interface so that we're only passing end camera state
    camera.add_transition(LerpTransition::new(start.clone(), start.clone(), 10));
    camera.add_transition(LerpTransition::new(start.clone(), end.clone(), 10));
    camera.add_transition(LerpTransition::new(end, start, 10));

    camera.roll();

    let image_writer = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");

    while camera.is_rolling() {
        println!("{}/{} frames", camera.cur_frame, camera.total_frames);
        let result = camera.capture_frame(&scene.world);
        match result {
            Some(frame) => image_writer.write(frame),
            None => println!("done"),
        }
    }

    /*
    let render_config = RenderConfig {
        width: 1080,
        height: 720,
        samples_per_pixel: 100,
        max_depth: 10,
        background: Color::splat(0.5),
    };

    let frame_pixels = BasicCamera::new(camera_config).render(&render_config, &scene.world);
    PngImageWriter::new(&render_config).write(frame_pixels, PathBuf::from("./output/out.png"));
    */
}
