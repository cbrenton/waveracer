use std::{collections::LinkedList, mem::take};

use glam::DVec3;
use kdam::{BarExt, tqdm};

use crate::{
    math::{
        Color, Ray,
        random::{random_double, random_in_unit_disk},
    },
    render::{CameraState, FrameData, Hittable, LerpTransition, MonteCarloRenderer},
};

#[derive(Debug)]
struct CameraFrameState {
    pixel00_loc: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    camera_center: DVec3,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
}

pub struct VideoCamera {
    vfov: f64,
    renderer: MonteCarloRenderer,
    transitions: LinkedList<LerpTransition>,
    pub cur_frame: i32,
    pub total_frames: i32,
    // TODO: this is gnarly. I should ask somebody if there's a better way
    trans_iterator: Option<Box<dyn Iterator<Item = CameraState>>>,
    defocus_angle: f64,
    // TODO: move to Film
    pub width: usize,
    pub height: usize,
}

impl VideoCamera {
    pub fn new(vfov: f64, renderer: MonteCarloRenderer, width: usize, height: usize) -> Self {
        Self {
            vfov,
            renderer,
            transitions: LinkedList::new(),
            cur_frame: 0,
            total_frames: 20,
            trans_iterator: None,
            // TODO: take this as param/put it somewhere else
            defocus_angle: 1.0,
            width,
            height,
        }
    }

    pub fn add_transition(&mut self, transition: LerpTransition) {
        self.transitions.push_back(transition);
    }

    pub fn capture_frame(&mut self, world: &[Hittable]) -> Option<FrameData> {
        if !self.is_rolling() {
            // TODO: improve this
            panic!("camera isn't rolling");
        }
        let state = self.trans_iterator.as_mut().and_then(Iterator::next);
        // TODO: so gross. make this better
        if state.is_none() {
            self.trans_iterator = None;
            return None;
        }
        // dbg!(state);
        let pixels = self.foo(world, &state.unwrap());
        let result = FrameData {
            w: self.width,
            h: self.height,
            pixels,
            frame_number: self.cur_frame,
            t: 0.0,
        };
        self.cur_frame += 1;
        Some(result)
    }

    pub fn roll(&mut self) {
        // self.renderer.prebake();

        // use std::mem::take to take ownership of the transitions field of a mutable struct
        let transitions = take(&mut self.transitions);
        // sigh...OPTION of FAT POINTER of something that IMPLEMENTS iterator OVER camerastate
        self.trans_iterator = Some(Box::new(transitions.into_iter().flatten()));
    }

    pub fn is_rolling(&self) -> bool {
        // TODO: I don't think this is correct
        self.trans_iterator.is_some() // self.cur_frame <= self.total_frames
    }

    // TODO: rename. also just redo this entire file
    fn foo(&self, world: &[Hittable], state: &CameraState) -> Vec<Color> {
        let mut result: Vec<Color> = vec![];

        let frame_config = self.generate_frame_config(state);

        let mut bar = tqdm!(total = self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let mut pixel_color = Color::ZERO;

                // cast SAMPLES_PER_PIXEL random-ish rays and then divide total color by
                // SAMPLES_PER_PIXEL for simple antialiasing
                for _ in 0..self.renderer.samples_per_pixel {
                    let ray = self.get_ray(x, y, &frame_config);

                    pixel_color += self.renderer.ray_color(&ray, world, 0);
                }
                // NOTE: I chose to not include samples in my progress bar for the sake of having a
                // semi-readable number
                bar.update(1).unwrap();
                result.push(pixel_color / self.renderer.samples_per_pixel as f64);
            }
        }

        result
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
        dbg!(pixel00_loc);

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
    fn get_ray(&self, x: usize, y: usize, frame_config: &CameraFrameState) -> Ray {
        let offset = self.sample_square();
        // let offset = DVec3::ZERO;

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
