use std::collections::HashSet;

use crate::types::{Vec3, ColliderHandle};

/// The internal representation of any physical object.
/// This generally has NO data hiding to keep things simple.
pub struct InternalEntity {
	/// The current 3D position.
	pub position : Vec3,
	/// The current 3D velocity.
	pub velocity : Vec3,
	/// All colliders that are attached/linked to this.
	pub colliders : HashSet<ColliderHandle>,
}

impl InternalEntity {
	/// Creates a new instance.
	pub fn new(position : &Vec3) -> InternalEntity {
		InternalEntity {
			position: position.clone(),
			velocity: Vec3::zeros(),
			colliders: HashSet::new(),
		}
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : Entity) -> Result<(),()> {
		self.position = source.position;
		self.velocity = source.velocity;
		Ok(())
	}
}

/// The public face of any physical object.
/// This is what users will interact with.
#[derive(Debug)]
pub struct Entity {
	/// The current 3D position.
	pub position : Vec3,
	/// The current 3D velocity.
	pub velocity : Vec3,
	/// All colliders that are attached/linked to this.
	colliders : HashSet<ColliderHandle>,
}

impl Entity {
	/// Creates from an InternalEntity.
	pub fn from(source : &InternalEntity) -> Entity {
		Entity {
			position: source.position.clone(),
			velocity: source.velocity.clone(),
			colliders: source.colliders.clone(),
		}
	}

	/// Gets all collider handles.
	/// Notibly this is just the getter, as this object cannot be used to modify what colliders are attached to this entity.
	pub fn get_colliders(&self) -> HashSet<ColliderHandle> {
		self.colliders.clone()
	}
}
