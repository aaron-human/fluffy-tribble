use crate::types::Vec3;

/// A simple structure for storing a force to be applied.
pub struct Force {
	/// The force vector.
	pub force : Vec3,
	/// The position to apply the force at (in world coordinates).
	pub position : Vec3,
}

impl Force {
	/// Creates a new instance by consuming the given vectors.
	pub fn new(force : Vec3, position : Vec3) -> Force {
		Force { force, position }
	}
}
