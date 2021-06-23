use crate::consts::*;
use crate::types::{Vec3, Mat3, EntityHandle, min, max};
use crate::collider::{ColliderType, Collider, InternalCollider};
use crate::orientation::Orientation;

/// The internal representation of an axis-aligned rectangular prism collider.
#[derive(Debug)]
pub struct InternalAlignedBoxCollider {
	/// The entity that this is linked to (if any).
	entity : Option<EntityHandle>,

	/// The position of this collider's origin.
	///
	/// This is in the parent entity's local space.
	pub position : Vec3,

	/// The corner with all of the smaller values.
	pub min_corner : Vec3,
	/// The corner with all of the larger values.
	pub max_corner : Vec3,

	/// The total mass. Must not be negative.
	pub mass : f32,

	/// The restituion coefficient.
	pub restitution_coefficient : f32,

	/// The ratio used to decide whether to use static friction or dynamic friction.
	pub friction_threshold : f32,

	/// The static friction coefficient. Should always at or between 0.0 and 1.0.
	pub static_friction_coefficient : f32,

	/// The dynamic friction coefficient. Should always at or between 0.0 and 1.0.
	pub dynamic_friction_coefficient : f32,
}

impl InternalAlignedBoxCollider {
	pub fn new_from(source : &AlignedBoxCollider) -> Result<Box<dyn InternalCollider>, ()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			Ok(Box::new(InternalAlignedBoxCollider {
				entity: None,
				position: source.position.clone(),
				min_corner: Vec3::new(
					min(source.min_corner.x, source.max_corner.x),
					min(source.min_corner.y, source.max_corner.y),
					min(source.min_corner.z, source.max_corner.z),
				),
				max_corner: Vec3::new(
					max(source.min_corner.x, source.max_corner.x),
					max(source.min_corner.y, source.max_corner.y),
					max(source.min_corner.z, source.max_corner.z),
				),
				mass: source.mass,
				restitution_coefficient: source.restitution_coefficient,
				friction_threshold: source.friction_threshold,
				static_friction_coefficient: source.static_friction_coefficient,
				dynamic_friction_coefficient: source.dynamic_friction_coefficient,
			}))
		}
	}

	/// Makes a AlignedBoxCollider copying this instance's values.
	pub fn make_pub(&self) -> AlignedBoxCollider {
		AlignedBoxCollider {
			entity: self.entity.clone(),
			position: self.position.clone(),
			min_corner: self.min_corner.clone(),
			max_corner: self.max_corner.clone(),
			mass: self.mass,
			restitution_coefficient: self.restitution_coefficient,
			friction_threshold: self.friction_threshold,
			static_friction_coefficient: self.static_friction_coefficient,
			dynamic_friction_coefficient: self.dynamic_friction_coefficient,
		}
	}

	/// Updates from the passed in AlignedBoxCollider object.
	pub fn update_from(&mut self, source : &AlignedBoxCollider) -> Result<(),()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			self.position = source.position;
			self.min_corner = Vec3::new(
				min(source.min_corner.x, source.max_corner.x),
				min(source.min_corner.y, source.max_corner.y),
				min(source.min_corner.z, source.max_corner.z),
			);
			self.max_corner = Vec3::new(
				max(source.min_corner.x, source.max_corner.x),
				max(source.min_corner.y, source.max_corner.y),
				max(source.min_corner.z, source.max_corner.z),
			);
			self.mass = source.mass;
			self.restitution_coefficient = source.restitution_coefficient;
			self.friction_threshold = source.friction_threshold;
			self.static_friction_coefficient = source.static_friction_coefficient;
			self.dynamic_friction_coefficient = source.dynamic_friction_coefficient;
			Ok(())
		}
	}
}

impl InternalCollider for InternalAlignedBoxCollider {
	/// The specific type.
	fn get_type(&self) -> ColliderType { ColliderType::ALIGNED_BOX }

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
	/// This IS NOT relative to this collider's "position" property.
	fn get_local_center_of_mass(&self) -> Vec3 { self.position + 0.5 * (self.min_corner + self.max_corner) }

	fn get_mass(&self) -> f32 { self.mass }

	fn get_moment_of_inertia_tensor(&self) -> Mat3 {
		let mut size = self.max_corner - self.min_corner;
		size.x *= size.x; size.y *= size.y; size.z *= size.z;
		let coefficient = self.mass / 12.0;
		Mat3::from_diagonal(&Vec3::new(
			coefficient * (size.y + size.z),
			coefficient * (size.x + size.z),
			coefficient * (size.x + size.y),
		))
	}

	fn get_restitution_coefficient(&self) -> f32 { self.restitution_coefficient }

	fn get_friction_threshold(&self) -> f32 { self.friction_threshold }

	fn get_static_friction_coefficient(&self) -> f32 { self.static_friction_coefficient }

	fn get_dynamic_friction_coefficient(&self) -> f32 { self.dynamic_friction_coefficient }
}

/// A copy of all of the publicly-accessible properties of an axis-aligned rectangular prism collider.
#[derive(Debug)]
pub struct AlignedBoxCollider {
	/// The entity that this is linked to (if any).
	///
	/// Defaults to None.
	entity : Option<EntityHandle>,

	/// The position of this collider's origin.
	///
	/// This is in the parent entity's local space.
	///
	/// Defaults to all zeros.
	pub position : Vec3,

	/// The corner with all of the smaller values.
	///
	/// This doesn't need to store the min corner for this to be valid; it only needs to be more than `EPSILON` from `max_corner`.
	///
	/// Defaults to origin.
	pub min_corner : Vec3,

	/// The corner with all of the larger values.
	///
	/// This doesn't need to store the max corner for this to be valid; it only needs to be more than `EPSILON` from `min_corner`.
	///
	/// Defaults to `(1.0, 1.0, 1.0)`.
	pub max_corner : Vec3,

	/// The total mass. Must not be negative.
	///
	/// Defaults to `1.0`.
	pub mass : f32,

	/// The restituion coefficient.
	///
	/// Defaults to one.
	pub restitution_coefficient : f32,

	/// The ratio used to decide whether to use static friction or dynamic friction.
	///
	/// Defaults to `1.0`.
	pub friction_threshold : f32,

	/// The static friction coefficient. Should always at or between 0.0 and 1.0.
	///
	/// Defaults to `0.25`.
	pub static_friction_coefficient : f32,

	/// The dynamic friction coefficient. Should always at or between 0.0 and 1.0.
	///
	/// Defaults to `0.3`.
	pub dynamic_friction_coefficient : f32,
}

impl AlignedBoxCollider {
	/// Creates a unit cube (from origin to (1.0, 1.0, 1.0)) with all values at default.
	pub fn new() -> AlignedBoxCollider {
		AlignedBoxCollider {
			entity: None,
			position: Vec3::zeros(),
			min_corner: Vec3::zeros(),
			max_corner: Vec3::new(1.0, 1.0, 1.0),
			mass: 0.0,
			restitution_coefficient: 1.0,
			friction_threshold: 0.25,
			static_friction_coefficient: 1.0,
			dynamic_friction_coefficient: 0.3,
		}
	}

	/// If this is in a valid state.
	pub fn is_valid(&self) -> bool {
		let size = self.max_corner - self.min_corner;
		EPSILON < size.x && EPSILON < size.y && EPSILON < size.z && 0.0 <= self.mass
	}
}

impl Collider for AlignedBoxCollider {
	fn get_type(&self) -> ColliderType { ColliderType::ALIGNED_BOX }

	fn get_entity(&self) -> Option<EntityHandle> { self.entity }

	fn get_center_of_mass(&self) -> Vec3 { self.position + 0.5 * (self.min_corner + self.max_corner) }
}
