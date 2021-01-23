
use nalgebra::{Translation3, Point3};

use crate::types::{Vec3, Quat, Isometry};

/// A structure for storing a set of heavily-related 3D items used for orientation.
#[derive(Copy, Clone, Debug)]
pub struct Orientation {
	/// The point all rotations are about.
	/// The will usually be the center-of-mass for physical objects.
	/// This is stored in WORLD coordinates.
	pub position : Vec3,
	/// The current rotation that this object is at.
	rotation : Quat,
	/// A special position that acts as the origin of the LOCAL space.
	/// This offset is stored relative to 'position' (in LOCAL coordinates).
	/// For physical objects, this is usually the entity's 'position'.
	internal_origin : Vec3,
}

impl Orientation {
	/// Creates a new instance.
	pub fn new(position : &Vec3, rotation : &Vec3, internal_origin : &Vec3) -> Orientation {
		Orientation {
			position: position.clone(),
			rotation: Quat::from_scaled_axis(*rotation),
			internal_origin: internal_origin.clone(),
		}
	}

	/// Creates a matrix to get into local space from world space.
	pub fn into_local(&self) -> Isometry {
		let mut transform = Isometry::from_parts(Translation3::from(-self.position), Quat::identity());
		transform.append_rotation_mut(&self.rotation.inverse());
		transform.append_translation_mut(&Translation3::from(-self.internal_origin));
		transform
	}

	/// Creates a matrix to get to world space from local space.
	pub fn into_world(&self) -> Isometry {
		let mut transform = self.into_local();
		transform.inverse_mut();
		transform
	}

	/// Converts a local position into world space.
	pub fn position_into_world(&self, position : &Vec3) -> Vec3 {
		self.into_world().transform_point(&Point3::from(*position)).coords
	}

	/// The local origin in world coordinates.
	pub fn local_origin_in_world(&self) -> Vec3 {
		self.position_into_world(&Vec3::zeros())
	}

	/// Creates an instance that is like this one after a rotation and translation has been applied.
	pub fn after_affected(&self, linear_movement : &Vec3, angular_movement : &Vec3) -> Orientation {
		Orientation {
			position: self.position + linear_movement,
			rotation: Quat::from_scaled_axis(*angular_movement) * self.rotation,
			internal_origin: self.internal_origin,
		}
	}

	/// Creates a axis-angle rotation vector describing the current rotation.
	pub fn rotation_vec(&self) -> Vec3 {
		self.rotation.scaled_axis()
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
}
