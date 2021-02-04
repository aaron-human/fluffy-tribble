use crate::physics_system::PhysicsSystem;
use crate::types::EntityHandle;
use crate::force::Force;

use core::fmt::Debug;
use downcast_rs::{Downcast, impl_downcast};

/// A way to send forces into the system that are applied to each object separately (i.e. rather than applying them to pairs of colliding pairs or anything else).
/// This mainly intended to implement gravity, thought it could apply other things too (i.e. springs).
pub trait UnaryForceGenerator : Downcast + Debug {
	/// The function to decide force based on the given Entity.
	fn make_force(&mut self, dt : f32, physics : &mut PhysicsSystem, entity : EntityHandle) -> Force;
}

impl_downcast!(UnaryForceGenerator);
