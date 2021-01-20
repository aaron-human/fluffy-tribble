use generational_arena::Arena;

use crate::types::{Vec3, EntityHandle, ColliderHandle};
use crate::entity::{InternalEntity, Entity};
use crate::collider::{ColliderType, InternalCollider, Collider};
use crate::sphere_collider::{InternalSphereCollider, SphereCollider};
use crate::collider_wrapper::ColliderWrapper;

/// The entire physics system.
pub struct PhysicsSystem {
	/// All the whole physical objects.
	entities : Arena<InternalEntity>,
	/// All of the colliders on the physical objects.
	colliders : Arena<Box<dyn InternalCollider>>,
}

impl PhysicsSystem {
	/// Creates a new instance.
	pub fn new() -> PhysicsSystem {
		PhysicsSystem {
			entities: Arena::new(),
			colliders : Arena::new(),
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

	/// Adds a collider to a given entity.
	pub fn add_collider(&mut self, source : ColliderWrapper) -> Result<ColliderHandle, ()> {
		match source {
			ColliderWrapper::Sphere(sphere) => {
				match InternalSphereCollider::from(&sphere) {
					Ok(internal) => {
						Ok(self.colliders.insert(internal))
					},
					Err(a) => Err(a)
				}
			}
		}
	}

	/// Removes a collider.
	pub fn remove_collider(&mut self, handle : ColliderHandle) {
		self.colliders.remove(handle);
	}

	/// Gets the collider's public interface.
	/// These values are all copies of the internal collider.
	pub fn get_collider(&self, handle : ColliderHandle) -> Option<ColliderWrapper> {
		if let Some(collider) = self.colliders.get(handle) {
			match collider.get_type() {
				ColliderType::SPHERE => {
					Some(ColliderWrapper::Sphere(collider.downcast_ref::<InternalSphereCollider>().unwrap().make_pub()))
				}
			}
		} else { None }
	}

	/// Gets the collider's public interface.
	/// These values are all copies of the internal collider.
	pub fn update_collider(&mut self, handle : ColliderHandle, source : ColliderWrapper) -> Result<(), ()> {
		match source {
			ColliderWrapper::Sphere(typed_source) => {
				if let Some(typed_dest) = self.colliders.get_mut(handle).and_then(|boxed| boxed.downcast_mut::<InternalSphereCollider>()) {
					typed_dest.update_from(&typed_source)
				} else { Err(()) }
			}
		}
	}

	/*// Links the collider to the entity.
	/// Will unlink it from any existing entity.
	pub fn link_collider(&mut self, collider_handle : ColliderHandle, entity_handle : EntityHandle) -> Result<(), ()> {
		//
	}*/

	/// Moves the system forward by the given time step.
	pub fn step(&mut self, dt : f32) {
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
		system.step(1.0);
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
		system.step(1.0);
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

	#[test]
	fn store_collider() {
		let mut system = PhysicsSystem::new();
		let id = system.add_collider(ColliderWrapper::Sphere(SphereCollider::new(
			&Vec3::new(0.0, 0.0, 1.0),
			2.0,
		))).unwrap();
		if let ColliderWrapper::Sphere(mut interface) = system.get_collider(id).unwrap() {
			assert_eq!(interface.center.x, 0.0);
			assert_eq!(interface.center.y, 0.0);
			assert_eq!(interface.center.z, 1.0);
			assert_eq!(interface.radius, 2.0);
			assert_eq!(interface.get_entity_handle(), None);
			interface.center.x = 5.0;
			system.update_collider(id, ColliderWrapper::Sphere(interface)).unwrap();
		} else {
			panic!("The collider didn't unwrap into the right type!");
		}
		if let ColliderWrapper::Sphere(mut interface) = system.get_collider(id).unwrap() {
			assert_eq!(interface.center.x, 5.0);
			assert_eq!(interface.center.y, 0.0);
			assert_eq!(interface.center.z, 1.0);
			assert_eq!(interface.radius, 2.0);
		} else {
			panic!("The collider didn't unwrap into the right type!");
		}
		system.remove_collider(id);
		{
			let interface = system.get_collider(id);
			assert!(interface.is_none());
		}
	}
}
