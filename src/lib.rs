//! A simple physics engine, so I can experiment.
//!
//! ## Implementation Notes
//!
//! * All collision detection is continuous; it tries to analytically find the time of collision.
//! * All collision detection is also linearized; so it calculates the starting and ending position and assumes the objects take a linear path between them. As long as time steps are kept small, this should be a good enough approximation (even for rotating bodies).
///!

mod consts;
mod types;
pub use types::{EntityHandle, ColliderHandle, UnaryForceGeneratorHandle};
mod range;

mod orientation;
pub use orientation::Orientation;
mod entity;
pub use entity::Entity;
mod collider;
pub use collider::{Collider, ColliderType};
mod null_collider;
pub use null_collider::NullCollider;
mod sphere_collider;
pub use sphere_collider::SphereCollider;
mod plane_collider;
pub use plane_collider::PlaneCollider;
mod collider_wrapper;
pub use collider_wrapper::ColliderWrapper;
mod collision;

mod force;
pub use force::Force;

mod physics_system;
pub use physics_system::PhysicsSystem;

mod unary_force_generator;
pub use unary_force_generator::UnaryForceGenerator;
mod gravity_generator;
pub use gravity_generator::GravityGenerator;
