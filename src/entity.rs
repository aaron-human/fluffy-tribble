use std::collections::HashSet;

use generational_arena::Arena;

use crate::types::{Vec3, Mat3, Quat, ColliderHandle};
use crate::collider::InternalCollider;
use crate::orientation::Orientation;

/// The internal representation of any physical object.
/// This generally has NO data hiding to keep things simple.
pub struct InternalEntity {
	/// The moment of inertia. TODO!
	/// The current 3D position.
	pub position : Vec3,
	/// The current 3D velocity.
	pub velocity : Vec3,
	/// All colliders that are attached/linked to this.
	pub colliders : HashSet<ColliderHandle>,

	/// The mass of this entity at position (as a point mass).
	/// This is NOT the total mass.
	pub own_mass : f32,
	/// The (cached) total mass (including all colliders).
	total_mass : f32,
	/// The (cached) relative center-of-mass (including all colliders).
	/// This is RELATIVE to `position`, and WITHIN the local space.
	center_of_mass_offset : Vec3,
	/// The (cached) moment-of-inertia tensor (including all colliders).
	moment_of_inertia : Mat3,

	/// The current rotation about the center of mass.
	pub rotation : Vec3,
	/// The current angular velocity about the center of mass.
	pub angular_velocity : Vec3,
}

impl InternalEntity {
	/// Creates a new instance.
	pub fn new(position : &Vec3, mass : f32) -> Result<InternalEntity, ()> {
		if 0.0 > mass { return Err(()); }
		Ok(InternalEntity {
			position: position.clone(),
			velocity: Vec3::zeros(),
			colliders: HashSet::new(),

			own_mass: mass,
			total_mass: mass,
			center_of_mass_offset: position.clone(),
			moment_of_inertia: Mat3::zeros(),

			rotation: Vec3::zeros(),
			angular_velocity: Vec3::zeros(),
		})
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : Entity) -> Result<(),()> {
		if 0.0 > source.own_mass { return Err(()); }
		self.own_mass = source.own_mass;
		self.position = source.position;
		self.velocity = source.velocity;
		self.rotation = source.rotation;
		self.angular_velocity = source.angular_velocity;
		Ok(())
	}

	/// Recalculates the (cached) mass and inertia values.
	pub fn recalculate(&mut self, colliders : &Arena<Box<dyn InternalCollider>>) {
		// First find the center of mass.
		self.total_mass = self.own_mass;
		self.center_of_mass_offset = Vec3::zeros();
		for handle in self.colliders.iter() {
			let collider = colliders.get(*handle).unwrap();
			let collider_mass = collider.get_mass();
			self.total_mass += collider_mass;
			self.center_of_mass_offset += collider.get_center_of_mass().scale(collider_mass);
		}
		self.center_of_mass_offset /= self.total_mass;

		// Then find the moment of inertia relative to the center-of-mass.
		self.moment_of_inertia = Mat3::zeros();
		// TODO? Make changes in the moment of inertia rescale the angular velocity? Could calculate the energy before then rescale it to be the same after...
		// TODO? Does the rotation angle need to change since the center-of-mass changed?
		for handle in self.colliders.iter() {
			let collider = colliders.get(*handle).unwrap();
			let offset = collider.get_center_of_mass() - self.center_of_mass_offset;
			let translated_moment_of_inertia = collider.get_moment_of_inertia_tensor() +
				collider.get_mass() * (Mat3::from_diagonal_element(offset.dot(&offset)) - offset * offset.transpose());
			self.moment_of_inertia += translated_moment_of_inertia;
		}
	}

	/// Gets the total mass of this entity and all of its colliders.
	pub fn get_total_mass(&self) -> f32 {
		self.total_mass
	}

	/// Gets the center of mass (accounting for all colliders), in LOCAL space.
	pub fn get_center_of_mass_offset(&self) -> Vec3 {
		self.center_of_mass_offset
	}

	/// Gets the moment of inertia tensor.
	pub fn get_moment_of_inertia(&self) -> Mat3 {
		let rotation = Quat::from_scaled_axis(self.rotation).to_rotation_matrix();
		rotation * self.moment_of_inertia * rotation.transpose()
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

	/// The current mass that just this entity contributes (does NOT include colliders).
	pub own_mass : f32,
	/// Last known total mass (including colliders).
	total_mass : f32,
	/// Last known center of mass. This does NOT get pushed back to InternalEntity upon update().
	center_of_mass_offset : Vec3,
	/// Last known moment of inertia tensor.
	moment_of_inertia : Mat3,

	/// The current rotation about the center of mass.
	pub rotation : Vec3,
	/// The current angular velocity about the center of mass.
	pub angular_velocity : Vec3,
}

impl Entity {
	/// Creates from an InternalEntity.
	pub fn from(source : &InternalEntity) -> Entity {
		Entity {
			own_mass: source.own_mass,
			position: source.position.clone(),
			velocity: source.velocity.clone(),
			colliders: source.colliders.clone(),

			total_mass: source.get_total_mass(),
			center_of_mass_offset: source.get_center_of_mass_offset(),
			moment_of_inertia: source.get_moment_of_inertia(),

			rotation: source.rotation,
			angular_velocity: source.angular_velocity,
		}
	}

	/// Gets all collider handles.
	/// Notibly this is just the getter, as this object cannot be used to modify what colliders are attached to this entity.
	pub fn get_colliders(&self) -> HashSet<ColliderHandle> {
		self.colliders.clone()
	}

	/// Gets the last known total mass of this entity.
	pub fn get_last_total_mass(&self) -> f32 { self.total_mass }
	/// Gets the last known center-of-mass of this entity.
	pub fn get_last_center_of_mass_offset(&self) -> Vec3 { self.center_of_mass_offset }
	/// Gets the last known moment-of-inertia tensor of this entity.
	pub fn get_last_moment_of_inertia(&self) -> Mat3 { self.moment_of_inertia }

	/// Makes an orientation from the current settings.
	/// This makes it easy to convert from local coordinates to global ones.
	pub fn make_orientation(&self) -> Orientation {
		Orientation::new(
			&(self.position + self.center_of_mass_offset),
			&self.rotation,
			&(-self.center_of_mass_offset),
		)
	}
}
