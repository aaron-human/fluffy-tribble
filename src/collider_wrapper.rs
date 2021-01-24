use crate::sphere_collider::SphereCollider;

/// An easier way to pass a collider into the PhysicsSystem.
pub enum ColliderWrapper {
	Sphere(SphereCollider),
}
