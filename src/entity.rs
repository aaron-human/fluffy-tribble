use std::f32::INFINITY;
use std::collections::HashSet;

use generational_arena::Arena;

use crate::consts::EPSILON;
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

	/// The (cached) of the moment-of-inertia tensor (including all colliders) BEFORE it's been rotated to be in world space.
	///
	/// This should only ever be udpated by calling recalculate_mass().
	prepped_moment_of_inertia : Mat3,

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
			prepped_moment_of_inertia: Mat3::zeros(),

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

			last_prepped_moment_of_inertia: self.prepped_moment_of_inertia.clone(),
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
		let mut found_infinite = false;
		for handle in self.colliders.iter() {
			let collider = colliders.get(*handle).unwrap();
			let collider_mass = collider.get_mass();
			if collider_mass.is_infinite() {
				found_infinite = true;
				break;
			}
			total_other_mass += collider_mass;
			center_of_mass += self.orientation.position_into_world(&collider.get_local_center_of_mass()).scale(collider_mass);
		}
		if found_infinite { self.total_mass = INFINITY; }
		if 0.0 < total_other_mass && !found_infinite {
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
		self.prepped_moment_of_inertia = Mat3::zeros();
		if !found_infinite {
			// TODO? Do orientation.rotation and angular_velocity need to change since the center-of-mass changed?
			for handle in self.colliders.iter() {
				let collider = colliders.get(*handle).unwrap();
				self.prepped_moment_of_inertia += self.orientation.prep_moment_of_inertia(
					&collider.get_local_center_of_mass(),
					collider.get_mass(),
					&collider.get_moment_of_inertia_tensor(),
				);
			}
		}
	}

	/// Gets the total mass of this entity and all of its colliders.
	pub fn get_total_mass(&self) -> f32 {
		self.total_mass
	}

	/// Gets the moment of inertia tensor in WORLD space.
	pub fn get_moment_of_inertia(&self) -> Mat3 {
		self.orientation.finalize_moment_of_inertia(&self.prepped_moment_of_inertia)
	}

	/// Gets the moment of inertia tensor in WORLD space.
	pub fn get_inverse_moment_of_inertia(&self) -> Mat3 {
		let moment = self.get_moment_of_inertia();
		if let Some(inverse) = moment.try_inverse() {
			inverse
		} else {
			if EPSILON < moment.magnitude() {
				println!("WARNING! No inverse found for moment of inertia! {:?}", moment);
			}
			Mat3::zeros()
		}
	}

	/// Gets the velocity at a point (that's specified in world coordinates).
	pub fn get_velocity_at_world_position(&self, position : &Vec3) -> Vec3 {
		self.velocity + self.angular_velocity.cross(&(position - self.orientation.position))
	}

	/// Gets the total energy of this object.
	#[allow(dead_code)]
	pub fn get_total_energy(&self) -> f32 {
		let linear_energy = (self.total_mass * self.velocity).dot(&self.velocity) / 2.0;
		let angular_energy = (self.get_moment_of_inertia() * self.angular_velocity).dot(&self.angular_velocity) / 2.0;
		linear_energy + angular_energy
	}

	// Applies an impulse at a (world) position to this instance's linear and angular velocities.
	pub fn apply_impulse(&mut self, position : &Vec3, impulse : &Vec3) {
		self.velocity += impulse.scale(1.0 / self.get_total_mass());
		self.angular_velocity += self.get_inverse_moment_of_inertia() * (position - self.orientation.position).cross(&impulse);
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

	/// Last known moment of inertia in world space (but BEFORE it was rotated according to 'rotation'). This is very much read-only.
	///
	/// Defaults to a zero matrix.
	last_prepped_moment_of_inertia : Mat3,
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
			last_prepped_moment_of_inertia: Mat3::zeros(),
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

	/// Gets the moment of inertia in WORLD space.
	fn get_last_moment_of_inertia(&self) -> Mat3 {
		self.make_orientation().finalize_moment_of_inertia(&self.last_prepped_moment_of_inertia)
	}

	/// Gets the total energy of this object.
	pub fn get_total_energy(&self) -> f32 {
		let linear_energy = (self.last_total_mass * self.velocity).dot(&self.velocity) / 2.0;
		let angular_energy = (self.get_last_moment_of_inertia() * self.angular_velocity).dot(&self.angular_velocity) / 2.0;
		linear_energy + angular_energy
	}
}
