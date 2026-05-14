use std::sync::Arc;

use glam::DVec3;

use crate::{
    math::{Color, Ray},
    render::HitRecord,
};

pub struct ScatterData {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterData> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: DVec3) -> Color {
        Color::ZERO
    }
}
