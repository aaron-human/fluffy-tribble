use std::f32::INFINITY;

use crate::consts::EPSILON;
use crate::types::{Vec3};
use crate::range::Range;
use crate::collider::{ColliderType, InternalCollider};
use crate::sphere_collider::{InternalSphereCollider};
use crate::plane_collider::{InternalPlaneCollider};
use crate::mesh_collider::{InternalMeshCollider};
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
pub fn collide(collider1 : &Box<dyn InternalCollider>, start1 : &Orientation, end1 : &Orientation, collider2 : &Box<dyn InternalCollider>, start2 : &Orientation, end2 : &Orientation, debug : &mut Vec<String>) -> Option<Collision> {
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
	// I don't think it makes sense to detect when two (infinite) planes are colliding...
	if ColliderType::SPHERE == collider1.get_type() && ColliderType::MESH == collider2.get_type() {
		let sphere = collider1.downcast_ref::<InternalSphereCollider>().unwrap();
		let sphere_start_position = start1.position_into_world(&sphere.center);
		let sphere_end_position = end1.position_into_world(&sphere.center);

		let mesh  = collider2.downcast_ref::<InternalMeshCollider>().unwrap();
		let mesh_start_position = start2.position_into_world(&mesh.position);
		let mesh_end_position = end2.position_into_world(&mesh.position);

		return collide_sphere_with_mesh(
			sphere.radius,
			&sphere_start_position,
			&(sphere_end_position - sphere_start_position),
			&mesh.vertices_in_world(&start2),
			&mesh.edges,
			&mesh.faces,
			&(mesh_end_position - mesh_start_position),
		);
	}
	if ColliderType::MESH == collider1.get_type() && ColliderType::SPHERE == collider2.get_type() {
		let mesh  = collider1.downcast_ref::<InternalMeshCollider>().unwrap();
		let mesh_start_position = start1.position_into_world(&mesh.position);
		let mesh_end_position = end1.position_into_world(&mesh.position);

		let sphere = collider2.downcast_ref::<InternalSphereCollider>().unwrap();
		let sphere_start_position = start2.position_into_world(&sphere.center);
		let sphere_end_position = end2.position_into_world(&sphere.center);

		let collision_option = collide_sphere_with_mesh(
			sphere.radius,
			&sphere_start_position,
			&(sphere_end_position - sphere_start_position),
			&mesh.vertices_in_world(&start1),
			&mesh.edges,
			&mesh.faces,
			&(mesh_end_position - mesh_start_position),
		);
		// Must negate the normal as the sphere is the second collider.
		if let Some(mut collision) = collision_option {
			collision.normal *= -1.0;
			return Some(collision);
		} else {
			return None
		}
	}

	if ColliderType::MESH == collider1.get_type() && ColliderType::PLANE == collider2.get_type() {
		let mesh  = collider1.downcast_ref::<InternalMeshCollider>().unwrap();

		let plane = collider2.downcast_ref::<InternalPlaneCollider>().unwrap();
		let plane_start_position = start2.position_into_world(&plane.position);
		let plane_end_position = end2.position_into_world(&plane.position);

		return collide_mesh_with_plane(
			&mesh.vertices,
			&mesh.position,
			start1,
			end1,
			&plane_start_position,
			&plane_end_position,
			&plane.normal,
			debug,
		);
	}

	if ColliderType::PLANE == collider1.get_type() && ColliderType::MESH == collider2.get_type() {

		let plane = collider1.downcast_ref::<InternalPlaneCollider>().unwrap();
		let plane_start_position = start1.position_into_world(&plane.position);
		let plane_end_position = end1.position_into_world(&plane.position);

		let mesh  = collider2.downcast_ref::<InternalMeshCollider>().unwrap();

		let collision_option = collide_mesh_with_plane(
			&mesh.vertices,
			&mesh.position,
			start2,
			end2,
			&plane_start_position,
			&plane_end_position,
			&plane.normal,
			debug,
		);
		// Must negate the normal as the mesh is the second collider.
		if let Some(mut collision) = collision_option {
			collision.normal *= -1.0;
			return Some(collision);
		} else {
			return None
		}
	}

	None
}

/// A helper to get the time of collision for a sphere overlapping a plane.
fn sphere_plane_overlap_time(radius1 : f32, center1 : &Vec3, movement1 : &Vec3, position2 : &Vec3, normal2 : &Vec3, movement2 : &Vec3, infinite_backdrop : bool) -> Range {
	let start_nearest  = center1 + normal2.scale(-radius1);
	let start_farthest = center1 + normal2.scale( radius1);
	let circle_range = Range::range(
		start_nearest.dot(normal2),
		start_farthest.dot(normal2),
	);
	let plane_value = position2.dot(normal2);
	let plane_range = Range::range(
		plane_value,
		if infinite_backdrop { -INFINITY } else { plane_value },
	);
	circle_range.linear_overlap(
		&plane_range,
		movement2.dot(normal2) - movement1.dot(normal2),
	)
}

/// Collide a sphere with an inifinite plane.
pub fn collide_sphere_with_plane(radius1 : f32, center1 : &Vec3, movement1 : &Vec3, position2 : &Vec3, normal2 : &Vec3, movement2 : &Vec3) -> Option<Collision> {
	let times = sphere_plane_overlap_time(
		radius1, center1, movement1,
		position2, normal2, movement2,
		true,
	).intersect(&Range::range(0.0, 1.0));
	if !times.is_empty() {
		let start_nearest  = center1 + normal2.scale(-radius1); // TODO: Pass this along somehow?
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

/// Detect when and where a sphere intersects the an infinite line.
pub fn collide_sphere_with_line(radius1 : f32, center1: &Vec3, movement1 : &Vec3, start2 : &Vec3, direction2 : &Vec3, movement2 : &Vec3) -> Option<Collision> {
	let direction = direction2.normalize();
	let movement = movement1 - movement2;
	let a = (center1 - start2).cross(&direction);
	let b = movement.cross(&direction);
	let times = Range::quadratic_zeros(
		b.dot(&b),
		2.0 * a.dot(&b),
		a.dot(&a) - radius1 * radius1,
	).intersect(&Range::range(0.0, 1.0));
	if !times.is_empty() {
		let line_position = start2 + movement2.scale(times.min());
		let center_position = center1 + movement1.scale(times.min());
		let along_length = (center_position - line_position).dot(&direction);
		let position = line_position + direction.scale(along_length);
		let normal = (position - center1).normalize();
		Some(Collision {
			times,
			position,
			normal,
		})
	} else { None }
}

/// Detect when and where a sphere intersects the middle of a line segment.
///
/// This isn't full line-segment vs sphere collision, as it lacks the collision checking for the end points. This is intentional, as this will only be used as a part of plane collision handling.
pub fn collide_sphere_with_mid_line_segment(radius1 : f32, center1: &Vec3, movement1 : &Vec3, start2 : &Vec3, end2 : &Vec3, movement2 : &Vec3) -> Option<Collision> {
	let length = end2 - start2;
	if let Some(hit) = collide_sphere_with_line(radius1, center1, movement1, start2, &length, movement2) {
		let hit_movement = movement2.scale(hit.times.min());
		let hit_start = start2 + hit_movement;
		let hit_end = end2 + hit_movement;
		if (((hit_start - hit.position).magnitude() + (hit_end - hit.position).magnitude()) - length.magnitude()).abs() < EPSILON {
			Some(hit)
		} else { None }
	} else { None }
}

/// Collide a sphere with a flat polygon bounded by convex line segments.
///
/// The passed in corners must be in order so that they progress in a convex manor around the edge of the polygon. They should all be coplanar.
///
/// **WARNING:** This isn't full collision handling between a sphere and the surface. It lacks the edge and corner collision handling. This is intentional as this is just a building-block to generate that sort of full-scale collision handling.
pub fn collide_sphere_with_polygon_surface(radius1: f32, center1: &Vec3, movement1: &Vec3, corners2 : &Vec<Vec3>, movement2 : &Vec3) -> Option<Collision> {
	assert!(3 <= corners2.len());
	let normal = (corners2[1] - corners2[0]).cross(&(corners2[2] - corners2[0])).normalize();
	let plane_start_position = corners2[0].clone();
	let times = sphere_plane_overlap_time(
		radius1, center1, movement1,
		&plane_start_position, &normal, movement2,
		false,
	).intersect(&Range::range(0.0, 1.0));
	if !times.is_empty() {
		let sphere_hit_position = center1 + movement1.scale(times.min());
		let total_plane_movement = movement2.scale(times.min());
		let plane_hit_position = plane_start_position + total_plane_movement;
		let hit_position = sphere_hit_position - normal.scale((sphere_hit_position - plane_hit_position).dot(&normal));
		let normal = (hit_position - sphere_hit_position).normalize();
		// Then verify the hit_position is in the polygon.
		let mut expected_sign : f32 = 0.0;
		for index in 0..corners2.len() {
			let first = corners2[index] + total_plane_movement;
			let second = corners2[if index+1 < corners2.len() { index + 1 } else { 0 }] + total_plane_movement;
			let sign = (hit_position - first).cross(&(second - first)).dot(&normal);
			// A zero 'sign' means that hit_position is basically on the line from first to second, which counts.
			// So move on immediately.
			if sign.abs() < EPSILON {
				continue;
			}
			// At this point defintiely have a sign, so compare it.
			if 0.0 == expected_sign {
				expected_sign = sign.signum();
			} else if expected_sign != sign.signum() {
				// This means that the point is on the wrong side of at least one of the polygon edges, so it's not intersecting. Short circuit out immediately.
				return None
			}
		}
		// If made it past all that, then the collision is valid.
		Some(Collision {
			times,
			position: hit_position,
			normal,
		})
	} else { None }
}

/// A helper object to grab the earliest collision of a series of passed in collisions.
struct EarliestCollisionAccumulator {
	/// The current earliest.
	earliest : Option<Collision>,
	/// The time of the current earliest.
	earliest_time : f32,
}

impl EarliestCollisionAccumulator {
	/// Creates a new instance with no earliest set.
	pub fn new() -> EarliestCollisionAccumulator {
		EarliestCollisionAccumulator {
			earliest: None,
			earliest_time: INFINITY,
		}
	}

	/// Considers storing the given possible collision.
	pub fn consider(&mut self, possible : Option<Collision>) {
		if let Some(collision) = possible {
			if collision.times.min() < self.earliest_time {
				println!("Got earliest!");
				self.earliest_time = collision.times.min();
				self.earliest = Some(collision);
			} else {
				println!("Got later...");
			}
		} else {
			println!("Got nothing!");
		}
	}

	/// Yields the closest found.
	pub fn get(self) -> Option<Collision> { self.earliest }
}

/// Collides a sphere against a mesh.
pub fn collide_sphere_with_mesh(radius1 : f32, center1: &Vec3, movement1 : &Vec3, vertices2 : &Vec<Vec3>, edges2 : &Vec<(usize, usize)>, faces2 : &Vec<Vec<usize>>, movement2 : &Vec3) -> Option<Collision> {
	let mut accumulator = EarliestCollisionAccumulator::new();
	// First check all the corners.
	for vertex in vertices2 {
		println!("vertex");
		accumulator.consider(collide_sphere_with_sphere(
			radius1, center1, movement1,
			0.0, vertex, movement2,
		));
	}
	// Then check all the edges.
	for (index1, index2) in edges2 {
		println!("edge");
		accumulator.consider(collide_sphere_with_mid_line_segment(
			radius1, center1, movement1,
			&vertices2[*index1], &vertices2[*index2], movement2,
		));
	}
	// Then check all the planes.
	for face in faces2 {
		println!("face");
		let mut corners = Vec::with_capacity(face.len());
		for index in face {
			corners.push(vertices2[*index].clone()); // TODO: Make this more efficient.
		}
		accumulator.consider(collide_sphere_with_polygon_surface(
			radius1, center1, movement1,
			&corners, movement2,
		));
	}
	accumulator.get()
}

struct _MeshCollisionInfo {
	start_position : Vec3,
	end_position : Vec3,

	start_distance : f32,
	end_distance : f32,
}

/// Collides a mesh with an (infinite) plane.
pub fn collide_mesh_with_plane(mesh_vertices : &Vec<Vec3>, mesh_position : &Vec3, mesh_start_orientation : &Orientation, mesh_end_orientation : &Orientation, plane_start_position : &Vec3, plane_end_position : &Vec3, plane_normal : &Vec3, debug : &mut Vec<String>) -> Option<Collision> {
	let mut start_distances = Range::empty();
	let mut end_distances = Range::empty();
	let mut calculated  = Vec::new();
	let mut debugs = Vec::new();
	for vertex in mesh_vertices {
		let internal_vertex_position = mesh_position + vertex;
		let mesh_start_position = mesh_start_orientation.position_into_world(&internal_vertex_position);
		let mesh_end_position = mesh_end_orientation.position_into_world(&internal_vertex_position);

		let start_distance = (mesh_start_position - plane_start_position).dot(plane_normal);
		let end_distance   = (mesh_end_position   - plane_end_position).dot(plane_normal);

		debugs.push(format!("point {:?} => {:?} {:?}", vertex, start_distance, end_distance));

		start_distances = start_distances.contain(&Range::single(start_distance));
		end_distances   = end_distances.contain(&Range::single(end_distance));

		calculated.push(_MeshCollisionInfo {
			start_position: mesh_start_position,
			end_position: mesh_end_position,

			start_distance, end_distance,
		});
	}

	let times = Range::range(-INFINITY, 0.0).linear_overlap(
		&start_distances,
		end_distances.min() - start_distances.min()
	).intersect(&Range::range(0.0, 1.0));

	if !times.is_empty() {
		let mut closest_start_position_sum = Vec3::zeros();
		let mut closest_start_position_count : f32 = 0.0;
		let mut closest_end_position_sum = Vec3::zeros();
		let mut closest_end_position_count : f32 = 0.0;
		let start_epsilon = start_distances.size() * 0.01;// Apparently the standard EPSILON is a bit too small...
		let end_epsilon = end_distances.size() * 0.01;// Apparently the standard EPSILON is a bit too small...
		for info in calculated {
			if start_epsilon > (info.start_distance - start_distances.min()).abs() {
				closest_start_position_sum += info.start_position;
				closest_start_position_count += 1.0;
			}
			if end_epsilon > (info.end_distance - end_distances.min()).abs() {
				closest_end_position_sum += info.end_position;
				closest_end_position_count += 1.0;
			}
		}
		closest_start_position_sum /= closest_start_position_count;
		closest_end_position_sum /= closest_end_position_count;

		let time = times.min();
		Some(Collision {
			times: times,
			position: closest_start_position_sum * (1.0 - time) + closest_end_position_sum * time,
			normal: -plane_normal,
		})
	} else {
		None
	}
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
			assert!((hit.position - Vec3::new(1.0, -1.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, -1.0, 0.0)).magnitude() < EPSILON);
		}
	}

	#[test]
	fn check_collide_sphere_with_line() {
		{ // The hit case
			let hit = collide_sphere_with_line(
				1.0,
				&Vec3::new(4.0, 3.0, 0.0),
				&Vec3::new(0.0, -2.0, 0.0),

				&Vec3::new(1.0, 1.0, 0.0),
				&Vec3::new(3.0, 0.0, 0.0),
				&Vec3::new(-1.0, 0.0, 0.0),
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(4.0, 1.0, 0.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, -1.0, 0.0)).magnitude() < EPSILON);
		}
		{ // The no hit case
			let hit = collide_sphere_with_line(
				1.0,
				&Vec3::new(4.0, 3.0, 0.0),
				&Vec3::new(0.0, 0.0, 1.0),

				&Vec3::new(1.0, 1.0, 0.0),
				&Vec3::new(3.0, 0.0, 0.0),
				&Vec3::new(-1.0, -1.0, 0.0),
			);
			assert!(hit.is_none());
		}
	}

	#[test]
	fn check_collide_sphere_with_mid_line_segment() {
		{ // The hit case
			let hit = collide_sphere_with_mid_line_segment(
				1.0,
				&Vec3::new(4.0, 3.0, 0.0),
				&Vec3::new(0.0, -2.0, 0.0),

				&Vec3::new(1.0, 1.0, 0.0),
				&Vec3::new(6.0, 1.0, 0.0),
				&Vec3::new(-1.0, 0.0, 0.0),
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(4.0, 1.0, 0.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, -1.0, 0.0)).magnitude() < EPSILON);
		}
		{ // The no hit case
			let hit = collide_sphere_with_mid_line_segment(
				1.0,
				&Vec3::new(4.0, 3.0, 0.0),
				&Vec3::new(0.0, 0.0, 1.0),

				&Vec3::new(1.0, 1.0, 0.0),
				&Vec3::new(-1.0, 1.0, 0.0),
				&Vec3::new(-1.0, -1.0, 0.0),
			);
			assert!(hit.is_none());
		}
	}

	#[test]
	fn check_collide_sphere_with_polygon_surface() {
		{ // The hit case
			let hit = collide_sphere_with_polygon_surface(
				1.0,
				&Vec3::new(0.0, 0.0, 3.0),
				&Vec3::new(0.0, 0.0, -2.0),

				&vec![
					Vec3::new(0.0, 1.0, 1.0),
					Vec3::new(-1.0, -1.0, 1.0),
					Vec3::new( 1.0, -1.0, 1.0),
				],
				&Vec3::new(-1.0, 0.0, 0.0),
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(0.0, 0.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, 0.0, -1.0)).magnitude() < EPSILON);
		}
		{ // The no hit case. Make this just slightly off one of the edges.
			let hit = collide_sphere_with_polygon_surface(
				1.0,
				&Vec3::new(1.0, 0.0, 3.0),
				&Vec3::new(0.0, 0.0, -2.0),

				&vec![
					Vec3::new(0.0, 1.0, 1.0),
					Vec3::new(-1.0, -1.0, 1.0),
					Vec3::new( 1.0, -1.0, 1.0),
				],
				&Vec3::new(-1.0, -1.0, 0.0),
			);
			assert!(hit.is_none());
		}
	}

	#[test]
	fn check_collide_sphere_with_mesh() {
		let vertices = vec![
			Vec3::new(0.0, 1.0, 1.0),
			Vec3::new(-1.0, -1.0, 1.0),
			Vec3::new( 1.0, -1.0, 1.0),
		];
		let edges = vec![
			(0, 1),
			(1, 2),
			(2, 0),
		];
		let faces = vec![
			vec![
				0, 1, 2,
			],
		];
		let movement = Vec3::zeros();
		{ // The hit a corner.
			let hit = collide_sphere_with_mesh(
				1.0,
				&Vec3::new(0.0, 3.0, 1.0),
				&Vec3::new(0.0,-2.0, 0.0),

				&vertices,
				&edges,
				&faces,
				&movement,
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(0.0, 1.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, -1.0, 0.0)).magnitude() < EPSILON);
		}
		{ // The hit an edge.
			let hit = collide_sphere_with_mesh(
				1.0,
				&Vec3::new(0.0, -3.0, 1.0),
				&Vec3::new(0.0, 2.0, 0.0),

				&vertices,
				&edges,
				&faces,
				&movement,
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(0.0, -1.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, 1.0, 0.0)).magnitude() < EPSILON);
		}
		{ // The hit the flat surface.
			let hit = collide_sphere_with_mesh(
				1.0,
				&Vec3::new(0.5, -0.5, 3.0),
				&Vec3::new(0.0, 0.0, -2.0),

				&vertices,
				&edges,
				&faces,
				&movement,
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(0.5, -0.5, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, 0.0, -1.0)).magnitude() < EPSILON);
		}
		{ // The no hit case.
			println!("Start!");
			let hit = collide_sphere_with_mesh(
				1.0,
				&Vec3::new(0.0, 0.0, 3.0),
				&Vec3::new(0.0, 0.0, -2.0),

				&vertices,
				&edges,
				&faces,
				&Vec3::new(0.0, 4.0, 0.0),
			);
			println!("no hit? {:?}", hit);
			assert!(hit.is_none());
		}
	}


	#[test]
	fn check_collide_mesh_with_plane() {
		let vertices = vec![
			Vec3::new(0.0, 0.0, 0.0),
			Vec3::new(1.0, 1.0, 1.0),
		];
		{ // A clean hit.
			let hit = collide_mesh_with_plane(
				&vertices,
				&Vec3::new(0.0, 0.0, 0.0),
				&Orientation::new(
					&Vec3::new(0.0, 0.0, 0.0),
					&Vec3::zeros(),
					&Vec3::zeros(),
				),
				&Orientation::new(
					&Vec3::new(0.0, 0.0, 2.0),
					&Vec3::zeros(),
					&Vec3::zeros(),
				),
				&Vec3::new(0.0, 0.0, 2.0),
				&Vec3::new(0.0, 0.0, 2.0),
				&Vec3::new(0.0, 0.0,-1.0),
			).unwrap();
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(1.0, 1.0, 2.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, 0.0, 1.0)).magnitude() < EPSILON);
		}
		{ // A miss.
			let hit = collide_mesh_with_plane(
				&vertices,
				&Vec3::new(0.0, 0.0, 0.0),
				&Orientation::new(
					&Vec3::new(0.0, 0.0, 0.0),
					&Vec3::zeros(),
					&Vec3::zeros(),
				),
				&Orientation::new(
					&Vec3::new(0.0, 0.0, 2.0),
					&Vec3::zeros(),
					&Vec3::zeros(),
				),
				&Vec3::new(0.0, 0.0, 2.0),
				&Vec3::new(0.0, 0.0, 8.0),
				&Vec3::new(0.0, 0.0,-1.0),
			);
			assert!(hit.is_none());
		}
		{ // A hit due to being embedded.
			let hit = collide_mesh_with_plane(
				&vertices,
				&Vec3::new(0.0, 0.0, 0.0),
				&Orientation::new(
					&Vec3::new(0.0, 0.0, 0.0),
					&Vec3::zeros(),
					&Vec3::zeros(),
				),
				&Orientation::new(
					&Vec3::new(0.0, 0.0, 2.0),
					&Vec3::zeros(),
					&Vec3::zeros(),
				),
				&Vec3::new(0.0, 0.0,-10.0),
				&Vec3::new(0.0, 0.0,-10.0),
				&Vec3::new(0.0, 0.0,-1.0),
			).unwrap();
			assert!((hit.times.min() - 0.0).abs() < EPSILON);
			assert!((hit.position - Vec3::new(1.0, 1.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, 0.0, 1.0)).magnitude() < EPSILON);
		}
	}
}
