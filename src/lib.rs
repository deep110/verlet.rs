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

mod physics2d;

pub use physics2d::VerletPhysics2D;
