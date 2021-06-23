
use nalgebra::{Translation3, Point3};

use crate::types::{Vec3, Mat3, Quat, Isometry};

/// A structure for storing the frame-of-reference for the local space of an entity.
///
/// Put another way, this is how to get from an entity's local space to world space (and vise versa).
#[derive(Copy, Clone, Debug)]
pub struct Orientation {
	/// The point all rotations are about.
	///
	/// The will be the center-of-mass for entities.
	///
	/// This is stored in WORLD coordinates.
	///
	/// Generally avoid changing this directly. It's better to use [Orientation::affect_with] and [Orientation::after_affected].
	pub position : Vec3,

	/// The current rotation that this reference frame.
	///
	/// This will be the rotation of the entity about it's center of mass.
	///
	/// Generally should preferr using the [Orientation::affect_with] and [Orientation::after_affected] functions to apply changes.
	pub rotation : Quat,

	/// The origin of the LOCAL space.
	///
	/// For entities this is the vector from the center of mass to the entity's "position" in LOCAL space.
	///
	/// This will generally never change unless an object's mass distribution changes.
	pub internal_origin_offset : Vec3,
}

/// Uses parallel axis theorem to translate the given moment of inertia tensor.
///
/// **WARNING:** This can only be applied to a moment of intertia tensor ONCE (as the math only works out if the passed in tensor is centered about the center of mass). In other words: once a moment of inertia tensor is passed through this it makes no sense to ever pass it through this again.
fn translate_moment_of_inertia(moment : &Mat3, total_mass : f32, translation : &Vec3) -> Mat3 {
	moment + total_mass * (Mat3::from_diagonal_element(translation.dot(&translation)) - translation * translation.transpose())
}

/// Rotates the given moment of inertia tensor.
///
/// This applies a very generic sort of generic "transform from one space into another" matrix handling. Nothing more unique is needed for moment of inertia tensors.
fn rotate_moment_of_inertia(moment : &Mat3, rotation : &Quat) -> Mat3 {
	let out_of = rotation.to_rotation_matrix();
	out_of * moment * out_of.transpose()
}

impl Orientation {
	/// Creates a new instance.
	pub fn new(position : &Vec3, rotation : &Vec3, internal_origin_offset : &Vec3) -> Orientation {
		Orientation {
			position: position.clone(),
			rotation: Quat::from_scaled_axis(*rotation),
			internal_origin_offset: internal_origin_offset.clone(),
		}
	}

	/// Creates a way to get into local space from world space.
	pub fn into_local(&self) -> Isometry {
		let mut transform = Isometry::from_parts(Translation3::from(-self.position), Quat::identity());
		transform.append_rotation_mut(&self.rotation.inverse());
		transform.append_translation_mut(&Translation3::from(-self.internal_origin_offset));
		transform
	}

	/// Creates a way to get to world space from local space.
	pub fn into_world(&self) -> Isometry {
		let mut transform = self.into_local();
		transform.inverse_mut(); // TODO: Could slightly optimize. Inverse is probably an expensive function...
		transform
	}

	/// Linearly interpolates between a starting and ending orientation.
	pub fn lerp(time : f32, start : &Orientation, end : &Orientation) -> Orientation {
		let opposite = 1.0 - time;
		let rotation_vec = start.rotation_vec() * opposite + end.rotation_vec() * time;
		Orientation {
			position: start.position * opposite + end.position * time,
			rotation: Quat::from_scaled_axis(rotation_vec),
			internal_origin_offset: start.internal_origin_offset.clone(),
		}
	}

	/// Converts a world position into local space.
	///
	/// So this applies the orientation's (inverse) rotation and (inverse) translation to the position.
	pub fn position_into_local(&self, position : &Vec3) -> Vec3 {
		self.into_local().transform_point(&Point3::from(*position)).coords
	}

	/// Converts a local position into world space.
	///
	/// So this applies the orientation's rotation and translation to the position.
	pub fn position_into_world(&self, position : &Vec3) -> Vec3 {
		self.into_world().transform_point(&Point3::from(*position)).coords
	}

	/// Converts a direction in world space into local space.
	///
	/// This means it applies the inverse of this orientation's rotation matrix to position.
	pub fn direction_into_local(&self, direction : &Vec3) -> Vec3 {
		self.into_local().transform_vector(direction)
	}

	/// Converts a direction in local space into world space.
	///
	/// This means it applies this orientation's rotation matrix to position.
	pub fn direction_into_world(&self, direction : &Vec3) -> Vec3 {
		self.into_world().transform_vector(direction)
	}

	/// Transform a moment of inertia tensor that's about the given `center_of_mass` (which is specified in local space) so that it's relative to this orientation's `position` in local space.
	///
	/// This should exclusively be used used internally. There's no good reason anything outside this crate would ever need to call this.
	///
	/// Since this orientation's `position` is usually its center-of-mass, this effectively gets the moment to be ready to be passed through [Orientation::finalize_moment_of_inertia] so it can be readily available in world-space (and be centered about the center of mass there).
	pub fn prep_moment_of_inertia(&self, center_of_mass : &Vec3, total_mass : f32, moment : &Mat3) -> Mat3 {
		translate_moment_of_inertia(moment, total_mass, &(self.internal_origin_offset + center_of_mass))
	}

	/// Completes converting a moment of inertia that was sent through prep_moment_of_inertia() so that it's in world-space.
	///
	/// This means it applies this orientation's rotation (matrix) to the tensor.
	pub fn finalize_moment_of_inertia(&self, moment : &Mat3) -> Mat3 {
		rotate_moment_of_inertia(moment, &self.rotation)
	}

	/// The local space's origin in world coordinates.
	pub fn local_origin_in_world(&self) -> Vec3 {
		self.position_into_world(&Vec3::zeros())
	}

	/// Creates a axis-angle rotation vector describing the current rotation.
	pub fn rotation_vec(&self) -> Vec3 {
		self.rotation.scaled_axis()
	}

	/// Applies the given rotation and translation to this instance.
	pub fn affect_with(&mut self, linear_movement : &Vec3, angular_movement : &Vec3) {
		self.position += linear_movement;
		self.rotation = Quat::from_scaled_axis(*angular_movement) * self.rotation;
	}

	/// Stores the result of applying a rotation and translation to this instance in a new instance.
	pub fn after_affected(&self, linear_movement : &Vec3, angular_movement : &Vec3) -> Orientation {
		let mut copy = self.clone();
		copy.affect_with(linear_movement, angular_movement);
		copy
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::f32::consts::PI;
	use crate::consts::EPSILON;

	/// Verify basic transformations work as expected.
	#[test]
	fn basic_transforms() {
		let mut orientation = Orientation::new(&Vec3::new(1.0, 1.0, 1.0), &Vec3::zeros(), &Vec3::new(0.0, 2.0, 0.0));
		{
			let origin = orientation.local_origin_in_world();
			assert_eq!(origin.x, 1.0);
			assert_eq!(origin.y, 3.0);
			assert_eq!(origin.z, 1.0);
			let transformed = orientation.into_world().transform_point(&Point3::new(0.0, -1.0, 0.0));
			assert_eq!(transformed.x, 1.0);
			assert_eq!(transformed.y, 2.0);
			assert_eq!(transformed.z, 1.0);
		}
		orientation = orientation.after_affected(&Vec3::zeros(), &Vec3::z().scale(-PI / 2.0));
		{
			let origin = orientation.local_origin_in_world();
			assert!((origin.x - 3.0).abs() < EPSILON);
			assert!((origin.y - 1.0).abs() < EPSILON);
			assert!((origin.z - 1.0).abs() < EPSILON);
			let transformed = orientation.into_world().transform_point(&Point3::new(0.0, -1.0, 0.0));
			assert!((transformed.x - 2.0).abs() < EPSILON);
			assert!((transformed.y - 1.0).abs() < EPSILON);
			assert!((transformed.z - 1.0).abs() < EPSILON);
			let transformed = orientation.into_world().transform_point(&Point3::new(1.0, 0.0, 0.0));
			assert!((transformed.x - 3.0).abs() < EPSILON);
			assert!((transformed.y - 0.0).abs() < EPSILON);
			assert!((transformed.z - 1.0).abs() < EPSILON);
		}
	}

	/*fn point_moment_of_inertia_tensor(point : &Vec3, mass : f32) -> Mat3 {
		let len = point.dot(&point);
		Mat3::new(
			len - point.x * point.x,     - point.x * point.y,     - point.x * point.z,
			    - point.y * point.x, len - point.y * point.y,     - point.y * point.z,
			    - point.z * point.x,     - point.z * point.y, len - point.z * point.z,
		).scale(mass)
	}*/
}
