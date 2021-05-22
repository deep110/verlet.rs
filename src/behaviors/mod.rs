mod constant_force_behavior;

use crate::Particle2D;

pub trait ParticleBehaviour2D {
    /// Applies the constraint to the passed in particle. The method is assumed
    /// to manipulate the given instance directly.
    fn apply(&self, p: &mut Particle2D);

    fn configure(&mut self, timestep: f32);
}

pub use constant_force_behavior::ConstantForceBehavior2D;
