use nalgebra::Vector3;

pub type Vec3 = Vector3<f32>;

use generational_arena::Index;

pub type EntityHandle = Index;
pub type ColliderHandle = Index;
