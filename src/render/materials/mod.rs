mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;
mod null;
mod texture;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use null::{NullMaterial, null_material_ptr};
pub use texture::*;

use glam::DVec3;
use std::{fmt::Debug, sync::Arc};

use crate::{
    math::{Color, Ray},
    render::HitRecord,
};

pub struct ScatterData {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Debug {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterData> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: DVec3) -> Color {
        Color::ZERO
    }
}
