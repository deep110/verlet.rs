mod pin_constraint;

use crate::Particle2D;

pub trait ParticleConstraint2D {
    fn apply(&self, p: &mut Particle2D);
}

pub use pin_constraint::PinConstraint2D;
