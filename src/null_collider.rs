use crate::types::{Vec3, Mat3, EntityHandle};
use crate::collider::{ColliderType, Collider, InternalCollider};

/// The internal representation of a null collider.
#[derive(Debug)]
pub struct InternalNullCollider {
	/// The entity that this is linked to (if any).
	entity : Option<EntityHandle>,
	/// The position of the mass (relative to the parent's origin).
	pub position : Vec3,
	/// The total mass. Must not be negative.
	pub mass : f32,
	/// The moment of inertia tensor. May be a zero matrix if there isn't any.
	pub moment_of_inertia : Mat3,
}

impl InternalNullCollider {
	/// Creates a new instance.
	pub fn new_from(source : &NullCollider) -> Result<Box<dyn InternalCollider>, ()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			Ok(Box::new(InternalNullCollider {
				entity: None,
				position: source.position,
				mass: source.mass,
				moment_of_inertia: source.moment_of_inertia,
			}))
		}
	}

	/// Makes a NullCollider copying this instance's values.
	pub fn make_pub(&self) -> NullCollider {
		NullCollider {
			entity: self.entity,
			position: self.position,
			mass: self.mass,
			moment_of_inertia: self.moment_of_inertia,
		}
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : &NullCollider) -> Result<(),()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			self.position = source.position;
			self.mass = source.mass;
			self.moment_of_inertia = source.moment_of_inertia;
			Ok(())
		}
	}
}

impl InternalCollider for InternalNullCollider {
	/// The specific type.
	fn get_type(&self) -> ColliderType { ColliderType::NULL }

	/// Sets the entity this is attached to, returning the previous one.
	fn set_entity(&mut self, handle : Option<EntityHandle>) -> Option<EntityHandle> {
		let old = self.entity;
		self.entity = handle;
		old
	}

	/// Retrieves the stored entity handle that this is attached to.
	fn get_entity(&mut self) -> Option<EntityHandle> { self.entity }

	fn get_local_center_of_mass(&self) -> Vec3 { self.position }

	fn get_mass(&self) -> f32 { self.mass }

	fn get_moment_of_inertia_tensor(&self) -> Mat3 { self.moment_of_inertia }

	fn get_restitution_coefficient(&self) -> f32 { 0.0 }

	fn get_friction_coefficient(&self) -> f32 { 0.0 }
}

/// A collider that doesn't collide. Instead it just provides mass and inertia at a point.
#[derive(Debug)]
pub struct NullCollider {
	/// The entity that this is linked to (if any). This is NOT copied back into InternalSphereCollider, hence why it's not "pub".
	///
	/// Defaults to None.
	entity : Option<EntityHandle>,

	/// The position of the mass (relative to the parent's origin).
	///
	/// Defaults to origin.
	pub position : Vec3,

	/// The total mass. Must not be negative.
	///
	/// Defaults to zero.
	pub mass : f32,

	/// The moment of inertia tensor. May be a zero matrix if there isn't any.
	///
	/// Defaults to all zeros.
	pub moment_of_inertia : Mat3,
}

impl NullCollider {
	/// Creates an instance.
	pub fn new() -> NullCollider {
		NullCollider {
			entity: None,
			position: Vec3::zeros(),
			mass: 0.0,
			moment_of_inertia: Mat3::zeros(),
		}
	}

	/// Check if this is in a valid state.
	pub fn is_valid(&self) -> bool {
		0.0 <= self.mass
	}
}

impl Collider for NullCollider {
	fn get_type(&self) -> ColliderType { ColliderType::NULL }

	fn get_entity(&self) -> Option<EntityHandle> { self.entity }

	fn get_center_of_mass(&self) -> Vec3 { self.position }
}
