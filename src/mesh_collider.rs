use crate::consts::EPSILON;
use crate::types::{Vec3, Mat3, EntityHandle};
use crate::collider::{ColliderType, Collider, InternalCollider};
use crate::orientation::Orientation;

/// The internal representation of a mesh collider.
#[derive(Debug)]
pub struct InternalMeshCollider {
	/// The entity that this is linked to (if any).
	entity : Option<EntityHandle>,

	/// The position of mesh origin.
	///
	/// This is in the parent entity's local space.
	pub position : Vec3,

	/// The vertices.
	pub vertices : Vec<Vec3>,
	/// The faces as indices into the `vertices` property.
	pub faces : Vec<Vec<usize>>,
	/// The lines segments as indices into the `vertices` property.
	pub edges : Vec<(usize, usize)>,

	/// The restituion coefficient.
	pub restitution_coefficient : f32,

	/// The ratio used to decide whether to use static friction or dynamic friction.
	pub friction_threshold : f32,

	/// The static friction coefficient. Should always at or between 0.0 and 1.0.
	pub static_friction_coefficient : f32,

	/// The dynamic friction coefficient. Should always at or between 0.0 and 1.0.
	pub dynamic_friction_coefficient : f32,
}

impl InternalMeshCollider {
	/// Creates a new instance.
	pub fn new_from(source : &MeshCollider) -> Result<Box<dyn InternalCollider>, ()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			Ok(Box::new(InternalMeshCollider {
				entity: None,
				position: source.position.clone(),
				vertices: source.vertices.clone(),
				faces: source.faces.clone(),
				edges: source.edges.clone(),
				restitution_coefficient: source.restitution_coefficient,
				friction_threshold: source.friction_threshold,
				static_friction_coefficient: source.static_friction_coefficient,
				dynamic_friction_coefficient: source.dynamic_friction_coefficient,
			}))
		}
	}

	/// Makes a MeshCollider copying this instance's values.
	pub fn make_pub(&self) -> MeshCollider {
		MeshCollider {
			entity: self.entity.clone(),
			position: self.position.clone(),
			vertices: self.vertices.clone(),
			faces: self.faces.clone(),
			edges: self.edges.clone(),
			restitution_coefficient: self.restitution_coefficient,
			friction_threshold: self.friction_threshold,
			static_friction_coefficient: self.static_friction_coefficient,
			dynamic_friction_coefficient: self.dynamic_friction_coefficient,
		}
	}

	/// Updates from the passed in Entity object.
	pub fn update_from(&mut self, source : &MeshCollider) -> Result<(),()> {
		if !source.is_valid() {
			Err(()) // TODO: An error type.
		} else {
			self.position = source.position;
			self.vertices = source.vertices.clone();
			self.faces = source.faces.clone();
			self.edges = source.edges.clone();
			self.restitution_coefficient = source.restitution_coefficient;
			self.friction_threshold = source.friction_threshold;
			self.static_friction_coefficient = source.static_friction_coefficient;
			self.dynamic_friction_coefficient = source.dynamic_friction_coefficient;
			Ok(())
		}
	}

	/// Returns all the verticies after being moved into world space. The passed in orientation should be from the owning Entity.
	pub fn vertices_in_world(&self, orientation : &Orientation) -> Vec<Vec3> {
		let mut transformed = Vec::with_capacity(self.vertices.len());
		for vertex in &self.vertices {
			transformed.push(orientation.position_into_world(&(self.position + vertex)));
		}
		transformed
	}
}

impl InternalCollider for InternalMeshCollider {
	/// The specific type.
	fn get_type(&self) -> ColliderType { ColliderType::MESH }

	/// Sets the entity this is attached to, returning the previous one.
	fn set_entity(&mut self, handle : Option<EntityHandle>) -> Option<EntityHandle> {
		let old = self.entity;
		self.entity = handle;
		old
	}

	/// Retrieves the stored entity handle that this is attached to.
	fn get_entity(&mut self) -> Option<EntityHandle> { self.entity }

	/// Gets the center of mass for this collider.
	/// This is relative to this collider's owning/linked/attached entity.
	/// This IS NOT relative to this collider's "position" property.
	fn get_local_center_of_mass(&self) -> Vec3 { self.position }

	fn get_mass(&self) -> f32 { 0.0 }

	fn get_moment_of_inertia_tensor(&self) -> Mat3 { Mat3::zeros() }

	fn get_restitution_coefficient(&self) -> f32 { self.restitution_coefficient }

	fn get_friction_threshold(&self) -> f32 { self.friction_threshold }

	fn get_static_friction_coefficient(&self) -> f32 { self.static_friction_coefficient }

	fn get_dynamic_friction_coefficient(&self) -> f32 { self.dynamic_friction_coefficient }
}

/// A copy of all of the publicly-accessible properties of a mesh collider.
#[derive(Debug)]
pub struct MeshCollider {
	/// The entity, if there is one. This is NOT copied back into InternalSphereCollider, hence why it's not "pub".
	///
	/// Defaults to None.
	entity : Option<EntityHandle>,

	/// The position of the collider's origin relative to the parent entity's origin (in the parent entity's local space).
	///
	/// Defaults to origin.
	pub position : Vec3,

	/// The points that make up the mesh.
	///
	/// Should never contain any duplicates.
	///
	/// Defaults to empty.
	vertices : Vec<Vec3>,
	/// The faces as indices into the `vertices` property. May contain duplicates.
	///
	/// Defaults to empty.
	faces : Vec<Vec<usize>>,
	/// The lines segments as indices into the `vertices` property.
	///
	/// Should never contain any duplicates. Lower indicies are first in the tuples.
	///
	/// Defaults to empty.
	edges : Vec<(usize, usize)>,

	/// The restituion coefficient.
	///
	/// Defaults to one.
	pub restitution_coefficient : f32,

	/// The ratio used to threshold whether to use static or dynamic friction for a given collision.
	///
	/// Defaults to `1.0`.
	pub friction_threshold : f32,

	/// The static friction coefficient. Should always at or between 0.0 and 1.0.
	///
	/// Defaults to `0.25`.
	pub static_friction_coefficient : f32,

	/// The dynamic friction coefficient. Should always at or between 0.0 and 1.0.
	///
	/// Defaults to `0.3`.
	pub dynamic_friction_coefficient : f32,
}

impl MeshCollider {
	/// Creates an instance with all values at default.
	///
	/// Starts with no geometry.
	pub fn new() -> MeshCollider {
		MeshCollider {
			entity: None,
			position: Vec3::zeros(),
			vertices: Vec::new(),
			faces: Vec::new(),
			edges: Vec::new(),
			restitution_coefficient: 1.0,
			friction_threshold: 0.25,
			static_friction_coefficient: 1.0,
			dynamic_friction_coefficient: 0.3,
		}
	}

	/// Adds a face to the mesh.
	///
	/// The points must be coplanar, and should represent a convex polygon on that plane.
	///
	/// It's more efficient to try and merge faces that are coplanar (into convex polygons) rather than specifying the triangulated faces into this method separately.
	pub fn add_face(&mut self, points : &Vec<Vec3>) {
		assert!(3 <= points.len(), "Not enough points to form a face.");
		// Make sure the points are coplanar and form a convex polygon.
		let normal = (points[1] - points[0]).cross(&(points[2] - points[0])).normalize();
		for index in 0..points.len() {
			let first  = &points[index];
			let second = &points[if index+1 < points.len() { index+1 } else { index+1 - points.len() }];
			let third  = &points[if index+2 < points.len() { index+2 } else { index+2 - points.len() }];
			let current_normal = (second - first).cross(&(third - first)).normalize();
			assert!((current_normal.dot(&normal) - 1.0).abs() < EPSILON, "Points not coplanar or not convex.");
		}
		// TODO: Could try deduplicating faces too...
		// TODO: Could also try merging faces if on same plane and sharing edges... Though that's fairly non-trivial.
		// Then start adding things in.
		let point_indices = self.add_points(points);
		for index in 0..point_indices.len() {
			self.add_edge(
				point_indices[index],
				point_indices[if index+1 < point_indices.len() { index+1 } else { 0 }],
			);
		}
		// Finally add the face.
		self.faces.push(point_indices);
	}

	/// Stores a single edge using the points at the given indices in the internal `points` vector.
	///
	/// This exists mainly to prevent duplicate edges from being stored.
	fn add_edge(&mut self, mut index1 : usize, mut index2 : usize) {
		// To keep deduplication easy, keep the first index smaller.
		if index1 > index2 {
			let temp = index1;
			index1 = index2;
			index2 = temp;
		}
		for (existing1, existing2) in &self.edges {
			if index1 == *existing1 && index2 == *existing2 {
				return // Give up immediately if see the edge already exists.
			}
		}
		// If didn't find a duplicate, then add the edge.
		self.edges.push((index1, index2));
	}

	/// Stores the given list of points into the internal `points` vector and returns a vec of the indices.
	///
	/// Mainly this deduplicates the points with any that already exist.
	fn add_points(&mut self, points : &Vec<Vec3>) -> Vec<usize> {
		let mut indices = Vec::with_capacity(points.len());
		for point in points {
			let mut found = false;
			for (index, existing) in self.vertices.iter().enumerate() {
				if (existing - point).magnitude() < EPSILON {
					found = true;
					indices.push(index);
					break;
				}
			}
			if !found {
				indices.push(self.vertices.len());
				self.vertices.push(point.clone());
			}
		}
		indices
	}

	/// The number of faces currently stored in this instance.
	pub fn face_count(&self) -> usize { self.faces.len() }
	/// The number of (unique) edges currently stored in this instance.
	pub fn edge_count(&self) -> usize { self.edges.len() }
	/// The number of (unique) vertices currently stored in this instance.
	pub fn vertex_count(&self) -> usize { self.vertices.len() }

	// TODO? Some functions to grab triangles/edges/vertices?
	// TODO? A function to clear the current geometry?

	/// If this is in a valid state.
	pub fn is_valid(&self) -> bool {
		3 <= self.vertices.len() && 1 <= self.faces.len() && 1 <= self.edges.len()
	}
}

impl Collider for MeshCollider {
	fn get_type(&self) -> ColliderType { ColliderType::MESH }

	fn get_entity(&self) -> Option<EntityHandle> { self.entity }

	fn get_center_of_mass(&self) -> Vec3 { self.position }
}


#[cfg(test)]
mod tests {
	use super::*;

	/// Verify can create and add faces to a mesh collider.
	#[test]
	fn check_create_mesh() {
		let mut collider = MeshCollider::new();
		assert_eq!(collider.is_valid(), false);
		assert_eq!(collider.face_count(), 0);
		assert_eq!(collider.edge_count(), 0);
		assert_eq!(collider.vertex_count(), 0);

		collider.add_face(&vec![
			Vec3::new( 0.0, 1.0, 0.0),
			Vec3::new( 1.0,-1.0, 0.0),
			Vec3::new(-1.0,-1.0, 0.0),
		]);
		assert_eq!(collider.is_valid(), true);
		assert_eq!(collider.face_count(), 1);
		assert_eq!(collider.edge_count(), 3);
		assert_eq!(collider.vertex_count(), 3);

		// Add a triangle that shares a single point.
		collider.add_face(&vec![
			Vec3::new( 2.0, 1.0, 0.0),
			Vec3::new( 1.0,-1.0, 0.0),
			Vec3::new( 3.0,-1.0, 0.0),
		]);
		assert_eq!(collider.is_valid(), true);
		assert_eq!(collider.face_count(), 2);
		assert_eq!(collider.edge_count(), 6);
		assert_eq!(collider.vertex_count(), 5);

		// Add a triangle that shares an edge.
		collider.add_face(&vec![
			Vec3::new( 0.0,-1.0, 1.0),
			Vec3::new( 1.0,-1.0, 0.0),
			Vec3::new(-1.0,-1.0, 0.0),
		]);
		assert_eq!(collider.is_valid(), true);
		assert_eq!(collider.face_count(), 3);
		assert_eq!(collider.edge_count(), 8);
		assert_eq!(collider.vertex_count(), 6);

		// Add a triangle that shares two edges.
		collider.add_face(&vec![
			Vec3::new( 0.0, 1.0, 0.0),
			Vec3::new( 1.0,-1.0, 0.0),
			Vec3::new( 2.0, 1.0, 0.0),
		]);
		assert_eq!(collider.is_valid(), true);
		assert_eq!(collider.face_count(), 4);
		assert_eq!(collider.edge_count(), 9);
		assert_eq!(collider.vertex_count(), 6);
	}
}
