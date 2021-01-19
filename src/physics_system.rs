use generational_arena::Arena;

use crate::types::{Vec3, EntityHandle};
use crate::entity::{InternalEntity, Entity};

/// The entire physics system.
pub struct PhysicsSystem {
	/// All the items.
	entities : Arena<InternalEntity>,
}

impl PhysicsSystem {
	/// Creates a new instance.
	pub fn new() -> PhysicsSystem {
		PhysicsSystem {
			entities: Arena::new(),
		}
	}

	/// Adds an entity and returns its handle.
	pub fn add_entity(&mut self, position : &Vec3) -> EntityHandle {
		self.entities.insert(InternalEntity::new(position))
	}

	/// Removes an entity.
	pub fn remove_entity(&mut self, handle : EntityHandle) {
		self.entities.remove(handle);
	}

	/// Gets an entity's public interface.
	/// These values are all copies of the internal entity.
	pub fn get_entity(&self, handle : EntityHandle) -> Option<Entity> {
		self.entities.get(handle).and_then(|internal| Some(Entity::from(internal)))
	}

	/// Updates an entity with the given values.
	pub fn update_entity(&mut self, handle : EntityHandle, source : Entity) -> Result<(),()> {
		self.entities.get_mut(handle).ok_or(()).and_then(|internal| internal.update_from(source))
	}

	/// Moves the system forward by the given step.
	pub fn update(&mut self, dt : f32) {
		for (_handle, entity) in &mut self.entities {
			let acceleration = Vec3::zeros(); // TODO: Calculate acceleration.
			entity.velocity += acceleration.scale(dt);
			entity.position += entity.velocity.scale(dt);
		}
	} 
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_update() {
		let mut system = PhysicsSystem::new();
		// Check nothing breaks with no items.
		system.update(1.0);
		let first = system.add_entity(&Vec3::new(1.0, 2.0, 3.0));
		{
			let mut interface = system.get_entity(first).unwrap();
			assert_eq!(interface.position.x, 1.0);
			assert_eq!(interface.position.y, 2.0);
			assert_eq!(interface.position.z, 3.0);
			assert_eq!(interface.velocity.norm(), 0.0);
			interface.velocity.x = 1.0;
			system.update_entity(first, interface).unwrap();
		}
		{
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.velocity.x, 1.0);
			assert_eq!(interface.velocity.y, 0.0);
			assert_eq!(interface.velocity.z, 0.0);
		}
		system.update(1.0);
		{
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.position.x, 2.0);
			assert_eq!(interface.position.y, 2.0);
			assert_eq!(interface.position.z, 3.0);

			assert_eq!(interface.velocity.x, 1.0);
			assert_eq!(interface.velocity.y, 0.0);
			assert_eq!(interface.velocity.z, 0.0);
		}
		system.remove_entity(first);
		{
			let interface = system.get_entity(first);
			assert!(interface.is_none());
		}
	}
}
