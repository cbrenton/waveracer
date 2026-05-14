use glam::DVec3;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: DVec3,
    pub normal: DVec3,
    // Whether we hit the outside of the object
    pub is_front_face: bool,
    // Texture UV coords
    pub u: f64,
    pub v: f64,
    // TODO: Optional material shared ptr
}
