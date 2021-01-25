use std::f32::INFINITY;

use crate::types::{Vec3};
use crate::range::Range;
use crate::collider::{ColliderType, InternalCollider};
use crate::sphere_collider::{InternalSphereCollider};
use crate::plane_collider::{InternalPlaneCollider};
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
	// Always ignore a NullCollider.
	// This is redundant now, but won't be in the future.
	if ColliderType::NULL == collider1.get_type() || ColliderType::NULL == collider2.get_type() {
		return None
	}

	if ColliderType::SPHERE == collider1.get_type() && ColliderType::SPHERE == collider2.get_type() {
		let col1 = collider1.downcast_ref::<InternalSphereCollider>().unwrap();
		let col2 = collider2.downcast_ref::<InternalSphereCollider>().unwrap();

		let col1_start_position = start1.position_into_world(&col1.center);
		let col1_end_position = end1.position_into_world(&col1.center);
		let col2_start_position = start2.position_into_world(&col2.center);
		let col2_end_position = end2.position_into_world(&col2.center);

		return collide_sphere_with_sphere(
			col1.radius,
			&col1_start_position,
			&(col1_end_position - col1_start_position),
			col2.radius,
			&col2_start_position,
			&(col2_end_position - col2_start_position),
		);
	}

	if ColliderType::SPHERE == collider1.get_type() && ColliderType::PLANE == collider2.get_type() {
		let sphere = collider1.downcast_ref::<InternalSphereCollider>().unwrap();
		let sphere_start_position = start1.position_into_world(&sphere.center);
		let sphere_end_position = end1.position_into_world(&sphere.center);

		let plane  = collider2.downcast_ref::<InternalPlaneCollider>().unwrap();
		let plane_start_position = start2.position_into_world(&plane.position);
		let plane_end_position = end2.position_into_world(&plane.position);

		return collide_sphere_with_plane(
			sphere.radius,
			&sphere_start_position,
			&(sphere_end_position - sphere_start_position),
			&plane_start_position,
			&plane.normal,
			&(plane_end_position - plane_start_position)
		);
	}
	if ColliderType::PLANE == collider1.get_type() && ColliderType::SPHERE == collider2.get_type() {
		let plane  = collider1.downcast_ref::<InternalPlaneCollider>().unwrap();
		let plane_start_position = start1.position_into_world(&plane.position);
		let plane_end_position = end1.position_into_world(&plane.position);

		let sphere = collider2.downcast_ref::<InternalSphereCollider>().unwrap();
		let sphere_start_position = start2.position_into_world(&sphere.center);
		let sphere_end_position = end2.position_into_world(&sphere.center);

		let collision_option = collide_sphere_with_plane(
			sphere.radius,
			&sphere_start_position,
			&(sphere_end_position - sphere_start_position),
			&plane_start_position,
			&plane.normal, // TODO: The plane's normal could rotate?
			&(plane_end_position - plane_start_position)
		);
		// Must negate the normal as the sphere is the first collider.
		if let Some(mut collision) = collision_option {
			collision.normal *= -1.0;
			return Some(collision);
		} else {
			return None
		}
	}
	// I don't think it makes sense to detect when two planes are colliding...

	None
}

/// Collide a sphere with an inifinite plane.
pub fn collide_sphere_with_plane(radius1 : f32, center1 : &Vec3, movement1 : &Vec3, position2 : &Vec3, normal2 : &Vec3, movement2 : &Vec3) -> Option<Collision> {
	let start_nearest  = center1 + normal2.scale(-radius1);
	let start_farthest = center1 + normal2.scale( radius1);
	let circle_range = Range::range(
		start_nearest.dot(normal2),
		start_farthest.dot(normal2),
	);
	let plane_range = Range::range(
		position2.dot(normal2),
		-INFINITY,
	);
	let mut times = circle_range.linear_overlap(
		&plane_range,
		movement2.dot(normal2) - movement1.dot(normal2),
	);
	times = times.intersect(&Range::range(0.0, 1.0));
	if !times.is_empty() {
		Some(Collision {
			times,
			position: start_nearest + movement1.scale(times.min()),
			normal: -normal2,
		})
	} else { None }
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

	#[test]
	fn check_collide_sphere_with_plane() {
		{ // Two spheres moving toward eachother.
			let hit = collide_sphere_with_plane(
				1.0,
				&Vec3::new(1.0, 1.0, 1.0),
				&Vec3::new(0.0, -2.0, 0.0),
				&Vec3::new(2.0, -1.0, 5.0),
				&Vec3::y(),
				&Vec3::new(1.0, 0.0, 1.0),
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			println!("hit: {:?}", hit);
			assert!((hit.position - Vec3::new(1.0, -1.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, -1.0, 0.0)).magnitude() < EPSILON);
		}
	}
}
