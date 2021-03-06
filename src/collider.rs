use core::fmt::Debug;

use downcast_rs::{Downcast, impl_downcast};

use crate::types::{Vec3, Mat3, EntityHandle};

/// A way to quickly determine collider type.
#[derive(PartialEq, Eq)]
pub enum ColliderType {
	/// For the [crate::NullCollider].
	NULL,
	/// For the [crate::SphereCollider].
	SPHERE,
	/// For the [crate::PlaneCollider].
	PLANE,
	/// For the [crate::MeshCollider].
	MESH,
	/// For the [crate::AlignedBox].
	ALIGNED_BOX,
}

/// The internal representation of an arbitrary collider.
/// This generally will have NO data hiding to keep things simple.
pub trait InternalCollider : Downcast + Debug {
	/// The specific type.
	fn get_type(&self) -> ColliderType;

	/// Sets the entity this is attached to.
	fn set_entity(&mut self, handle : Option<EntityHandle>) -> Option<EntityHandle>;

	/// Retrieves the stored entity handle that this is attached to.
	fn get_entity(&mut self) -> Option<EntityHandle>;

	/// Gets the center of mass for this collider in it's owning entity's local space.
	fn get_local_center_of_mass(&self) -> Vec3;

	/// Gets the mass of this collider. Must not be negative.
	fn get_mass(&self) -> f32;

	/// Gets the moment of inertia tensor about the center of mass.
	///
	/// This is oriented according to the owning entity's local space.
	fn get_moment_of_inertia_tensor(&self) -> Mat3;

	/// Gets the coefficient of restitution for this instance.
	fn get_restitution_coefficient(&self) -> f32;

	/// Gets the friction ratio threshold used to decide whether to use static or dynamic friction.
	fn get_friction_threshold(&self) -> f32;

	/// Gets the static friction coefficient.
	fn get_static_friction_coefficient(&self) -> f32;

	/// Gets the dynamic friction coefficient.
	fn get_dynamic_friction_coefficient(&self) -> f32;
}

impl dyn InternalCollider {
	// Nothing for now.
}

impl_downcast!(InternalCollider);

/// The generic public representation of an arbitrary collider.
pub trait Collider : Downcast + Debug {
	/// The specific type.
	fn get_type(&self) -> ColliderType;

	/// Gets the entity this is linked to (if there is one).
	///
	/// This is read-only. To link things together, use PhysicsSystem.link_collider().
	fn get_entity(&self) -> Option<EntityHandle>;

	/// Gets the center of mass for this collider in it's owning entity's local space.
	fn get_center_of_mass(&self) -> Vec3;
}

impl dyn Collider {
	// Nothing for now.
}

impl_downcast!(Collider);
