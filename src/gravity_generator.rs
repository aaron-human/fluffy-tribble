use crate::types::{Vec3, EntityHandle};
use crate::physics_system::PhysicsSystem;
use crate::force::Force;
use crate::unary_force_generator::UnaryForceGenerator;

/// A force generator for simulating (simple, mono-direcitonal) gravity.
#[derive(Debug)]
pub struct GravityGenerator {
	/// The direction and magnitude of gravitational acceleration.
	pub acceleration : Vec3,
}

impl GravityGenerator {
	/// Creates a new gravitational force.
	pub fn new(acceleration : Vec3) -> GravityGenerator {
		GravityGenerator { acceleration }
	}
}

impl UnaryForceGenerator for GravityGenerator {
	fn make_force(&mut self, _dt : f32, physics : &mut PhysicsSystem, handle : EntityHandle) -> Force {
		let entity = physics.get_entity(handle).unwrap();
		Force::new(
			self.acceleration.scale(entity.get_last_total_mass()),
			entity.position,
		)
	}
}
