mod dielectric;
mod lambertian;
mod material;
mod metal;
mod null;
mod texture;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use material::*;
pub use metal::Metal;
pub use null::{NullMaterial, null_material_ptr};
pub use texture::*;
