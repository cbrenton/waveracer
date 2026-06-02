use std::sync::Arc;

use glam::{DVec3, IVec3};
use image::imageops::FilterType::Triangle;

use crate::{
    math::Color,
    render::{Hittable, Lambertian, TriangleMesh},
};

pub struct SceneData {
    pub world: Vec<Hittable>,
    pub name: String,
    // TODO: add accel structure here maybe?
}

pub fn sample_scene() -> SceneData {
    let mut world: Vec<Hittable> = vec![];

    let material_center = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));

    let a = DVec3::new(-0.7, 0.5, -1.2);
    let b = DVec3::new(0.7, 0.5, -1.2);
    let c = DVec3::new(0.0, -0.5, -1.2);
    let d = DVec3::new(0.0, 0.0, -0.7);
    let vertices = vec![a, b, c, d];
    let triangles = vec![
        IVec3::new(0, 2, 3),
        IVec3::new(2, 1, 3),
        IVec3::new(1, 0, 3),
    ];
    let mesh = TriangleMesh::new(vertices, triangles, material_center);

    world.push(Hittable::TriangleMesh(Box::new(mesh)));

    SceneData {
        world,
        name: String::from("sample"),
    }
}
