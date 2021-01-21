use nalgebra::{Vector3, Matrix3};

pub type Mat3 = Matrix3<f32>;
pub type Vec3 = Vector3<f32>;

use generational_arena::Index;

pub type EntityHandle = Index;
pub type ColliderHandle = Index;
