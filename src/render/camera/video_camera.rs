use std::{collections::LinkedList, mem::take};

use glam::DVec3;
use kdam::{BarExt, tqdm};

use crate::{
    math::{
        Color,
        random::{random_double, random_in_unit_disk},
    },
    render::{
        CameraState, DummyRenderer, Film, FrameData, Hittable, LerpTransition, RaySpawner, Renderer,
    },
};

pub struct VideoCamera {
    pub vfov: f64,
    renderer: DummyRenderer,
    transitions: LinkedList<LerpTransition>,
    pub cur_frame: i32,
    pub total_frames: i32,
    pub film: Film,
}

impl VideoCamera {
    pub fn new(vfov: f64, renderer: DummyRenderer, film: Film) -> Self {
        Self {
            vfov,
            renderer,
            transitions: LinkedList::new(),
            cur_frame: 0,
            total_frames: 20,
            film,
        }
    }

    pub fn add_transition(&mut self, transition: LerpTransition) {
        self.transitions.push_back(transition);
    }

    pub fn render_frame(
        &self,
        world: &[Hittable],
        camera_state: &CameraState,
        frame_number: usize,
    ) -> FrameData {
        let mut pixels: Vec<Color> = vec![];

        let ray_spawner = RaySpawner::new(self, camera_state);

        let mut bar = tqdm!(
            total = self.film.width * self.film.height,
            position = 1,
            desc = "  frame"
        );
        for y in 0..self.film.height {
            for x in 0..self.film.width {
                let mut pixel_color = Color::ZERO;

                // cast SAMPLES_PER_PIXEL random-ish rays and then divide total color by
                // SAMPLES_PER_PIXEL for simple antialiasing
                for _ in 0..self.film.samples_per_pixel {
                    let offset = self.sample_square();

                    let ray = ray_spawner.generate_primary_ray(x, y, offset);

                    pixel_color += self.renderer.ray_color(&ray, world, 0);
                }
                // NOTE: I chose to not include samples in my progress bar for the sake of having a
                // semi-readable number
                bar.update(1).unwrap();
                pixels.push(pixel_color / self.film.samples_per_pixel as f64);
            }
        }

        FrameData {
            w: self.film.width,
            h: self.film.height,
            pixels,
            frame_number,
            t: 0.0,
        }
    }

    fn sample_square(&self) -> DVec3 {
        DVec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
