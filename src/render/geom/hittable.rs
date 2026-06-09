use crate::{
    math::{Bounds3, DInterval, Ray},
    render::{HitRecord, Sphere, Triangle, TriangleMesh},
};

// TODO: add triangle, hittablelist, bvhnode, etc
pub enum Hittable {
    Dummy,
    Sphere(Sphere),
    Triangle(Triangle),
    // This is a Box because enums are sized according to their largest possible value. A
    // TriangleMesh can be enormous, so we want to store it in a Box so that it doesn't make
    // Hittable instances be really really big.
    TriangleMesh(Box<TriangleMesh>),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord> {
        match self {
            Hittable::Dummy => None,
            Hittable::Sphere(sphere) => sphere.hit(ray, ray_t),
            Hittable::Triangle(tri) => tri.hit(ray, ray_t),
            Hittable::TriangleMesh(mesh) => mesh.hit(ray, ray_t),
        }
    }

    pub fn aabb(&self) -> Bounds3 {
        match self {
            Hittable::Dummy => Bounds3::EMPTY,
            Hittable::Sphere(sphere) => sphere.aabb(),
            Hittable::Triangle(tri) => tri.aabb(),
            Hittable::TriangleMesh(mesh) => mesh.aabb(),
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use super::*;

    #[test]
    fn test_hit_dummy_hittable_returns_none() {
        let foo = Hittable::Dummy;
        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;
        assert!(foo.hit(&ray, ray_t).is_none());
    }
}
