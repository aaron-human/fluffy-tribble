use downcast_rs::{Downcast, impl_downcast};

/// The internal representation of an arbitrary collider.
/// This generally will have NO data hiding to keep things simple.
pub trait InternalCollider : Downcast {
	// Nothing for now.
}

impl dyn InternalCollider {
	// Nothing for now.
}

impl_downcast!(InternalCollider);

/// The public representation of an arbitrary collider.
pub trait Collider : Downcast {
	// Nothing for now.
}

impl dyn Collider {
	// Nothing for now.
}

impl_downcast!(Collider);
