use crate::render::{Hittable, VideoCamera};

pub struct SceneData<T> {
    pub world: Vec<Hittable>,
    pub name: String,
    pub camera: VideoCamera<T>,
    // TODO: add accel structure here maybe?
}
