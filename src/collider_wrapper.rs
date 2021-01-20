use crate::sphere_collider::SphereCollider;

/// An easier way to pass a collider into the PhysicsSystem.
/// Can't quite use this pattern internally (as using an enum usually consumes it).
pub enum ColliderWrapper {
	Sphere(SphereCollider),
}
