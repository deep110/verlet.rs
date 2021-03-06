//! A lightweight implementation of verlet 2d physics engine in
//! pure rust
//!
//! # Example
//!
//! ```ignore
//! ```

// forbid unsafe code
#![forbid(unsafe_code)]

pub mod behaviors;
pub mod constraints;
pub mod objects;


pub(crate) use behaviors::ParticleBehaviour2D;
pub(crate) use constraints::ParticleConstraint2D;

mod particle2d;
mod physics2d;
mod vector2d;
mod spring;


pub use particle2d::{Particle2D, ParticleKey};
pub use physics2d::VerletPhysics2D;
pub use vector2d::Vector2D;
pub use spring::Spring2D;
pub use objects::VerletObject2D;
