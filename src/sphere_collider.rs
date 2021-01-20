use crate::types::{Vec3, EntityHandle};
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
}

impl InternalSphereCollider {
	/// Creates a new instance.
	pub fn new(offset : &Vec3, radius : f32) -> Result<Box<dyn InternalCollider>, ()> {
		if CONTACT_MARGIN >= radius {
			Err(()) // TODO: An error type.
		} else {
			Ok(Box::new(InternalSphereCollider {
				entity: None,
				center: offset.clone(),
				radius,
			}))
		}
	}

	/// Creates from an InternalEntity.
	pub fn from(source : &SphereCollider) -> Result<Box<dyn InternalCollider>, ()> {
		InternalSphereCollider::new(
			&source.center,
			source.radius,
		)
	}

	/// Makes a SphereCollider copying this instance's values.
	pub fn make_pub(&self) -> SphereCollider {
		SphereCollider::from(self)
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : &SphereCollider) -> Result<(),()> {
		if CONTACT_MARGIN >= source.radius {
			Err(()) // TODO: An error type.
		} else {
			self.center = source.center;
			self.radius = source.radius;
			Ok(())
		}
	}
}

impl InternalCollider for InternalSphereCollider {
	/// The specific type.
	fn get_type(&self) -> ColliderType { ColliderType::SPHERE }

	/// Sets the entity this is attached to, returning the previous one.
	fn set_entity_handle(&mut self, handle : Option<EntityHandle>) -> Option<EntityHandle> {
		let old = self.entity;
		self.entity = handle;
		old
	}

	/// Retrieves the stored entity handle that this is attached to.
	fn get_entity_handle(&mut self) -> Option<EntityHandle> {
		self.entity
	}
}

/// The public face of a sphere collider.
/// This is what users will interact with.
#[derive(Debug)]
pub struct SphereCollider {
	/// The entity, if there is one.
	entity : Option<EntityHandle>,
	/// The position of the center relative to the parent entity's origin.
	pub center : Vec3,
	/// The radius.
	pub radius : f32,
}

impl SphereCollider {
	/// Creates an instance.
	pub fn new(center : &Vec3, radius : f32) -> SphereCollider {
		SphereCollider { entity: None, center: center.clone(), radius }
	}

	/// Creates from an InternalSphereCollider.
	pub fn from(source : &InternalSphereCollider) -> SphereCollider {
		SphereCollider {
			entity: source.entity.clone(),
			center: source.center.clone(),
			radius: source.radius,
		}
	}
}

impl Collider for SphereCollider {
	fn get_type(&self) -> ColliderType { ColliderType::SPHERE }

	fn get_entity_handle(&self) -> Option<EntityHandle> { self.entity }
}
