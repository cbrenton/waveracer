use rt2::render::DummyRenderer;
use rt2::render::MultiFilePngWriter;
use rt2::render::VideoCamera;
use rt2::scene::sample_scene;

fn main() {
    let scene = sample_scene();

    //defocus_angle: 1.0,
    //focus_distance: 1.4,

    let renderer = DummyRenderer {};

    let mut camera = VideoCamera::new(90.0, renderer);

    let image_writer = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");

    while camera.is_rolling() {
        println!("{}/{} frames", camera.cur_frame, camera.total_frames);
        let frame = camera.capture_frame(&scene.world);
        image_writer.write(frame);
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
