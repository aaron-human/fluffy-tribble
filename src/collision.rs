use crate::types::Vec3;
use crate::range::Range;

/// A structure for storing collision information.
#[derive(Debug)]
pub struct Collision {
	/// The range of times when the collision happened. Will be in the range 0.0 (meaning the very start) and 1.0 (the very end). May also be empty, signaling there is no collision.
	pub times : Range,
	/// The position of the hit. Will be zero-length if there is no hit.
	pub position : Vec3,
	/// The normal of the hit (pointing off the first object). Will be zero-length if there is no hit.
	pub normal : Vec3,
}

impl Collision {
	/// Check if this represents a collision.
	pub fn is_valid(&self) -> bool {
		!self.times.is_empty()
	}
}

/// Detect when and where a point hits a sphere (if ever).
pub fn collide_sphere_with_sphere(radius1 : f32, center1 : Vec3, movement1 : Vec3, radius2 : f32, center2 : Vec3, movement2 : Vec3) -> Collision {
	let dv = movement1 - movement2;
	let dc = center1 - center2;
	let radius = radius1 + radius2;
	let mut collision = Collision {
		times: Range::quadratic_zeros(
			dv.dot(&dv),
			2.0 * dv.dot(&dc),
			dc.dot(&dc) - radius * radius,
		).intersect(&Range::range(0.0, 1.0)),
		position: Vec3::zeros(),
		normal: Vec3::zeros(),
	};
	if !collision.times.is_empty() {
		collision.position = (
			(center1 + movement1.scale(collision.times.min())) * radius2 +
			(center2 + movement2.scale(collision.times.min())) * radius1
		).scale(1.0 / radius);
		collision.normal = (collision.position - center1).normalize();
	}
	collision
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
				Vec3::new(1.0, 1.0, 1.0),
				Vec3::new(2.0, 0.0, 0.0),
				1.0,
				Vec3::new(5.0, 1.0, 1.0),
				Vec3::new(-2.0, 0.0, 0.0),
			);
			assert!(hit.is_valid());
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(3.0, 1.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(1.0, 0.0, 0.0)).magnitude() < EPSILON);
		}
		{ // Two spheres moving away from eachother.
			let hit = collide_sphere_with_sphere(
				1.0,
				Vec3::new(1.0, 1.0, 1.0),
				Vec3::new(-2.0, 0.0, 0.0),
				1.0,
				Vec3::new(4.0, 1.0, 1.0),
				Vec3::new(2.0, 0.0, 0.0),
			);
			assert!(!hit.is_valid());
		}
		{ // Two spheres moving perpendicular to eachother.
			let hit = collide_sphere_with_sphere(
				1.0,
				Vec3::new(1.0, 1.0, 1.0),
				Vec3::new(2.0, 0.0, 0.0),
				1.0,
				Vec3::new(4.0, 0.0, 1.0),
				Vec3::new(0.0, 2.0, 0.0),
			);
			assert!(hit.is_valid());
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			assert!((hit.position - Vec3::new(3.0, 1.0, 1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(1.0, 0.0, 0.0)).magnitude() < EPSILON);
		}
		{ // Two spheres moving toward eachother, but different sized
			let hit = collide_sphere_with_sphere(
				1.0,
				Vec3::new(-1.0, 1.0, -1.0),
				Vec3::new( 0.0, 2.0,  0.0),
				2.0,
				Vec3::new(-1.0, 6.0, -1.0),
				Vec3::new( 0.0, -2.0,  0.0),
			);
			assert!(hit.is_valid());
			assert!((hit.times.min() - 0.5).abs() < EPSILON);
			println!("{:?}", hit.position);
			assert!((hit.position - Vec3::new(-1.0, 3.0, -1.0)).magnitude() < EPSILON);
			assert!((hit.normal - Vec3::new(0.0, 1.0, 0.0)).magnitude() < EPSILON);
		}
	}
}
