use std::cell::RefCell;
use std::borrow::BorrowMut;

use generational_arena::Arena;

use crate::consts::EPSILON;
use crate::types::{Vec3, EntityHandle, ColliderHandle, UnaryForceGeneratorHandle};
use crate::entity::{InternalEntity, Entity};
use crate::collider::{ColliderType, InternalCollider};
#[allow(unused_imports)] // Need this trait, but Rust's warning system doesn't seem to understand that.
use crate::collider::Collider;
use crate::null_collider::{InternalNullCollider};
use crate::sphere_collider::{InternalSphereCollider};
use crate::plane_collider::{InternalPlaneCollider};
use crate::collider_wrapper::ColliderWrapper;
use crate::collision::{collide, Collision};

use crate::unary_force_generator::UnaryForceGenerator;

/// The entire physics system.
pub struct PhysicsSystem {
	/// All the whole physical objects.
	entities : RefCell<Arena<InternalEntity>>,
	/// All of the colliders on the physical objects.
	colliders : RefCell<Arena<Box<dyn InternalCollider>>>,
	/// All of the unary forces to apply.
	unary_force_generators : RefCell<Arena<Box<dyn UnaryForceGenerator>>>,
	/// The max number of physics iterations allowed per step.
	///
	/// For now this limits how many collisions can be handled in a step.
	pub iteration_max : u8,

	/// A debugging value to get info out.
	pub debug : Vec<String>,
}

#[derive(Debug)]
struct EntityStepInfo {
	/// The entity handle.
	handle : EntityHandle,
	/// The planned linear motion for the entity.
	linear_movement : Vec3,
	/// The planned angular motion for the entity.
	angular_movement : Vec3,
}

impl PhysicsSystem {
	/// Creates a new instance.
	pub fn new() -> PhysicsSystem {
		PhysicsSystem {
			entities: RefCell::new(Arena::new()),
			colliders : RefCell::new(Arena::new()),
			unary_force_generators : RefCell::new(Arena::new()),
			iteration_max : 5,

			debug: Vec::new(),
		}
	}

	/// Adds an entity and returns its handle.
	pub fn add_entity(&mut self, source : Entity) -> Result<EntityHandle, ()> {
		let new_entity = InternalEntity::new_from(source)?;
		Ok(self.entities.borrow_mut().insert(new_entity))
	}

	/// Removes an entity and all of it's associated colliders.
	///
	/// Returns if anything changed (i.e. if the entity existed and was removed).
	pub fn remove_entity(&mut self, handle : EntityHandle) -> bool {
		let removed = self.entities.borrow_mut().remove(handle);
		if let Some(entity) = removed {
			// Also remove all associated colliders.
			for collider in entity.colliders {
				self.remove_collider(collider);
			}
			true
		} else { false }
	}

	/// Gets an entity's public interface.
	///
	/// These values are all copies of the internal entity.
	pub fn get_entity(&self, handle : EntityHandle) -> Option<Entity> {
		self.entities.borrow().get(handle).and_then(|internal| Some(internal.make_pub()))
	}

	/// Updates an entity with the given values.
	///
	/// This does NOT update the list of linked/attached colliders. Must use link_collider() for that.
	pub fn update_entity(&mut self, handle : EntityHandle, source : Entity) -> Result<(),()> {
		self.entities.borrow_mut().get_mut(handle).ok_or(()).and_then(|internal| {
			if let Ok(_) = internal.update_from(source) {
				internal.recalculate_mass(&*self.colliders.borrow());
				Ok(())
			} else { Err(()) }
		})
	}

	/// Adds a collider to the system.
	pub fn add_collider(&mut self, source : ColliderWrapper) -> Result<ColliderHandle, ()> {
		match source {
			ColliderWrapper::Null(source) => {
				match InternalNullCollider::new_from(&source) {
					Ok(internal) => {
						Ok(self.colliders.borrow_mut().insert(internal))
					},
					Err(a) => Err(a)
				}
			}
			ColliderWrapper::Sphere(source) => {
				match InternalSphereCollider::new_from(&source) {
					Ok(internal) => {
						Ok(self.colliders.borrow_mut().insert(internal))
					},
					Err(a) => Err(a)
				}
			}
			ColliderWrapper::Plane(source) => {
				match InternalPlaneCollider::new_from(&source) {
					Ok(internal) => {
						Ok(self.colliders.borrow_mut().insert(internal))
					},
					Err(a) => Err(a)
				}
			}
		}
	}

	/// Removes a collider.
	pub fn remove_collider(&mut self, handle : ColliderHandle) {
		if let Some(mut remainder) = self.colliders.borrow_mut().remove(handle) {
			// Force the associated entity to update (if there is one).
			if let Some(entity_handle) = remainder.get_entity() {
				if let Some(entity) = self.entities.borrow_mut().get_mut(entity_handle) {
					entity.recalculate_mass(&*self.colliders.borrow());
				}
			}
		}
	}

	/// Gets the collider's public interface.
	///
	/// These values are all copies of the internal collider.
	pub fn get_collider(&self, handle : ColliderHandle) -> Option<ColliderWrapper> {
		if let Some(collider) = self.colliders.borrow().get(handle) {
			match collider.get_type() {
				ColliderType::NULL => {
					Some(ColliderWrapper::Null(collider.downcast_ref::<InternalNullCollider>().unwrap().make_pub()))
				}
				ColliderType::SPHERE => {
					Some(ColliderWrapper::Sphere(collider.downcast_ref::<InternalSphereCollider>().unwrap().make_pub()))
				}
				ColliderType::PLANE => {
					Some(ColliderWrapper::Plane(collider.downcast_ref::<InternalPlaneCollider>().unwrap().make_pub()))
				}
			}
		} else { None }
	}

	/// Gets the collider's public interface.
	///
	/// These values are all copies of the internal collider.
	///
	/// This does NOT update the list of linked/attached colliders. Must use link_collider() for that.
	pub fn update_collider(&mut self, handle : ColliderHandle, source : ColliderWrapper) -> Result<(), ()> {
		let mut colliders = self.colliders.borrow_mut();
		let collider;
		if let Some(collider_) = colliders.get_mut(handle) {
			collider = collider_;
		} else {
			return Err(());
		}
		let entity_handle_option = collider.get_entity();
		let result = match source {
			ColliderWrapper::Null(typed_source) => {
				if let Some(typed_dest) = collider.downcast_mut::<InternalNullCollider>() {
					typed_dest.update_from(&typed_source)
				} else {
					return Err(());
				}
			}
			ColliderWrapper::Sphere(typed_source) => {
				if let Some(typed_dest) = collider.downcast_mut::<InternalSphereCollider>() {
					typed_dest.update_from(&typed_source)
				} else {
					return Err(());
				}
			}
			ColliderWrapper::Plane(typed_source) => {
				if let Some(typed_dest) = collider.downcast_mut::<InternalPlaneCollider>() {
					typed_dest.update_from(&typed_source)
				} else {
					return Err(());
				}
			}
		};
		// Then, because mass might've changed, try to update the associated entity (if it exists).
		if let Some(entity_handle) = entity_handle_option {
			if let Some(entity) = self.entities.borrow_mut().get_mut(entity_handle) {
				entity.recalculate_mass(&*colliders);
			}
		}
		result
	}

	/// Links the collider to the entity.
	///
	/// Will unlink it from any existing entity.
	pub fn link_collider(&mut self, collider_handle : ColliderHandle, entity_handle : Option<EntityHandle>) -> Result<(), ()> {
		// Start by verifying the collider exists. Nothing can happen without it.
		if !self.colliders.borrow().contains(collider_handle) {
			return Err(());
		}

		// Then try to handle the passed in entity_handle, which can be None...
		// This part is mainly done before anything else so won't touch the collider unless entity_handle is valid.
		if let Some(handle) = entity_handle.clone() {
			if let Some(entity) = self.entities.borrow_mut().get_mut(handle) {
				entity.colliders.insert(collider_handle);
				entity.recalculate_mass(&*self.colliders.borrow());
			} else { return Err(()); }
		}

		// Then get the collider.
		let prior_entity_handle_option;
		if let Some(collider_box) = self.colliders.borrow_mut().get_mut(collider_handle) {
			// Then switch out the value in the collider.
			prior_entity_handle_option = collider_box.as_mut().set_entity(entity_handle);
		} else {
			return Err(());
		}

		// Finally try to get the old entity and remove this collider from it.
		// Only do this if the entity changed.
		if prior_entity_handle_option != entity_handle {
			if let Some(prior_entity_handle) = prior_entity_handle_option {
				if let Some(prior_entity) = self.entities.borrow_mut().get_mut(prior_entity_handle) {
					prior_entity.colliders.borrow_mut().remove(&collider_handle);
					prior_entity.recalculate_mass(&*self.colliders.borrow());
				}
				// Ignore if the entity no longer exists (shouldn't happen, but also there's really no reason to complain if it does).
			}
		}

		Ok(())
	}

	/// Adds a UnaryForceGenerator to the system.
	pub fn add_unary_force_generator(&mut self, generator : Box<dyn UnaryForceGenerator>) -> Result<UnaryForceGeneratorHandle, ()> {
		Ok(self.unary_force_generators.borrow_mut().insert(generator))
	}

	/// Removes and returns a UnaryForceGenerator from the system.
	pub fn remove_unary_force_generator(&mut self, handle : UnaryForceGeneratorHandle) -> Option<Box<dyn UnaryForceGenerator>> {
		self.unary_force_generators.borrow_mut().remove(handle)
	}

	/// Moves the system forward by the given time step.
	///
	/// Note that a large `dt` will most likely lead to instability.
	///
	/// Also this isn't guaranteed to move everything forward by `dt`. It might move things forward less if it hits a computational limit.
	pub fn step(&mut self, dt : f32) {
		self.debug.clear();
		// Go through all entities and perform the initial integration.
		let mut entity_handles = Vec::with_capacity(self.entities.borrow().len());
		for (handle, _) in self.entities.borrow().iter() {
			entity_handles.push(handle);
		}
		let mut unary_force_generator_handles = Vec::with_capacity(self.unary_force_generators.borrow().len());
		for (handle, _) in self.unary_force_generators.borrow().iter() {
			unary_force_generator_handles.push(handle);
		}
		let mut entity_info = Vec::with_capacity(self.entities.borrow().len());
		for handle in entity_handles { // TODO: Optimize this.
			let mut acceleration = Vec3::zeros();
			let mut torque = Vec3::zeros();

			{
				let entity_copy = self.get_entity(handle).unwrap();
				// Since 0.0 * INFINITY becomes NaN, best to just acceleration and torque on infinite masses.
				if entity_copy.get_last_total_mass().is_finite() {
					for generator_handle in &unary_force_generator_handles {
						let mut generators_borrow = self.unary_force_generators.borrow_mut();
						let generator_borrow = generators_borrow.get_mut(*generator_handle).unwrap();
						let force = generator_borrow.make_force(dt, &self, handle);

						acceleration += force.force.scale(1.0 / entity_copy.get_last_total_mass());
						torque += entity_copy.get_last_moment_of_inertia() * (force.position - entity_copy.position).cross(&force.force);
					}
				}
			}

			let mut entities_borrow = self.entities.borrow_mut();
			let entity = entities_borrow.get_mut(handle).unwrap();
			entity.velocity += acceleration.scale(dt);
			let linear_movement = entity.velocity.scale(dt);

			entity.angular_velocity += entity.get_inverse_moment_of_inertia() * torque.scale(dt);
			let angular_movement = entity.angular_velocity.scale(dt);

			entity_info.push(EntityStepInfo { handle, linear_movement, angular_movement, });
		}

		// TODO: Setup a broad-phase that checks AABBs.
		// That should be able to split the world into islands of boxes that collide

		let mut time_left = dt;
		for _iteration in 0..self.iteration_max {
			// The simplest start is to find the closest collision, handle it, then move the simulation up to that point, and repeat looking for a collision.
			// Will be "done" once no collisions left or run out of iterations.

			// So start by going through every unique pair of handles and finding the first collision.
			let mut earliest_collision_percent = 1.0; // Collisions must happen before 100% of time_left.
			let mut earliest_collision = None;
			let mut earliest_collision_restitution = 1.0;
			let mut earliest_collision_first_entity_handle = None;
			let mut earliest_collision_second_entity_handle = None;
			let mut earliest_collision_first_info_index = 0;
			let mut earliest_collision_second_info_index = 0;
			// TODO: Someday optimize so it keeps track of collisions, and only calculates new collisions if one of the associated bodies has been modified by the last iteration.
			for first_index in 0..entity_info.len() {
				let (lower_entity_infos, upper_entity_infos) = entity_info.split_at_mut(first_index+1);
				let first_entity_info = &mut lower_entity_infos[first_index];
				for second_offset_index in 0..upper_entity_infos.len() {
					let second_index = first_index + second_offset_index + 1;
					let second_entity_info = &upper_entity_infos[second_offset_index];
					let mut entities = self.entities.borrow_mut();
					let (first_option, second_option) = entities.get2_mut(first_entity_info.handle, second_entity_info.handle);
					let first = first_option.unwrap();
					let second = second_option.unwrap();
					//
					for first_collider_handle in first.colliders.iter() {
						for second_collider_handle in second.colliders.iter() {
							let colliders = self.colliders.borrow();
							let first_collider_box  = colliders.get(*first_collider_handle ).unwrap();
							let second_collider_box = colliders.get(*second_collider_handle).unwrap();

							let first_start_orientation = first.orientation;
							let first_end_orientation = first.orientation.after_affected(
								&first_entity_info.linear_movement, &first_entity_info.angular_movement
							);

							let second_start_orientation = second.orientation;
							let second_end_orientation = second.orientation.after_affected(
								&second_entity_info.linear_movement, &second_entity_info.angular_movement
							);

							let collision_option = collide(
								first_collider_box,
								&first_start_orientation,
								&first_end_orientation,
								second_collider_box,
								&second_start_orientation,
								&second_end_orientation,
							);

							if let Some(collision) = collision_option {
								let time = collision.times.min();
								// If the objects are (already) moving away from the point of contact, then ignore the collision.
								let first_full_velocity = first.get_velocity_at_world_position(&collision.position);
								let second_full_velocity = second.get_velocity_at_world_position(&collision.position);
								let velocity_delta = first_full_velocity - second_full_velocity;

								if EPSILON > velocity_delta.dot(&collision.normal) {
									//self.debug.push(format!("Dropping collision at: {:?} between {:?} (velocity: {:?}) and {:?} (velocity: {:?})", collision.position, first_collider_handle, first_full_velocity, second_collider_handle, second_full_velocity));
									continue;
								}
								// Otherwise check if this collision is the closest.
								if time < earliest_collision_percent {
									earliest_collision_percent = time;
									earliest_collision = Some(collision);
									earliest_collision_restitution = first_collider_box.get_restitution_coefficient() *  second_collider_box.get_restitution_coefficient() ;
									earliest_collision_first_entity_handle = Some(first_entity_info.handle);
									earliest_collision_second_entity_handle = Some(second_entity_info.handle);
									earliest_collision_first_info_index = first_index;
									earliest_collision_second_info_index = second_index;
								}
							}
						}
					}
				}
			}

			// Re-adjust all of the movements to account for time stepping forward to just before (time_left * earliest_collision).
			let mut entities = self.entities.borrow_mut();
			let after_collision_percent = 1.0 - earliest_collision_percent;
			let time_after_collision = time_left * after_collision_percent;
			for info in &mut entity_info {
				// Always advance the actual entity forward by time (to keep all the movement values in lock-step).
				let entity = entities.get_mut(info.handle).unwrap();
				entity.orientation.affect_with(
					&(info.linear_movement  * earliest_collision_percent),
					&(info.angular_movement * earliest_collision_percent),
				);
				info.linear_movement *= after_collision_percent;
				info.angular_movement *= after_collision_percent;
			}
			time_left = time_after_collision;

			// Then respond to the collision.
			if let Some(collision) = earliest_collision {
				let first_entity_handle  = earliest_collision_first_entity_handle.unwrap();
				let second_entity_handle = earliest_collision_second_entity_handle.unwrap();

				let (first_option, second_option) = entities.get2_mut(first_entity_handle, second_entity_handle);
				let first  = first_option.unwrap();
				let second = second_option.unwrap();

				let impulse = PhysicsSystem::calc_collision_impulse(
					&first,
					&second,
					earliest_collision_restitution,
					&collision,
				);

				{
					// Apply the impluse and re-integrate the movement.
					let info = &mut entity_info[earliest_collision_first_info_index];

					first.apply_impulse(&collision.position, &impulse);

					info.linear_movement = first.velocity * time_after_collision;
					info.angular_movement = first.angular_velocity * time_after_collision;
				}
				{
					// Apply the impulse and re-integrate the movement.
					let info = &mut entity_info[earliest_collision_second_info_index];

					second.apply_impulse(&collision.position, &-impulse);

					info.linear_movement = second.velocity * time_after_collision;
					info.angular_movement = second.angular_velocity * time_after_collision;
				}
			} else {
				break; // No collision means done handling the entire step. So quit out of this loop.
			}
		}
	}

	/// Calculates the collision impulse between two entities.
	pub fn calc_collision_impulse(first : &InternalEntity, second : &InternalEntity, restitution_coefficient : f32, collision : &Collision) -> Vec3 {
		let first_offset  = collision.position - first.orientation.position;
		let second_offset = collision.position - second.orientation.position;

		let first_full_velocity  = first.get_velocity_at_world_position( &collision.position);
		let second_full_velocity = second.get_velocity_at_world_position(&collision.position);
		let velocity_delta = first_full_velocity - second_full_velocity;

		let numerator = -(1.0 + restitution_coefficient) * velocity_delta.dot(&collision.normal);
		let first_linear_weight   = 1.0 / first.get_total_mass();
		let second_linear_weight  = 1.0 / second.get_total_mass();
		let first_angular_amount = first.get_inverse_moment_of_inertia()   * first_offset.cross( &collision.normal);
		let first_angular_weight  = first_angular_amount.cross(&first_offset).dot( &collision.normal);
		let second_angular_amount = second.get_inverse_moment_of_inertia() * second_offset.cross(&collision.normal);
		let second_angular_weight = second_angular_amount.cross(&second_offset).dot(&collision.normal);
		let denominator = first_linear_weight + second_linear_weight + first_angular_weight + second_angular_weight;

		collision.normal.scale(numerator / denominator)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::f32::INFINITY;
	use crate::null_collider::NullCollider;
	use crate::sphere_collider::SphereCollider;
	use crate::plane_collider::PlaneCollider;
	use crate::gravity_generator::GravityGenerator;

	/// Verify can create/store/remove entities.
	#[test]
	fn basic_update() {
		let mut system = PhysicsSystem::new();
		// Check nothing breaks with no items.
		system.step(1.0);
		let first = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(1.0, 2.0, 3.0);
			system.add_entity(entity).unwrap()
		};
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
			assert_eq!(interface.position.x, 1.0);
			assert_eq!(interface.position.y, 2.0);
			assert_eq!(interface.position.z, 3.0);

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
		let id = {
			let mut sphere = SphereCollider::new(2.0);
			sphere.center = Vec3::new(0.0, 0.0, 1.0);
			system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
		};
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
		if let ColliderWrapper::Sphere(interface) = system.get_collider(id).unwrap() {
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
		let first = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(0.0, 0.0, 1.0);
			system.add_entity(entity).unwrap()
		};
		let collider = {
			let sphere = SphereCollider::new(2.0);
			system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
		};
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
		let second = {
			let entity = Entity::new();
			system.add_entity(entity).unwrap()
		};
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
			let temp = {
				let entity = Entity::new();
				system.add_entity(entity).unwrap()
			};
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
			let temp = {
				let sphere = SphereCollider::new(2.0);
				system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
			};
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

	/// Verify that attaching and removing colliders doesn't affect the origin of an entity's local space.
	#[test]
	fn entity_local_space_unchanged() {
		let mut system = PhysicsSystem::new();
		let entity = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(1.0, 2.0, 3.0);
			system.add_entity(entity).unwrap()
		};
		{
			let interface = system.get_entity(entity).unwrap();
			assert!((interface.position - Vec3::new(1.0, 2.0, 3.0)).magnitude() < EPSILON);
		}
		let collider1 = {
			let mut sphere = SphereCollider::new(1.0);
			sphere.center = Vec3::new(-1.0, 15.0, 8.0);
			sphere.mass = 1.0;
			system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
		};
		system.link_collider(collider1, Some(entity)).unwrap();
		{
			let interface = system.get_entity(entity).unwrap();
			let origin = interface.get_last_orientation().local_origin_in_world();
			println!("origin: {:?}", origin);
			assert!((origin - Vec3::new(1.0, 2.0, 3.0)).magnitude() < EPSILON);
		}
		system.link_collider(collider1, None).unwrap();
		{
			let interface = system.get_entity(entity).unwrap();
			let origin = interface.get_last_orientation().local_origin_in_world();
			assert!((origin - Vec3::new(1.0, 2.0, 3.0)).magnitude() < EPSILON);
		}
		let collider2 = {
			let mut sphere = SphereCollider::new(1.0);
			sphere.center = Vec3::new(8.0, -1.0, 5.0);
			sphere.mass = 1.0;
			system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
		};
		system.link_collider(collider2, Some(entity)).unwrap();
		system.link_collider(collider1, Some(entity)).unwrap();
		{
			let interface = system.get_entity(entity).unwrap();
			let origin = interface.get_last_orientation().local_origin_in_world();
			assert!((origin - Vec3::new(1.0, 2.0, 3.0)).magnitude() < EPSILON);
		}
	}

	/// Verify can create a NullCollider and it can move the center of mass.
	#[test]
	fn link_null_collider() {
		let mut system = PhysicsSystem::new();
		let entity = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(-1.0, -1.0, -1.0);
			system.add_entity(entity).unwrap()
		};
		{
			let interface = system.get_entity(entity).unwrap();
			assert!((interface.position - Vec3::new(-1.0, -1.0, -1.0)).magnitude() < EPSILON);
		}
		let collider = {
			let mut null = NullCollider::new();
			null.position = Vec3::new(2.0, 2.0, 2.0);
			null.mass = 1.0;
			system.add_collider(ColliderWrapper::Null(null)).unwrap()
		};
		system.link_collider(collider, Some(entity)).unwrap();
		{
			let interface = system.get_entity(entity).unwrap();
			assert!((interface.position - Vec3::new(1.0, 1.0, 1.0)).magnitude() < EPSILON);
		}
		system.step(1.0); // Make sure nothing panics with the collider.
		system.link_collider(collider, None).unwrap();
		{
			let interface = system.get_entity(entity).unwrap();
			assert!((interface.position - Vec3::new(-1.0, -1.0, -1.0)).magnitude() < EPSILON);
		}
	}

	/// Verify can create/link/update a PlaneCollider.
	#[test]
	fn basic_plane_collider() {
		let mut system = PhysicsSystem::new();
		let collider = {
			let plane = PlaneCollider::new();
			assert!(plane.is_valid());
			system.add_collider(ColliderWrapper::Plane(plane)).unwrap()
		};
		{
			let mut plane = PlaneCollider::new();
			plane.normal = Vec3::new(1.0, 0.0, 0.0);
			assert!(plane.is_valid());
			system.update_collider(collider, ColliderWrapper::Plane(plane)).unwrap()
		}
		{
			let mut plane = PlaneCollider::new();
			plane.normal = Vec3::new(1.0, 0.0, 0.0);
			if let ColliderWrapper::Plane(plane) = system.get_collider(collider).unwrap() {
				assert!((plane.normal - Vec3::new(1.0, 0.0, 0.0)).magnitude() < EPSILON);
			} else {
				panic!("Didn't get a plane!");
			}
		}
		let entity = {
			let entity = Entity::new();
			system.add_entity(entity).unwrap()
		};
		system.link_collider(collider, Some(entity)).unwrap();
		system.step(1.0); // Make sure nothing panics with the collider.
		system.link_collider(collider, None).unwrap();
	}

	/// Verify very basic billiard-ball example: two equal masses. One's at rest, the other hits it exactly head-on. All velocity should travel to the immobile one.
	#[test]
	fn equal_mass_billiard_balls() {
		let mut system = PhysicsSystem::new();
		let first = {
			let mut entity = Entity::new();
			entity.position = Vec3::zeros();
			entity.own_mass = 1.0;
			system.add_entity(entity).unwrap()
		};
		{
			let collider = {
				let mut sphere = SphereCollider::new(1.0);
				sphere.mass = 1.0;
				system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
			};
			system.link_collider(collider, Some(first)).unwrap();
		}
		{
			let mut temp = system.get_entity(first).unwrap();
			temp.velocity.x = 2.0;
			system.update_entity(first, temp).unwrap();
		}
		let second = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(3.0, 0.0, 0.0);
			entity.own_mass = 1.0;
			system.add_entity(entity).unwrap()
		};
		{
			let collider = {
				let mut sphere = SphereCollider::new(1.0);
				sphere.mass = 1.0;
				system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
			};
			system.link_collider(collider, Some(second)).unwrap();
		}
		system.step(1.0);
		{
			let temp = system.get_entity(first).unwrap();
			println!("First after: {:?}", temp);
			assert!((temp.velocity.x - 0.0).abs() < EPSILON);
			assert!((temp.position.x - 1.0).abs() < EPSILON);
		}
		{
			let temp = system.get_entity(second).unwrap();
			println!("Second after: {:?}", temp);
			assert!((temp.velocity.x - 2.0).abs() < EPSILON);
			assert!((temp.position.x - 4.0).abs() < EPSILON);
		}
	}

	/// Check that the entity mass updates at the right times (i.e. whenever it or anything that's linked to it changes).
	#[test]
	fn entity_auto_update() {
		let mut system = PhysicsSystem::new();
		let first = {
			let mut entity = Entity::new();
			entity.own_mass = 1.0;
			system.add_entity(entity).unwrap()
		};
		{
			let mut temp = system.get_entity(first).unwrap();
			temp.own_mass = 2.0;
			system.update_entity(first, temp).unwrap();
			// Verify the total mass changed.
			assert_eq!(system.get_entity(first).unwrap().get_last_total_mass(), 2.0);
		}
		let collider = {
			let mut sphere = SphereCollider::new(1.0);
			sphere.mass = 1.0;
			system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
		};
		{
			assert_eq!(system.get_entity(first).unwrap().get_last_total_mass(), 2.0);
		}
		system.link_collider(collider, Some(first)).unwrap();
		{
			assert_eq!(system.get_entity(first).unwrap().get_last_total_mass(), 3.0);
		}
		// Update the collider and make sure the entity also updates.
		if let ColliderWrapper::Sphere(mut interface) = system.get_collider(collider).unwrap() {
			interface.mass = 5.0;
			println!("Starting update!");
			system.update_collider(collider, ColliderWrapper::Sphere(interface)).unwrap();
			println!("Updated!");
		} else {
			panic!("The collider didn't unwrap into the right type!");
		}
		{
			assert_eq!(system.get_entity(first).unwrap().get_last_total_mass(), 7.0);
		}
		// Remove the collider and make sure the entity updates.
		// This checks switching the collider to a new entity.
		let second = {
			let entity = Entity::new();
			system.add_entity(entity).unwrap()
		};
		system.link_collider(collider, Some(second)).unwrap();
		{
			assert_eq!(system.get_entity(first).unwrap().get_last_total_mass(), 2.0);
			assert_eq!(system.get_entity(second).unwrap().get_last_total_mass(), 5.0);
		}
		// This checks just removing the collider.
		system.link_collider(collider, None).unwrap();
		{
			assert_eq!(system.get_entity(first).unwrap().get_last_total_mass(), 2.0);
			assert_eq!(system.get_entity(second).unwrap().get_last_total_mass(), 0.0);
		}
	}

	/// Check that angular velocity steps like it should.
	#[test]
	fn angular_update() {
		let mut system = PhysicsSystem::new();
		let first = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(1.0, 1.0, 1.0);
			system.add_entity(entity).unwrap()
		};
		let collider = {
			let sphere = SphereCollider::new(1.0);
			system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
		};
		system.link_collider(collider, Some(first)).unwrap();
		{
			let mut temp = system.get_entity(first).unwrap();
			temp.angular_velocity.z = 1.0;
			system.update_entity(first, temp).unwrap();
		}
		{
			let temp = system.get_entity(first).unwrap();
			assert!((temp.position - Vec3::new(1.0, 1.0, 1.0)).magnitude() < EPSILON);
			assert!((temp.rotation - Vec3::new(0.0, 0.0, 0.0)).magnitude() < EPSILON);
		}
		system.step(1.0);
		{
			let temp = system.get_entity(first).unwrap();
			assert!((temp.position - Vec3::new(1.0, 1.0, 1.0)).magnitude() < EPSILON);
			assert!((temp.rotation - Vec3::new(0.0, 0.0, 1.0)).magnitude() < EPSILON);
		}
		system.step(1.0);
		{
			let temp = system.get_entity(first).unwrap();
			assert!((temp.position - Vec3::new(1.0, 1.0, 1.0)).magnitude() < EPSILON);
			assert!((temp.rotation - Vec3::new(0.0, 0.0, 2.0)).magnitude() < EPSILON);
		}
	}

	/// Hit a pair of spheres in a way that causes the original sphere to stop moving.
	#[test]
	fn angular_adsorb_all_momentum() {
		let mut system = PhysicsSystem::new();
		let first = {
			let entity = Entity::new();
			system.add_entity(entity).unwrap()
		};
		{ // Add a collider
			let collider = {
				let mut sphere = SphereCollider::new(1.0);
				sphere.mass = 1.0;
				system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
			};
			system.link_collider(collider, Some(first)).unwrap();
		}
		{ // Set the velocity to y = +1
			let mut temp = system.get_entity(first).unwrap();
			temp.velocity.y = 1.0;
			system.update_entity(first, temp).unwrap();
		}
		let dual = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(2.0, 3.0, 0.0);
			system.add_entity(entity).unwrap()
		};
		{ // Add two colliders
			let left = {
				let mut sphere = SphereCollider::new(1.0);
				sphere.center = Vec3::new(-2.0, 0.0, 0.0);
				sphere.mass = 1.0;
				system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
			};
			system.link_collider(left, Some(dual)).unwrap();
			let right = {
				let mut sphere = SphereCollider::new(1.0);
				sphere.center = Vec3::new(2.0, 0.0, 0.0);
				sphere.mass = 1.0;
				system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap()
			};
			system.link_collider(right, Some(dual)).unwrap();
		}
		system.step(2.0);
		{
			let temp = system.get_entity(first).unwrap();
			println!("{:?}", temp);
			// The accumulated error on the below is surprisingly high.
			assert!((temp.position - Vec3::new(0.0, 1.0, 0.0)).magnitude() < 0.05);
			assert!((temp.velocity - Vec3::new(0.0, 0.0, 0.0)).magnitude() < 0.05);
		}
		{
			let temp = system.get_entity(dual).unwrap();
			assert!(temp.velocity.dot(&Vec3::new(0.0, 1.0, 0.0)) > EPSILON);
			assert!(temp.angular_velocity.dot(&Vec3::new(0.0, 0.0, -1.0)) > EPSILON);
		}
	}

	/// Verify the plane collider with zero restitution will stop a sphere.
	#[test]
	fn floor_stop() {
		let mut system = PhysicsSystem::new();
		let ball = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(0.0, 0.0, 2.0);
			entity.velocity = Vec3::new(0.0, 0.0, 2.0);
			let entity_handle = system.add_entity(entity).unwrap();
			let mut sphere = SphereCollider::new(1.0);
			sphere.mass = 1.0;
			let sphere_handle = system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap();
			system.link_collider(sphere_handle, Some(entity_handle)).unwrap();
			entity_handle
		};
		let wall = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(-1.0, 8.0, 4.0);
			let entity_handle = system.add_entity(entity).unwrap();
			let mut plane = PlaneCollider::new();
			plane.normal = -Vec3::z();
			plane.restitution_coefficient = 0.0;
			plane.mass = INFINITY;
			let plane_handle = system.add_collider(ColliderWrapper::Plane(plane)).unwrap();
			system.link_collider(plane_handle, Some(entity_handle)).unwrap();
			entity_handle
		};
		system.step(1.0);
		{
			let entity = system.get_entity(ball).unwrap();
			assert!((entity.position - Vec3::new(0.0, 0.0, 3.0)).magnitude() < EPSILON);
			assert!((entity.velocity - Vec3::new(0.0, 0.0, 0.0)).magnitude() < EPSILON);
		}
		{
			let entity = system.get_entity(wall).unwrap();
			assert!((entity.position - Vec3::new(-1.0, 8.0, 4.0)).magnitude() < EPSILON);
			assert!((entity.velocity - Vec3::new(0.0, 0.0, 0.0)).magnitude() < EPSILON);
		}
	}

	/// I'm seeing the physics engine glitch out and give huge amounts of angular velocity when hitting an infinitely massive wall.
	/// So this is trying to isolate that.
	#[test]
	fn wall_riccochet_energy() {
		let mut system = PhysicsSystem::new();
		const RADIUS : f32 = 1.0;
		const START_LINEAR_VELOCITY : f32 = 2.0;
		let dual = {
			let mut entity = Entity::new();
			entity.velocity = Vec3::new(0.0, 0.0, -START_LINEAR_VELOCITY);
			entity.angular_velocity = Vec3::new(0.0, -1.0, 0.0);
			let entity_handle = system.add_entity(entity).unwrap();
			//
			let mut sphere = SphereCollider::new(RADIUS);
			sphere.mass = 1.0;
			sphere.center = Vec3::new(1.0, 0.0, 0.0);
			let sphere_handle = system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap();
			system.link_collider(sphere_handle, Some(entity_handle)).unwrap();
			println!("sphere1: {:?}", sphere_handle);
			//
			let mut sphere = SphereCollider::new(RADIUS);
			sphere.mass = 1.0;
			sphere.center = Vec3::new(-1.0, 0.0, 0.0);
			let sphere_handle = system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap();
			system.link_collider(sphere_handle, Some(entity_handle)).unwrap();
			println!("sphere2: {:?}", sphere_handle);
			//
			entity_handle
		};
		let wall = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(0.0, 0.0, -2.0);
			let entity_handle = system.add_entity(entity).unwrap();
			let mut plane = PlaneCollider::new();
			plane.normal = Vec3::z();
			plane.mass = INFINITY;
			let plane_handle = system.add_collider(ColliderWrapper::Plane(plane)).unwrap();
			system.link_collider(plane_handle, Some(entity_handle)).unwrap();
			println!("wall: {:?}", plane_handle);
			entity_handle
		};
		const STEP : f32 = 0.1;
		for iteration in 0..100 {
			// Reset the positions/velocities/etc of the dual and the wall.
			let distance = -(iteration as f32) / 30.0 - 2.0;
			let wall_position = Vec3::new(0.0, 0.0, distance);
			{
				let mut entity = Entity::new();
				entity.velocity = Vec3::new(0.0, 0.0, -START_LINEAR_VELOCITY);
				entity.angular_velocity = Vec3::new(0.1, -1.0, 0.1); // The original bug didn't happen unless the axes were at least slightly skewed in more than one axis.
				system.update_entity(dual, entity).unwrap();
				//
				let mut entity = Entity::new();
				entity.position = wall_position;
				system.update_entity(wall, entity).unwrap();
			}
			let initial_energy = system.get_entity(dual).unwrap().get_total_energy();
			let total_time = 2.0 * (distance.abs() - RADIUS) / START_LINEAR_VELOCITY;
			for _ in 0..((total_time / STEP).ceil() as i32) {
				system.step(STEP);
			}
			let (final_energy, final_velocity) = {
				let entity = system.get_entity(dual).unwrap();
				(entity.get_total_energy(), entity.velocity)
			};
			let delta = final_energy - initial_energy;
			println!("T{} Energy change: {:?} -> {:?} (delta: {:?})", iteration, initial_energy, final_energy, delta);
			assert!(delta.abs() < EPSILON*10.0);
			assert!(0.0 < final_velocity.z);
			{ // Also verify the wall hasn't moved.
				let new_wall_position = system.get_entity(wall).unwrap().position;
				assert!((new_wall_position - wall_position).magnitude() < EPSILON);
			}
		}
	}

	/// Check that can add and remove a simple UnaryForceGenerator (GravityGenerator in this case).
	#[test]
	fn add_remove_unary_force_generator() {
		let mut system = PhysicsSystem::new();
		let handle = system.add_unary_force_generator(Box::new(GravityGenerator::new(Vec3::new(1.0, 2.0, 3.0)))).unwrap();
		let returned = system.remove_unary_force_generator(handle).unwrap();
		assert!((returned.downcast::<GravityGenerator>().unwrap().acceleration - Vec3::new(1.0, 2.0, 3.0)).magnitude() < EPSILON);
		assert!(system.remove_unary_force_generator(handle).is_none());
	}

	/// Check that gravity will drag a (perfectly inelastic) ball straight to the ground.
	#[test]
	fn basic_gravity() {
		const RADIUS : f32 = 1.0;
		let mut system = PhysicsSystem::new();
		let handle = {
			let mut entity = Entity::new();
			entity.position = Vec3::new(0.0, 3.0, 0.0);
			let entity_handle = system.add_entity(entity).unwrap();
			//
			let mut sphere = SphereCollider::new(RADIUS);
			sphere.mass = 1.0;
			sphere.restitution_coefficient = 0.0;
			let sphere_handle = system.add_collider(ColliderWrapper::Sphere(sphere)).unwrap();
			system.link_collider(sphere_handle, Some(entity_handle)).unwrap();

			entity_handle
		};
		{
			let entity_handle = system.add_entity(Entity::new()).unwrap();
			let mut plane = PlaneCollider::new();
			plane.normal = Vec3::y();
			plane.mass = INFINITY;
			let plane_handle = system.add_collider(ColliderWrapper::Plane(plane)).unwrap();
			system.link_collider(plane_handle, Some(entity_handle)).unwrap();
		}

		system.add_unary_force_generator(Box::new(GravityGenerator::new(Vec3::new(0.0, -1.0, 0.0)))).unwrap();

		for _ in 0..5 {
			system.step(5.0);
		}

		{
			let position = system.get_entity(handle).unwrap().position;
			println!("Final position: {:?}", position);
			assert!((position - Vec3::new(0.0, 1.0, 0.0)).magnitude() < EPSILON);
		}
	}

	// TODO? Only angular inertia into a collision.
	// TODO? Check attaching a collider with mass after rotation has already begun -> verify doesn't look weird.
}
