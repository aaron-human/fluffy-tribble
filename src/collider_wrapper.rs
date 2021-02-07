use crate::null_collider::NullCollider;
use crate::sphere_collider::SphereCollider;
use crate::plane_collider::PlaneCollider;
use crate::mesh_collider::MeshCollider;

/// How [crate::Collider] generics are passed into [crate::PhysicsSystem].
///
/// As it turns out, an enum is easier to work with than a `Box<dyn ...>`.
pub enum ColliderWrapper {
	Null(NullCollider),
	Sphere(SphereCollider),
	Plane(PlaneCollider),
	Mesh(MeshCollider),
}
