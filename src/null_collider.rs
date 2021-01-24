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
	pub fn new(position : &Vec3, mass : f32, moment_of_inertia : Mat3) -> Result<Box<dyn InternalCollider>, ()> {
		if 0.0 > mass {
			Err(()) // TODO: An error type.
		} else {
			Ok(Box::new(InternalNullCollider {
				entity: None,
				position: *position,
				mass,
				moment_of_inertia,
			}))
		}
	}

	/// Creates from an NullCollider.
	pub fn from(source : &NullCollider) -> Result<Box<dyn InternalCollider>, ()> {
		InternalNullCollider::new(
			&source.position,
			source.mass,
			source.moment_of_inertia,
		)
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
		if 0.0 > source.mass {
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

	/// Gets the center of mass for this collider.
	/// This is relative to this collider's owning/linked/attached entity.
	/// This IS NOT relative to this collider's "center" property.
	fn get_local_center_of_mass(&self) -> Vec3 { self.position }

	fn get_mass(&self) -> f32 { self.mass }

	fn get_moment_of_inertia_tensor(&self) -> Mat3 { self.moment_of_inertia }

	fn get_restitution_coefficient(&self) -> f32 { 0.0 }
}

/// A collider that doesn't collide. Instead it just provides mass and inertia at a point.
#[derive(Debug)]
pub struct NullCollider {
	/// The entity that this is linked to (if any).
	entity : Option<EntityHandle>,
	/// The position of the mass (relative to the parent's origin).
	pub position : Vec3,
	/// The total mass. Must not be negative.
	pub mass : f32,
	/// The moment of inertia tensor. May be a zero matrix if there isn't any.
	pub moment_of_inertia : Mat3,
}

impl NullCollider {
	/// Creates an instance.
	pub fn new(position : &Vec3, mass : f32, moment_of_inertia : Mat3) -> NullCollider {
		NullCollider { entity: None, position: position.clone(), mass, moment_of_inertia, }
	}
}

impl Collider for NullCollider {
	fn get_type(&self) -> ColliderType { ColliderType::NULL }

	fn get_entity(&self) -> Option<EntityHandle> { self.entity }

	fn get_center_of_mass(&self) -> Vec3 { self.position }
}
