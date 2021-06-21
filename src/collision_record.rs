use crate::types::{EntityHandle, Vec3};

pub struct CollisionRecord {
	/// The first entity in the collision pair.
	pub first_entity : EntityHandle,
	/// The second entity in the collision pair.
	pub second_entity : EntityHandle,
	/// The point where the collision happened.
	pub position : Vec3,
	/// The time when the collision happened. (The time `0.0` is the start of the `step()` call.)
	pub time : f32,
	/// The collision normal. **Points off of the first entity**.
	pub normal : Vec3,

	/// The collision's restitution coefficient.
	pub restitution_coefficient : f32,
	/// The magnitude of the resulting impulse.
	pub impulse_magnitude : f32,
}