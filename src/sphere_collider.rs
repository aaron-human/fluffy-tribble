use crate::types::{Vec3, Mat3, EntityHandle};
use crate::collider::{ColliderType, Collider, InternalCollider};

/// The minimum radius
const MINIMUM_RADIUS : f32 = 0.05;

/// The internal representation of a sphere collider.
#[derive(Debug)]
pub struct InternalSphereCollider {
	/// The entity that this is linked to (if any).
	entity : Option<EntityHandle>,

	/// The position of the center.
	///
	/// This is in the parent entity's local space.
	pub center : Vec3,

	/// The radius.
	pub radius : f32,

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

impl InternalSphereCollider {
	/// Creates a new instance.
	pub fn new_from(source : &SphereCollider) -> Result<Box<dyn InternalCollider>, ()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			Ok(Box::new(InternalSphereCollider {
				entity: None,
				center: source.center.clone(),
				radius: source.radius,
				mass: source.mass,
				restitution_coefficient: source.restitution_coefficient,
				friction_threshold: source.friction_threshold,
				static_friction_coefficient: source.static_friction_coefficient,
				dynamic_friction_coefficient: source.dynamic_friction_coefficient,
			}))
		}
	}

	/// Makes a SphereCollider copying this instance's values.
	pub fn make_pub(&self) -> SphereCollider {
		SphereCollider {
			entity: self.entity.clone(),
			center: self.center.clone(),
			radius: self.radius,
			mass: self.mass,
			restitution_coefficient: self.restitution_coefficient,
			friction_threshold: self.friction_threshold,
			static_friction_coefficient: self.static_friction_coefficient,
			dynamic_friction_coefficient: self.dynamic_friction_coefficient,
		}
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : &SphereCollider) -> Result<(),()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			self.center = source.center;
			self.radius = source.radius;
			self.mass = source.mass;
			self.restitution_coefficient = source.restitution_coefficient;
			self.friction_threshold = source.friction_threshold;
			self.static_friction_coefficient = source.static_friction_coefficient;
			self.dynamic_friction_coefficient = source.dynamic_friction_coefficient;
			Ok(())
		}
	}
}

impl InternalCollider for InternalSphereCollider {
	/// The specific type.
	fn get_type(&self) -> ColliderType { ColliderType::SPHERE }

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
	fn get_local_center_of_mass(&self) -> Vec3 { self.center }

	fn get_mass(&self) -> f32 { self.mass }

	fn get_moment_of_inertia_tensor(&self) -> Mat3 {
		let inertia = 2.0 / 5.0 * self.mass * self.radius;
		Mat3::from_diagonal(&Vec3::new(inertia, inertia, inertia))
	}

	fn get_restitution_coefficient(&self) -> f32 { self.restitution_coefficient }

	fn get_friction_threshold(&self) -> f32 { self.friction_threshold }

	fn get_static_friction_coefficient(&self) -> f32 { self.static_friction_coefficient }

	fn get_dynamic_friction_coefficient(&self) -> f32 { self.dynamic_friction_coefficient }
}

/// A copy of all of the publicly-accessible properties of a spherical collider.
#[derive(Debug)]
pub struct SphereCollider {
	/// The entity, if there is one. This is NOT copied back into InternalSphereCollider, hence why it's not "pub".
	///
	/// Defaults to None.
	entity : Option<EntityHandle>,

	/// The position of the center relative to the parent entity's origin (in the parent entity's local space).
	///
	/// Defaults to origin.
	pub center : Vec3,

	/// The radius.
	///
	/// Has no default.
	pub radius : f32,

	/// The total mass.
	///
	/// Defaults to zero.
	pub mass : f32,

	/// The restituion coefficient.
	///
	/// Defaults to one.
	pub restitution_coefficient : f32,

	/// The ratio used to threshold whether to use static or dynamic friction for a given collision.
	///
	/// Defaults to `0.25`.
	pub friction_threshold : f32,

	/// The static friction coefficient. Should always at or between 0.0 and 1.0.
	///
	/// Defaults to `1.0`.
	pub static_friction_coefficient : f32,

	/// The dynamic friction coefficient. Should always at or between 0.0 and 1.0.
	///
	/// Defaults to `0.3`.
	pub dynamic_friction_coefficient : f32,
}

impl SphereCollider {
	/// Creates an instance with all values at default.
	pub fn new(radius : f32) -> SphereCollider {
		SphereCollider {
			entity: None,
			center: Vec3::zeros(),
			radius,
			mass: 0.0,
			restitution_coefficient: 1.0,
			friction_threshold: 0.25,
			static_friction_coefficient: 1.0,
			dynamic_friction_coefficient: 0.3,
		}
	}

	/// If this is in a valid state.
	pub fn is_valid(&self) -> bool {
		MINIMUM_RADIUS < self.radius && 0.0 <= self.mass
	}
}

impl Collider for SphereCollider {
	fn get_type(&self) -> ColliderType { ColliderType::SPHERE }

	fn get_entity(&self) -> Option<EntityHandle> { self.entity }

	fn get_center_of_mass(&self) -> Vec3 { self.center }
}
