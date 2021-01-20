use core::fmt::Debug;

use downcast_rs::{Downcast, impl_downcast};

use crate::types::EntityHandle;

/// All the types of colliders.
#[derive(PartialEq, Eq)]
pub enum ColliderType {
	SPHERE,
}

/// The internal representation of an arbitrary collider.
/// This generally will have NO data hiding to keep things simple.
pub trait InternalCollider : Downcast + Debug {
	/// The specific type.
	fn get_type(&self) -> ColliderType;
	/// Sets the entity this is attached to.
	fn set_entity_handle(&mut self, handle : Option<EntityHandle>) -> Option<EntityHandle>;
	/// Retrieves the stored entity handle that this is attached to.
	fn get_entity_handle(&mut self) -> Option<EntityHandle>;
}

impl dyn InternalCollider {
	// Nothing for now.
}

impl_downcast!(InternalCollider);

/// The public representation of an arbitrary collider.
pub trait Collider : Downcast + Debug {
	/// The specific type.
	fn get_type(&self) -> ColliderType;

	/// Gets the entity this is linked to (if there is one).
	/// This is read-only. To link things together, use PhysicsSystem.link_collider().
	fn get_entity_handle(&self) -> Option<EntityHandle>;
}

impl dyn Collider {
	// Nothing for now.
}

impl_downcast!(Collider);
