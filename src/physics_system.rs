use generational_arena::Arena;

use crate::types::{Vec3, EntityHandle, ColliderHandle};
use crate::entity::{InternalEntity, Entity};
use crate::collider::{ColliderType, InternalCollider, Collider};
use crate::sphere_collider::{InternalSphereCollider, SphereCollider};
use crate::collider_wrapper::ColliderWrapper;
use crate::collision::collide;

/// The entire physics system.
pub struct PhysicsSystem {
	/// All the whole physical objects.
	entities : Arena<InternalEntity>,
	/// All of the colliders on the physical objects.
	colliders : Arena<Box<dyn InternalCollider>>,
}

struct EntityStepInfo {
	/// The entity handle.
	handle : EntityHandle,
	/// The planned motion for the entity.
	movement : Vec3,
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

	/// Removes an entity and all of it's associated colliders.
	/// Returns if anything changed (i.e. if the entity existed and was removed).
	pub fn remove_entity(&mut self, handle : EntityHandle) -> bool {
		if let Some(entity) = self.entities.remove(handle) {
			// Also remove all associated colliders.
			for collider in entity.colliders {
				self.remove_collider(collider);
			}
			true
		} else { false }
	}

	/// Gets an entity's public interface.
	/// These values are all copies of the internal entity.
	pub fn get_entity(&self, handle : EntityHandle) -> Option<Entity> {
		self.entities.get(handle).and_then(|internal| Some(Entity::from(internal)))
	}

	/// Updates an entity with the given values.
	/// This does NOT update the list of linked/attached colliders. Must use link_collider() for that.
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
	/// This does NOT update the list of linked/attached colliders. Must use link_collider() for that.
	pub fn update_collider(&mut self, handle : ColliderHandle, source : ColliderWrapper) -> Result<(), ()> {
		match source {
			ColliderWrapper::Sphere(typed_source) => {
				if let Some(typed_dest) = self.colliders.get_mut(handle).and_then(|boxed| boxed.downcast_mut::<InternalSphereCollider>()) {
					typed_dest.update_from(&typed_source)
				} else { Err(()) }
			}
		}
	}

	/// Links the collider to the entity.
	/// Will unlink it from any existing entity.
	pub fn link_collider(&mut self, collider_handle : ColliderHandle, entity_handle : Option<EntityHandle>) -> Result<(), ()> {
		// ;Start by getting the collider. Nothing happens without it existing.
		if let Some(collider_box) = self.colliders.get_mut(collider_handle) {
			// Then try to handle the passed in entity_handle, which can be None...
			// This part is mainly done before anything else so won't touch the collider unless entity_handle is valid.
			if let Some(handle) = entity_handle.clone() {
				if let Some(entity) = self.entities.get_mut(handle) {
					entity.colliders.insert(collider_handle);
				} else { return Err(()); }
			}
			// Then switch out the value in the collider.
			if let Some(prior_entity_handle) = collider_box.as_mut().set_entity(entity_handle) {
				// Try to get the old entity and remove this collider from it.
				// Only do this if the entity changed.
				if Some(prior_entity_handle) != entity_handle {
					if let Some(prior_entity) = self.entities.get_mut(prior_entity_handle) {
						prior_entity.colliders.remove(&collider_handle);
					}
					// Ignore if the entity no longer exists (shouldn't happen, but also there's really no reason to complain if it does).
				}
			}
			Ok(())
		} else { Err(()) }
	}

	/// Moves the system forward by the given time step.
	pub fn step(&mut self, dt : f32) {
		// Go through all entities and perform integration.
		let mut entity_info = Vec::with_capacity(self.entities.len());
		for (handle, entity) in self.entities.iter_mut() { // TODO: Optimize this.
			let acceleration = Vec3::zeros(); // TODO: Calculate acceleration.
			entity.velocity += acceleration.scale(dt);
			entity_info.push(EntityStepInfo {
				handle,
				movement: entity.velocity.scale(dt),
			});
		}

		// TODO: Setup a broad-phase that checks AABBs.

		// Go through every unique pair of handles and deal with collisions.
		for first_handle_index in 0..entity_info.len() {
			// Get both entity_info elements.
			let (lower_entity_infos, upper_entity_infos) = entity_info.split_at_mut(first_handle_index+1);
			let first_entity_info = &mut lower_entity_infos[first_handle_index];
			for second_entity_info in upper_entity_infos {
				let (first_option, second_option) = self.entities.get2_mut(first_entity_info.handle, second_entity_info.handle);
				let first = first_option.unwrap();
				let second = second_option.unwrap();
				//
				for first_collider_handle in first.colliders.iter() {
					for second_collider_handle in second.colliders.iter() {
						let first_collider_box  = self.colliders.get(*first_collider_handle ).unwrap();
						let second_collider_box = self.colliders.get(*second_collider_handle).unwrap();
						let collision_option = collide(
							first_collider_box,
							&first.position,
							&first_entity_info.movement,
							second_collider_box,
							&second.position,
							&second_entity_info.movement,
						);
						if let Some(collision) = collision_option {
							//
						}
					}
				}
			}
		}

		// Once all the physics has been handled, apply the movement.
		for info in entity_info {
			let first = self.entities.get_mut(info.handle).unwrap();
			first.position += info.movement;
		}
	} 
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Verify can create/store/remove entities.
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

	/// Verify can create/store/remove colliders.
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
			assert_eq!(interface.get_entity(), None);
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

	/// Verify can link colliders to entities.
	#[test]
	fn link_collider() {
		let mut system = PhysicsSystem::new();
		let first = system.add_entity(&Vec3::zeros());
		let collider = system.add_collider(ColliderWrapper::Sphere(SphereCollider::new(
			&Vec3::new(0.0, 0.0, 1.0),
			2.0,
		))).unwrap();
		{ // Entities start with no colliders. And colliders start with no entities.
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.get_colliders().len(), 0);
			if let ColliderWrapper::Sphere(interface) = system.get_collider(collider).unwrap() {
				assert_eq!(interface.get_entity(), None);
			} else { panic!("Didn't get a sphere?"); }
		}
		system.link_collider(collider, Some(first)).unwrap();
		{ // Can add and things work right.
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.get_colliders().len(), 1);
			assert!(interface.get_colliders().contains(&collider));
			if let ColliderWrapper::Sphere(interface) = system.get_collider(collider).unwrap() {
				assert_eq!(interface.get_entity(), Some(first));
			} else { panic!("Didn't get a sphere?"); }
		}
		let second = system.add_entity(&Vec3::zeros());
		system.link_collider(collider, Some(second)).unwrap();
		{ // Can transfer collider easily.
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.get_colliders().len(), 0);
			let interface = system.get_entity(second).unwrap();
			assert_eq!(interface.get_colliders().len(), 1);
			assert!(interface.get_colliders().contains(&collider));
			if let ColliderWrapper::Sphere(interface) = system.get_collider(collider).unwrap() {
				assert_eq!(interface.get_entity(), Some(second));
			} else { panic!("Didn't get a sphere?"); }
		}
		{ // Verify can't add a collider to a missing entity.
			let temp = system.add_entity(&Vec3::zeros());
			system.remove_entity(temp);
			assert_eq!(system.link_collider(collider, Some(temp)), Err(()));
			// That shouldn't have changed anything.
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.get_colliders().len(), 0);
			let interface = system.get_entity(second).unwrap();
			assert_eq!(interface.get_colliders().len(), 1);
			assert!(interface.get_colliders().contains(&collider));
			if let ColliderWrapper::Sphere(interface) = system.get_collider(collider).unwrap() {
				assert_eq!(interface.get_entity(), Some(second));
			} else { panic!("Didn't get a sphere?"); }
		}
		{ // Verify can't add a missing collier to an entity.
			let temp = system.add_collider(ColliderWrapper::Sphere(SphereCollider::new(
				&Vec3::new(0.0, 0.0, 1.0),
				2.0,
			))).unwrap();
			system.remove_collider(temp);
			assert_eq!(system.link_collider(temp, Some(second)), Err(()));
			// That shouldn't have changed anything.
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.get_colliders().len(), 0);
			let interface = system.get_entity(second).unwrap();
			assert_eq!(interface.get_colliders().len(), 1);
			assert!(interface.get_colliders().contains(&collider));
			if let ColliderWrapper::Sphere(interface) = system.get_collider(collider).unwrap() {
				assert_eq!(interface.get_entity(), Some(second));
			} else { panic!("Didn't get a sphere?"); }
		}
		system.link_collider(collider, Some(second)).unwrap();
		{ // Verify can "transfer" to current entity.
			// That shouldn't have changed anything.
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.get_colliders().len(), 0);
			let interface = system.get_entity(second).unwrap();
			assert_eq!(interface.get_colliders().len(), 1);
			assert!(interface.get_colliders().contains(&collider));
			if let ColliderWrapper::Sphere(interface) = system.get_collider(collider).unwrap() {
				assert_eq!(interface.get_entity(), Some(second));
			} else { panic!("Didn't get a sphere?"); }
		}
		system.link_collider(collider, None).unwrap();
		{ // Can transfer collider to being unowned easily.
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.get_colliders().len(), 0);
			let interface = system.get_entity(second).unwrap();
			assert_eq!(interface.get_colliders().len(), 0);
			if let ColliderWrapper::Sphere(interface) = system.get_collider(collider).unwrap() {
				assert_eq!(interface.get_entity(), None);
			} else { panic!("Didn't get a sphere?"); }
		}
		system.link_collider(collider, Some(second)).unwrap();
		system.remove_entity(second);
		{ // Removing the entity should also remove the collider.
			let interface = system.get_entity(first).unwrap();
			assert_eq!(interface.get_colliders().len(), 0);
			assert!(system.get_collider(collider).is_none());
		}
	}
}
