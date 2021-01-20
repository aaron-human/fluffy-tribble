use crate::types::Vec3;
use crate::collider::{Collider, InternalCollider};

/// An amount of radius that allows objects to just be "in contact" without being considered far enough in to force them to be pushed out.
const CONTACT_MARGIN : f32 = 0.05;

/// The internal representation of a sphere collider.
#[derive(Debug)]
pub struct InternalSphereCollider {
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
				center: offset.clone(),
				radius,
			}))
		}
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : SphereCollider) -> Result<(),()> {
		self.center = source.center;
		self.radius = source.radius;
		Ok(())
	}
}

impl InternalCollider for InternalSphereCollider { }

/// The public face of any physical object.
/// This is what users will interact with.
#[derive(Debug)]
pub struct SphereCollider {
	/// The position of the center relative to the parent entity's origin.
	pub center : Vec3,
	/// The radius.
	pub radius : f32,
}

impl SphereCollider {
	/// Creates from an InternalEntity.
	pub fn from(source : &InternalSphereCollider) -> SphereCollider {
		SphereCollider {
			center: source.center.clone(),
			radius: source.radius,
		}
	}
}

impl Collider for SphereCollider { }
