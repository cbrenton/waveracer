use std::{
    fs,
    ops::Mul,
    path::{Path, PathBuf},
};

use image::{Rgb, RgbImage};

use crate::{
    math::{Color, IInterval, linear_to_gamma},
    render::{FrameData, RenderConfig},
};

pub struct MultiFilePngWriter {
    output_dir: PathBuf,
    filename_template: String,
}

impl MultiFilePngWriter {
    pub fn new(output_dir: &str, filename_template: &str) -> Self {
        Self {
            output_dir: PathBuf::from(output_dir),
            filename_template: String::from(filename_template),
        }
    }

    pub fn write(&self, frame: FrameData) {
        let mut buf = RgbImage::new(frame.w as u32, frame.h as u32);
        for y in 0..frame.h {
            for x in 0..frame.w {
                let pixel = buf.get_pixel_mut(x as u32, y as u32);
                *pixel = self.convert_pixel(frame.pixels[(y * frame.w + x) as usize]);
            }
        }

        // create output dir
        let _ = fs::create_dir(self.output_dir.clone());

        // force png extension
        let mut output = self.output_dir.clone();
        let filename_str = self.output_filename(frame);
        output.push(filename_str);
        output.set_extension("png");

        // write image
        match buf.save(output.clone()) {
            Ok(_) => println!("Wrote file to {:?}", output),
            Err(err) => println!("Error while writing image: {}", err),
        }
    }

    fn convert_pixel(&self, pixel: Color) -> Rgb<u8> {
        // translate the [0.0, 1.0] component values to the byte range [0.0, 255.0]
        let r = Self::INTENSITY.scale(linear_to_gamma(pixel.x)) as u8;
        let g = Self::INTENSITY.scale(linear_to_gamma(pixel.y)) as u8;
        let b = Self::INTENSITY.scale(linear_to_gamma(pixel.z)) as u8;

        Rgb([r, g, b])
    }

    fn output_filename(&self, frame: FrameData) -> String {
        self.filename_template
            .replace("{{frame_number}}", frame.frame_number.to_string().as_str())
    }

    pub const INTENSITY: IInterval = IInterval { min: 0, max: 255 };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_frame(frame_number: usize) -> FrameData {
        FrameData {
            w: 1,
            h: 1,
            pixels: vec![Color::default()],
            frame_number,
            t: 0.0,
        }
    }

    #[test]
    fn test_write_appends_png_to_filename_if_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path().join("output");

        let w = MultiFilePngWriter::new(
            output_dir.to_str().expect("should never fail"),
            "frame_{{frame_number}}",
        );

        let frame = dummy_frame(0);
        w.write(frame);

        assert!(tmp.path().join("output/frame_0.png").is_file());
    }

    #[test]
    fn test_write_leaves_filename_alone_if_already_has_extension() {
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path().join("output");

        let w = MultiFilePngWriter::new(
            output_dir.to_str().expect("should never fail"),
            "frame_{{frame_number}}",
        );

        let frame = dummy_frame(0);
        w.write(frame);

        assert!(tmp.path().join("output/frame_0.png").is_file());
    }

    #[test]
    fn test_write_creates_output_dir_if_dne() {
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path().join("output");

        let w = MultiFilePngWriter::new(
            output_dir.to_str().expect("should never fail"),
            "frame_{{frame_number}}",
        );

        let frame = dummy_frame(0);
        w.write(frame);

        assert!(tmp.path().join("output/").is_dir());
    }

    #[test]
    fn test_write_creates_one_file_per_frame() {
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path().join("output");

        let w = MultiFilePngWriter::new(
            output_dir.to_str().expect("should never fail"),
            "frame_{{frame_number}}",
        );

        w.write(dummy_frame(0));
        w.write(dummy_frame(1));

        assert!(tmp.path().join("output/frame_0.png").is_file());
        assert!(tmp.path().join("output/frame_1.png").is_file());
    }

    #[test]
    fn test_output_filename_replaces_placeholder() {
        let w = MultiFilePngWriter::new("./output", "frame_{{frame_number}}");

        let frame = dummy_frame(0);
        assert_eq!(w.output_filename(frame), "frame_0");
        let frame1 = dummy_frame(1);
        assert_eq!(w.output_filename(frame1), "frame_1");
    }

    #[test]
    fn test_convert_pixel_black_returns_0_0_0() {
        let w = MultiFilePngWriter::new("output", "frame_{{frame_number}}");

        let pixel = Color::ZERO;
        let color = w.convert_pixel(pixel);
        assert_eq!(color, Rgb([0, 0, 0]));
    }

    #[test]
    fn test_convert_pixel_white_returns_255_255_255() {
        let w = MultiFilePngWriter::new("output", "frame_{{frame_number}}");

        let pixel = Color::ONE;
        let color = w.convert_pixel(pixel);
        assert_eq!(color, Rgb([255, 255, 255]));
    }
}
