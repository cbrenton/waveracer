use std::{collections::LinkedList, mem::take};

use glam::DVec3;
use kdam::{BarExt, tqdm};

use crate::{
    math::{
        Color, Ray,
        random::{random_double, random_in_unit_disk},
    },
    render::{
        CameraState, CameraTransition, FrameData, Hittable, MonteCarloRenderer,
        monte_carlo_renderer::Renderer,
    },
};

// TODO: move somewhere else, take generate_primary_ray with it
#[derive(Debug)]
struct CameraFrameState {
    pixel00_loc: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    camera_center: DVec3,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
}

pub struct VideoCamera<T> {
    vfov: f64,
    renderer: T,
    transitions: LinkedList<CameraTransition>,
    pub cur_frame: i32,
    pub total_frames: usize,
    defocus_angle: f64,
    // TODO: move to Film
    pub width: usize,
    pub height: usize,
}

impl<T: Renderer> VideoCamera<T> {
    pub fn new(vfov: f64, renderer: T, width: usize, height: usize) -> Self {
        Self {
            vfov,
            renderer,
            transitions: LinkedList::new(),
            cur_frame: 0,
            total_frames: 0,
            // TODO: take this as param/put it somewhere else
            defocus_angle: 1.0,
            width,
            height,
        }
    }

    pub fn add_transition(&mut self, transition: CameraTransition) {
        self.total_frames += transition.ticks;
        self.transitions.push_back(transition);
    }

    // TODO: change this from Option to Result
    fn render_frame(
        &self,
        world: &[Hittable],
        camera_state: &CameraState,
        frame_number: usize,
    ) -> FrameData {
        let mut pixels: Vec<Color> = vec![];

        let frame_config = self.generate_frame_config(camera_state);

        let mut bar = tqdm!(
            total = self.width * self.height,
            position = 1,
            desc = "  frame"
        );
        for y in 0..self.height {
            for x in 0..self.width {
                let mut pixel_color = Color::ZERO;

                // cast SAMPLES_PER_PIXEL random-ish rays and then divide total color by
                // SAMPLES_PER_PIXEL for simple antialiasing
                for _ in 0..self.renderer.samples_per_pixel() {
                    let offset = self.sample_square();

                    let ray = self.generate_primary_ray(x, y, offset, &frame_config);

                    pixel_color += self.renderer.ray_color(&ray, world, 0);
                }
                // NOTE: I chose to not include samples in my progress bar for the sake of having a
                // semi-readable number
                bar.update(1).unwrap();
                pixels.push(pixel_color / self.renderer.samples_per_pixel() as f64);
            }
        }

        FrameData {
            w: self.width,
            h: self.height,
            pixels,
            frame_number,
            t: 0.0,
        }
    }

    pub fn render_frames(&self, world: &[Hittable]) -> impl Iterator<Item = FrameData> {
        self.transitions
            .clone()
            .into_iter()
            .flatten()
            .enumerate()
            .map(move |(frame, s)| self.render_frame(world, &s, frame))
    }

    fn generate_frame_config(&self, state: &CameraState) -> CameraFrameState {
        // let image_h: f64 = (self.width as f64 / (16.0 / 9.0)).max(1.0);
        let camera_center = state.pos;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        // TODO: fix (put in config)
        let focus_distance = 1.4;
        let viewport_height = 2.0 * h * focus_distance;
        // recalculate aspect ratio because image_h might not be what we intended
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);

        // calculate u,v,w unit basis vectors for camera
        let w = (camera_center - state.look_at).normalize();
        let u = state.up.cross(w).normalize();
        let v = w.cross(u);

        // calculate the vectors along the horizontal and vertical edges of the viewport
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / self.width as f64;
        let pixel_delta_v = viewport_v / self.height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left =
            camera_center - (focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        // dbg!(pixel00_loc);

        let defocus_radius = focus_distance * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        CameraFrameState {
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    /// Spawns a single ray at pixel [x, y] using the CameraRenderConfig's settings. For this
    /// camera the ray is offset by the defocus amount to add depth of field
    fn generate_primary_ray(
        &self,
        x: usize,
        y: usize,
        offset: DVec3,
        frame_config: &CameraFrameState,
    ) -> Ray {
        let pixel_center = frame_config.pixel00_loc
            + ((x as f64 + offset.x) * frame_config.pixel_delta_u)
            + ((y as f64 + offset.y) * frame_config.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            frame_config.camera_center
        } else {
            self.defocus_disk_sample(frame_config)
        };
        // let ray_origin = frame_config.camera_center;
        let ray_direction = pixel_center - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self, frame_config: &CameraFrameState) -> DVec3 {
        let p = random_in_unit_disk();
        frame_config.camera_center
            + (p.x * frame_config.defocus_disk_u)
            + (p.y * frame_config.defocus_disk_v)
    }

    fn sample_square(&self) -> DVec3 {
        DVec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::render::monte_carlo_renderer::MockRenderer;

    use super::*;

    #[test]
    fn test_add_transition_updates_total_frames() {
        let ren = MockRenderer::new();
        let mut cam = VideoCamera::new(90.0, ren, 10, 10);

        let trans = LerpTransition::hold(&CameraState::default(), 10);
        cam.add_transition(trans);
        assert_eq!(cam.total_frames, 10);

        let trans2 = LerpTransition::hold(&CameraState::default(), 15);
        cam.add_transition(trans2);
        assert_eq!(cam.total_frames, 25);
    }

    #[test]
    fn test_add_transition_appends_transition() {
        let ren = MockRenderer::new();
        let mut cam = VideoCamera::new(90.0, ren, 10, 10);

        let trans = LerpTransition::hold(&CameraState::default(), 10);
        cam.add_transition(trans);

        assert_eq!(cam.transitions.len(), 1);

        let trans2 = LerpTransition::hold(&CameraState::default(), 15);
        cam.add_transition(trans2);
        assert_eq!(cam.transitions.len(), 2);
    }

    #[test]
    #[should_panic = "camera isn't rolling"]
    fn test_capture_frame_panics_if_not_rolling() {
        let ren = MockRenderer::new();
        let mut cam = VideoCamera::new(90.0, ren, 10, 10);
        let world: Vec<Hittable> = vec![];
        cam.capture_frame(&world);
    }

    #[test]
    fn test_capture_frame_advances_frame() {
        let ren = MockRenderer::new();
        let mut cam = VideoCamera::new(90.0, ren, 10, 10);
        let world: Vec<Hittable> = vec![];

        cam.roll();
        assert_eq!(cam.cur_frame, 0);

        cam.capture_frame(&world);
        assert_eq!(cam.cur_frame, 1);
    }

    #[test]
    fn test_capture_frame_advances_cur_transition_if_not_finished() {
        let mut ren = MockRenderer::new();
        ren.expect_samples_per_pixel().returning(|| 1);
        ren.expect_ray_color().returning(|_, _, _| Color::ZERO);
        let mut cam = VideoCamera::new(90.0, ren, 10, 10);
        let world: Vec<Hittable> = vec![];

        let trans = LerpTransition::hold(&CameraState::default(), 10);
        cam.add_transition(trans);
        cam.roll();

        dbg!(cam.transitions);
        println!("foo");
        /*
        assert_eq!(cam.transitions.front().unwrap().ticks_left(), 10);

        cam.capture_frame(&world);
        assert_eq!(cam.transitions.front().unwrap().ticks_left(), 9);
        */
    }
}
*/
