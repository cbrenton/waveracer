use std::{
    fs,
    path::{Path, PathBuf},
};

use image::{Rgb, RgbImage};

use crate::{
    math::{Color, IInterval, linear_to_gamma},
    render::RenderConfig,
};

pub struct PngImageWriter {
    pub width: usize,
    pub height: usize,
}

impl PngImageWriter {
    pub fn new(render_config: &RenderConfig) -> Self {
        Self {
            width: render_config.width,
            height: render_config.height,
        }
    }

    pub fn write(&self, pixel_data: Vec<Color>, mut output: PathBuf) {
        let dirname = output.parent().unwrap();

        let mut buf = RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = buf.get_pixel_mut(x as u32, y as u32);
                *pixel = self.convert_pixel(pixel_data[y * self.width + x]);
            }
        }

        // create output dir
        let _ = fs::create_dir(dirname);

        // force png extension
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

    pub const INTENSITY: IInterval = IInterval { min: 0, max: 255 };
}

#[cfg(test)]
mod tests {
    use crate::render::{BasicCameraConfig, BasicCameraRenderConfig};

    use super::*;

    #[test]
    fn test_write_appends_png_to_filename_if_missing() {
        let w = PngImageWriter {
            width: 1,
            height: 1,
        };
        let tmp = tempfile::tempdir().unwrap();

        w.write(vec![Color::default()], tmp.path().join("output/out"));

        assert!(tmp.path().join("output/out.png").is_file());
    }

    #[test]
    fn test_write_leaves_filename_alone_if_already_has_extension() {
        let w = PngImageWriter {
            width: 1,
            height: 1,
        };
        let tmp = tempfile::tempdir().unwrap();

        w.write(vec![Color::default()], tmp.path().join("output/out.png"));

        assert!(tmp.path().join("output/out.png").is_file());
    }

    #[test]
    fn test_write_creates_output_dir_if_dne() {
        let w = PngImageWriter {
            width: 1,
            height: 1,
        };
        let tmp = tempfile::tempdir().unwrap();

        w.write(vec![Color::default()], tmp.path().join("output/out.png"));

        assert!(tmp.path().join("output/").is_dir());
    }

    #[test]
    fn test_convert_pixel_black_returns_0_0_0() {
        let w = PngImageWriter {
            width: 1,
            height: 1,
        };

        let pixel = Color::ZERO;
        let color = w.convert_pixel(pixel);
        assert_eq!(color, Rgb([0, 0, 0]));
    }

    #[test]
    fn test_convert_pixel_white_returns_255_255_255() {
        let w = PngImageWriter {
            width: 1,
            height: 1,
        };

        let pixel = Color::ONE;
        let color = w.convert_pixel(pixel);
        assert_eq!(color, Rgb([255, 255, 255]));
    }
}
