use crate::consts::EPSILON;
use crate::types::{Vec3, Mat3, EntityHandle};
use crate::collider::{ColliderType, Collider, InternalCollider};

/// The internal representation of a plane collider.
#[derive(Debug)]
pub struct InternalPlaneCollider {
	/// The entity that this is linked to (if any).
	entity : Option<EntityHandle>,

	/// The position of a point on the plane.
	///
	/// This is in the parent entity's local space.
	pub position : Vec3,

	/// The plane's normal. Points AWAY from the side that this collider "fills".
	pub normal : Vec3,

	/// The total mass. Must not be negative.
	pub mass : f32,

	/// The restituion coefficient.
	pub restitution_coefficient : f32,
}

impl InternalPlaneCollider {
	/// Creates a new instance.
	pub fn new_from(source : &PlaneCollider) -> Result<Box<dyn InternalCollider>, ()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			Ok(Box::new(InternalPlaneCollider {
				entity: None,
				position: source.position.clone(),
				normal: source.normal.normalize(),
				mass: source.mass,
				restitution_coefficient: source.restitution_coefficient,
			}))
		}
	}

	/// Makes a PlaneCollider copying this instance's values.
	pub fn make_pub(&self) -> PlaneCollider {
		PlaneCollider {
			entity: self.entity.clone(),
			position: self.position.clone(),
			normal: self.normal.clone(),
			mass: self.mass,
			restitution_coefficient: self.restitution_coefficient,
		}
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : &PlaneCollider) -> Result<(),()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			self.position = source.position;
			self.normal = source.normal;
			self.mass = source.mass;
			self.restitution_coefficient = source.restitution_coefficient;
			Ok(())
		}
	}
}

impl InternalCollider for InternalPlaneCollider {
	/// The specific type.
	fn get_type(&self) -> ColliderType { ColliderType::PLANE }

	/// Sets the entity this is attached to, returning the previous one.
	fn set_entity(&mut self, handle : Option<EntityHandle>) -> Option<EntityHandle> {
		let old = self.entity;
		self.entity = handle;
		old
	}

	/// Retrieves the stored entity handle that this is attached to.
	fn get_entity(&mut self) -> Option<EntityHandle> { self.entity }

	/// Gets the center of mass for this collider.
	///
	/// This is relative to this collider's owning/linked/attached entity.
	fn get_local_center_of_mass(&self) -> Vec3 { self.position }

	fn get_mass(&self) -> f32 { self.mass }

	fn get_moment_of_inertia_tensor(&self) -> Mat3 { Mat3::zeros() }

	fn get_restitution_coefficient(&self) -> f32 { self.restitution_coefficient }
}

/// A copy of all of the publicly-accessible properties of an infinite plane collider.
///
/// This collider basically bisects space. Everything on one side is considered "in" the collision geometry, and will be collided out.
#[derive(Debug)]
pub struct PlaneCollider {
	/// The entity, if there is one. This is NOT copied back into InternalSphereCollider, hence why it's not "pub".
	///
	/// Defaults to None.
	entity : Option<EntityHandle>,

	/// The position of a point on the plane.
	///
	/// This is in the parent entity's local space.
	///
	/// Defaults to origin.
	pub position : Vec3,

	/// The plane's normal. Points AWAY from the side that this collider "fills".
	///
	/// Will be normalized when stored into the PhysicsSystem. The creation will be rejected if it's got a near-zero magnitude.
	///
	/// Defaults to +y.
	pub normal : Vec3,

	/// The total mass.
	///
	/// Defaults to zero.
	pub mass : f32,

	/// The restituion coefficient.
	///
	/// Defaults to one.
	pub restitution_coefficient : f32,
}

impl PlaneCollider {
	/// Creates an instance with all values at default.
	pub fn new() -> PlaneCollider {
		PlaneCollider {
			entity: None,
			position: Vec3::zeros(),
			normal: Vec3::y(),
			mass: 0.0,
			restitution_coefficient: 1.0,
		}
	}

	/// If this is in a valid state.
	pub fn is_valid(&self) -> bool {
		0.0 <= self.mass && EPSILON < self.normal.magnitude()
	}
}

impl Collider for PlaneCollider {
	fn get_type(&self) -> ColliderType { ColliderType::PLANE }

	fn get_entity(&self) -> Option<EntityHandle> { self.entity }

	fn get_center_of_mass(&self) -> Vec3 { self.position }
}
