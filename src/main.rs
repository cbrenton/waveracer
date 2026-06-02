use rt2::math::Color;
use rt2::render::{FrameData, MultiFilePngWriter};
use rt2::scene::sample_scene;

fn main() {
    let scene = sample_scene();

    let writer = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");
    writer.write(FrameData {
        w: 1,
        h: 1,
        pixels: vec![Color::default()],
        frame_number: 0,
        t: 0.0,
    });
}
