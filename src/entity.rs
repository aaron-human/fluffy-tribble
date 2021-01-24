use std::collections::HashSet;

use generational_arena::Arena;

use crate::types::{Vec3, Mat3, Quat, ColliderHandle};
use crate::collider::InternalCollider;
use crate::orientation::Orientation;

/// The internal representation of any physical object.
/// This generally has NO data hiding to keep things simple.
pub struct InternalEntity {
	/// The current position and rotation.
	pub orientation : Orientation,

	/// The mass of this entity at the center of mass (as a point mass).
	/// This is NOT the total mass.
	pub own_mass : f32,

	/// The (cached) total mass (including all colliders).
	///
	/// This should only ever be udpated by calling recalculate_mass().
	total_mass : f32,

	/// The (cached) inverse of the moment-of-inertia tensor (including all colliders).
	/// This is in WORLD space.
	///
	/// This should only ever be udpated by calling recalculate_mass().
	inverse_moment_of_inertia : Mat3,

	/// The current linear velocity.
	pub velocity : Vec3,

	/// The current angular velocity (about the center of mass).
	pub angular_velocity : Vec3,

	/// All colliders that are attached/linked to this.
	pub colliders : HashSet<ColliderHandle>,
}

impl InternalEntity {
	/// Creates a new instance.
	pub fn new_from(source : Entity) -> Result<InternalEntity, ()> {
		if 0.0 > source.own_mass { return Err(()); }
		Ok(InternalEntity {
			orientation: source.make_orientation(),

			own_mass: source.own_mass,
			total_mass: source.own_mass,
			inverse_moment_of_inertia: Mat3::zeros(),

			velocity: source.velocity,
			angular_velocity: source.angular_velocity,
			colliders: HashSet::new(),
		})
	}

	/// Creates the public interface for this instance.
	pub fn make_pub(&self) -> Entity {
		Entity {
			position: self.orientation.position.clone(),
			rotation: self.orientation.rotation_vec(),

			last_orientation: self.orientation.clone(),

			own_mass: self.own_mass,
			last_total_mass: self.get_total_mass(),

			velocity: self.velocity.clone(),
			angular_velocity: self.angular_velocity,

			colliders: self.colliders.clone(),
		}
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : Entity) -> Result<(),()> {
		if 0.0 > source.own_mass { return Err(()); }
		self.own_mass = source.own_mass;
		self.orientation.position = source.position;

		self.orientation.rotation = Quat::from_scaled_axis(source.rotation);

		self.velocity = source.velocity;
		self.angular_velocity = source.angular_velocity;
		Ok(())
	}

	/// Recalculates the (cached) mass and inertia values.
	pub fn recalculate_mass(&mut self, colliders : &Arena<Box<dyn InternalCollider>>) {
		// First find the center of mass.
		self.total_mass = self.own_mass;
		let mut center_of_mass = Vec3::zeros();
		let mut total_other_mass = 0.0;
		for handle in self.colliders.iter() {
			let collider = colliders.get(*handle).unwrap();
			let collider_mass = collider.get_mass();
			total_other_mass += collider_mass;
			center_of_mass += self.orientation.position_into_world(&collider.get_local_center_of_mass()).scale(collider_mass);
		}
		if 0.0 < total_other_mass {
			self.total_mass += total_other_mass;
			// If there are colliders with mass, then use them to decide where this entity's center-of-mass is.
			//
			// Note that this entity's center of mass decides where it's own_mass is distributed. And that the center of mass calculation doesn't affix that mass to any point.
			// SO the own_mass practically just teleports to where ever the center of mass moves to.
			center_of_mass /= total_other_mass;
			let center_of_mass_movement = center_of_mass - self.orientation.position; // How much the center of mass moves in world space.
			self.orientation.internal_origin_offset -= self.orientation.direction_into_local(&center_of_mass_movement); // Keep the origin of the local space at the same position as the position of the local space moves.
			self.orientation.position += center_of_mass_movement; // Then move the center-of-mass accordingly.
		} else {
			// If there are no colliders then move the center-of-mass to the origin of the local space.
			let local_center_of_mass_movement = self.orientation.internal_origin_offset;
			self.orientation.internal_origin_offset -= local_center_of_mass_movement; // Keep the origin of the local space at the same position as the position of the local space moves.
			self.orientation.position += self.orientation.direction_into_world(&local_center_of_mass_movement); // Then move the center-of-mass accordingly.
		}

		// Then find the moment of inertia relative to the center-of-mass.
		// Note that everything here is done in WORLD space not local.
		self.inverse_moment_of_inertia = Mat3::zeros();
		// TODO? Do orientation.rotation and angular_velocity need to change since the center-of-mass changed?
		for handle in self.colliders.iter() {
			let collider = colliders.get(*handle).unwrap();
			let offset = self.orientation.position_into_world(&collider.get_local_center_of_mass()) - self.orientation.position;
			let translated_moment_of_inertia =
				self.orientation.tensor_into_world(&collider.get_moment_of_inertia_tensor()) +
				collider.get_mass() * (Mat3::from_diagonal_element(offset.dot(&offset)) - offset * offset.transpose());
			self.inverse_moment_of_inertia += translated_moment_of_inertia;
		}
		self.inverse_moment_of_inertia.try_inverse_mut();
	}

	/// Gets the total mass of this entity and all of its colliders.
	pub fn get_total_mass(&self) -> f32 {
		self.total_mass
	}

	/// Gets the moment of inertia tensor in WORLD space.
	pub fn get_inverse_moment_of_inertia(&self) -> Mat3 {
		self.inverse_moment_of_inertia
	}
}

/// A copy of all of the publicly-accessible properties of a physical object in the world.
#[derive(Debug, Clone)]
pub struct Entity {
	/// The current position of the center of mass in WORLD space.
	///
	/// Defaults to origin.
	pub position : Vec3,

	/// The current rotation about the center of mass in WORLD space.
	///
	/// Defaults to no rotation (zero vector).
	pub rotation : Vec3,

	/// The current velocity of the center of mass in WORLD space.
	///
	/// Defaults to no movement (zero vector).
	pub velocity : Vec3,

	/// The current angular velocity about the center of mass in WORLD space.
	///
	/// Defaults to no rotation (zero vector).
	pub angular_velocity : Vec3,

	/// All colliders that are attached/linked to this.
	///
	/// Defaults to an empty set.
	colliders : HashSet<ColliderHandle>,

	/// The current mass that just this entity contributes as a point-mass at the center of mass.
	///
	/// This does NOT include collider masses.
	///
	/// Note that this mass does NOT affect how the center of mass is decided. That's strictly a weighted sum with the colliders.
	///
	/// Defaults to zero.
	pub own_mass : f32,

	/// The last known orientation. This is very much read-only.
	///
	/// Defaults to having no offset or transform.
	last_orientation : Orientation,

	/// Last known total mass (including colliders). This is very much read-only.
	///
	/// Defaults to zero.
	last_total_mass : f32,
}

impl Entity {
	/// Creates a new store for entity information with everything set to its defaults.
	/// Can use this to store info for an [crate::physics_system::add_entity] call later.
	pub fn new() -> Entity {
		Entity {
			position: Vec3::zeros(),
			rotation: Vec3::zeros(),
			velocity: Vec3::zeros(),
			angular_velocity: Vec3::zeros(),
			colliders: HashSet::new(),
			own_mass: 0.0,
			last_orientation: Orientation::new(
				&Vec3::zeros(),
				&Vec3::zeros(),
				&Vec3::zeros(),
			),
			last_total_mass: 0.0,
		}
	}

	/// Gets all collider handles.
	///
	/// Notibly this is just the getter, as this object cannot be used to modify what colliders are attached to this entity (must use `link_collider()` for that).
	pub fn get_colliders(&self) -> HashSet<ColliderHandle> {
		self.colliders.clone()
	}

	/// Gets the last known total mass of this entity.
	pub fn get_last_total_mass(&self) -> f32 { self.last_total_mass }

	/// Gets the last orientation used by the entity.
	///
	/// This makes it easy to convert from local coordinates to global ones.
	pub fn get_last_orientation<'a>(&'a self) -> &'a Orientation {
		&self.last_orientation
	}

	/// Creates a new orientation using the current values of position and rotation along with the center of mass offset from the last orientation.
	pub fn make_orientation(&self) -> Orientation {
		Orientation::new(
			&self.position,
			&self.rotation,
			&self.last_orientation.internal_origin_offset,
		)
	}
}
