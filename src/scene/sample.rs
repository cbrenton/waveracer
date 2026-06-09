use std::sync::Arc;

use glam::{DVec3, IVec3};

use crate::{
    math::Color,
    render::{
        CheckerTexture, Dielectric, DiffuseLight, Hittable, Lambertian, SolidColor, Sphere,
        Triangle, TriangleMesh,
    },
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

    let light_tex = Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0)));
    let material_emit = Arc::new(DiffuseLight::new(light_tex));
    let material_glass = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));

    world.push(Hittable::Sphere(Sphere::new(
        DVec3::new(1.0, 0.0, -1.2),
        0.5,
        material_emit,
    )));
    world.push(Hittable::Sphere(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_glass,
    )));
    world.push(Hittable::Sphere(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));

    // create a fake "plane" via 2 20x20 triangles
    let plane_a = DVec3::new(-10.0, -0.5, -11.0);
    let plane_b = DVec3::new(-10.0, -0.5, 9.0);
    let plane_c = DVec3::new(10.0, -0.5, -11.0);
    let plane_d = DVec3::new(10.0, -0.5, 9.0);
    let checker_tex = Arc::new(CheckerTexture::new(
        0.32,
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));
    let material_checker = Arc::new(Lambertian::new(checker_tex));
    let plane_left = Triangle::new(plane_a, plane_b, plane_c, material_checker.clone());
    let plane_right = Triangle::new(plane_b, plane_c, plane_d, material_checker.clone());
    world.push(Hittable::Triangle(plane_left));
    world.push(Hittable::Triangle(plane_right));

    SceneData {
        world,
        name: String::from("sample"),
    }
}
