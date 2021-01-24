use crate::types::{Vec3, Mat3, EntityHandle};
use crate::collider::{ColliderType, Collider, InternalCollider};

/// An amount of radius that allows objects to just be "in contact" without being considered far enough in to force them to be pushed out.
const CONTACT_MARGIN : f32 = 0.05;

/// The internal representation of a sphere collider.
#[derive(Debug)]
pub struct InternalSphereCollider {
	/// The entity that this is linked to (if any).
	entity : Option<EntityHandle>,
	/// The position of the center relative to the parent entity's origin.
	pub center : Vec3,
	/// The radius.
	pub radius : f32,
	/// The total mass. Must not be negative.
	pub mass : f32,
	/// The restituion coefficient.
	pub restitution_coefficient : f32,
}

impl InternalSphereCollider {
	/// Creates a new instance.
	pub fn new(offset : &Vec3, radius : f32, mass : f32, restitution_coefficient : f32) -> Result<Box<dyn InternalCollider>, ()> {
		if CONTACT_MARGIN >= radius || 0.0 > mass {
			Err(()) // TODO: An error type.
		} else {
			Ok(Box::new(InternalSphereCollider {
				entity: None,
				center: offset.clone(),
				radius,
				mass,
				restitution_coefficient,
			}))
		}
	}

	/// Creates from an InternalEntity.
	pub fn from(source : &SphereCollider) -> Result<Box<dyn InternalCollider>, ()> {
		InternalSphereCollider::new(
			&source.center,
			source.radius,
			source.mass,
			source.restitution_coefficient,
		)
	}

	/// Makes a SphereCollider copying this instance's values.
	pub fn make_pub(&self) -> SphereCollider {
		SphereCollider::from(self)
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : &SphereCollider) -> Result<(),()> {
		if CONTACT_MARGIN >= source.radius || 0.0 > source.mass {
			Err(()) // TODO: An error type.
		} else {
			self.center = source.center;
			self.radius = source.radius;
			self.mass = source.mass;
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

	/// Gets the mass of this collider. Must not be negative.
	fn get_mass(&self) -> f32 { self.mass }

	/// Gets the moment of inertia tensor about the center of mass.
	fn get_moment_of_inertia_tensor(&self) -> Mat3 {
		let inertia = 2.0 / 5.0 * self.mass * self.radius;
		Mat3::from_diagonal(&Vec3::new(inertia, inertia, inertia))
	}

	fn get_restitution_coefficient(&self) -> f32 {
		self.restitution_coefficient
	}
}

/// A copy of all of the publicly-accessible properties of a spherical collider.
#[derive(Debug)]
pub struct SphereCollider {
	/// The entity, if there is one.
	entity : Option<EntityHandle>,
	/// The position of the center relative to the parent entity's origin.
	pub center : Vec3,
	/// The radius.
	pub radius : f32,
	/// The total mass.
	pub mass : f32,
	/// The restituion coefficient.
	pub restitution_coefficient : f32,
}

impl SphereCollider {
	/// Creates an instance.
	pub fn new(center : &Vec3, radius : f32, mass : f32, restitution_coefficient : f32) -> SphereCollider {
		SphereCollider { entity: None, center: center.clone(), radius, mass, restitution_coefficient, }
	}

	/// Creates from an InternalSphereCollider.
	pub fn from(source : &InternalSphereCollider) -> SphereCollider {
		SphereCollider {
			entity: source.entity.clone(),
			center: source.center.clone(),
			radius: source.radius,
			mass: source.mass,
			restitution_coefficient: source.restitution_coefficient,
		}
	}
}

impl Collider for SphereCollider {
	fn get_type(&self) -> ColliderType { ColliderType::SPHERE }

	fn get_entity(&self) -> Option<EntityHandle> { self.entity }

	fn get_center_of_mass(&self) -> Vec3 { self.center }
}
