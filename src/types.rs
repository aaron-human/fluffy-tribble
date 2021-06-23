use nalgebra::{Vector3, Matrix3, UnitQuaternion, Isometry3};

pub type Mat3 = Matrix3<f32>;
pub type Vec3 = Vector3<f32>;
pub type Quat = UnitQuaternion<f32>;
pub type Isometry = Isometry3<f32>;

use generational_arena::Index;

/// A way to reference a [crate::Entity] stored in [crate::PhysicsSystem] without actually having a ref to it.
pub type EntityHandle = Index;

/// A way to reference a [crate::Collider] stored in [crate::PhysicsSystem] without actually having a ref to it.
pub type ColliderHandle = Index;

/// A way to reference a [crate::UnaryForceGenerator] stored in [crate::PhysicsSystem] without actually having a ref to it.
pub type UnaryForceGeneratorHandle = Index;

/// Gets the minimum of two float values.
pub fn min(val1 : f32, val2: f32) -> f32 {
	if val1 < val2 { val1 } else { val2 }
}

/// Gets the maximum of two float values.
pub fn max(val1 : f32, val2: f32) -> f32 {
	if val1 > val2 { val1 } else { val2 }
}
