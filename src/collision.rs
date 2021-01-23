use crate::types::{Vec3};
use crate::range::Range;
use crate::collider::{ColliderType, InternalCollider};
use crate::sphere_collider::{InternalSphereCollider};
use crate::orientation::{Orientation};

/// A structure for storing collision information.
#[derive(Debug)]
pub struct Collision {
	/// The range of times when the collision happened. Will be in the range 0.0 (meaning the very start) and 1.0 (the very end).
	pub times : Range,
	/// The position of the hit.
	pub position : Vec3,
	/// The normal of the hit (pointing off the first object).
	pub normal : Vec3,
}

impl Collision {
	//
}

/// Tries to collide any two arbitrary colliders.
pub fn collide(collider1 : &Box<dyn InternalCollider>, start1 : &Orientation, end1 : &Orientation, collider2 : &Box<dyn InternalCollider>, start2 : &Orientation, end2 : &Orientation) -> Option<Collision> {
	if ColliderType::SPHERE == collider1.get_type() && ColliderType::SPHERE == collider2.get_type() {
		let col1 = collider1.downcast_ref::<InternalSphereCollider>().unwrap();
		let col2 = collider2.downcast_ref::<InternalSphereCollider>().unwrap();

		let col1_start_position = start1.position_into_world(&col1.center);
		let col1_end_position = end1.position_into_world(&col1.center);
		let col2_start_position = start2.position_into_world(&col2.center);
		let col2_end_position = end2.position_into_world(&col2.center);
		collide_sphere_with_sphere(
			col1.radius,
			&col1_start_position,
			&(col1_end_position - col1_start_position),
			col2.radius,
			&col2_start_position,
			&(col2_end_position - col2_start_position),
		)
	} else {
		None
	}
}

/// Detect when and where a point hits a sphere (if ever).
pub fn collide_sphere_with_sphere(radius1 : f32, center1 : &Vec3, movement1 : &Vec3, radius2 : f32, center2 : &Vec3, movement2 : &Vec3) -> Option<Collision> {
	let dv = movement1 - movement2;
	let dc = center1 - center2;
	let radius = radius1 + radius2;
	let times = Range::quadratic_zeros(
		dv.dot(&dv),
		2.0 * dv.dot(&dc),
		dc.dot(&dc) - radius * radius,
	).intersect(&Range::range(0.0, 1.0));
	if !times.is_empty() {
		let position = (
			(center1 + movement1.scale(times.min())) * radius2 +
			(center2 + movement2.scale(times.min())) * radius1
		).scale(1.0 / radius);
		let normal = (position - center1).normalize();
		Some(Collision {
			times,
			position,
			normal,
		})
	} else { None }
}

#[cfg(test)]
mod tests {
	use crate::consts::EPSILON;
	use super::*;

	#[test]
	fn check_collide_sphere_with_sphere() {
		{ // Two spheres moving toward eachother.
			let hit = collide_sphere_with_sphere(
				1.0,
				&Vec3::new(1.0, 1.0, 1.0),
				&Vec3::new(2.0, 0.0, 0.0),
				1.0,
				&Vec3::new(5.0, 1.0, 1.0),
				&Vec3::new(-2.0, 0.0, 0.0),
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(3.0, 1.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(1.0, 0.0, 0.0)).magnitude() < EPSILON);
		}
	}
}
