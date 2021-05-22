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
mod constraints;
mod particle2d;
mod physics2d;
mod vector2d;

pub(crate) use behaviors::ParticleBehaviour2D;

pub use particle2d::Particle2D;
pub use physics2d::VerletPhysics2D;
pub use vector2d::Vector2D;
