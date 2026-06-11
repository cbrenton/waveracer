use kdam::{BarExt, tqdm};
use rt2::render::Film;
use rt2::render::MonteCarloRenderer;
use rt2::render::MultiFilePngWriter;
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

    let image_writer = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");

    let scene = sample_scene(renderer, film);

    let mut progress = tqdm!(total = scene.camera.total_frames);
    progress.refresh().unwrap();

    let mut errs = vec![];

    for frame in scene.camera.render_frames(&scene.world) {
        if let Err(val) = image_writer.write(&frame) {
            errs.push(format!("frame {}: {}", frame.frame_number, val));
        }
        progress.update(1).unwrap();
    }

    if !errs.is_empty() {
        dbg!(errs);
    }
}
