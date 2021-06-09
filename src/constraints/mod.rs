mod pin_constraint;

use crate::{Particle2D, ParticleKey};

pub trait ParticleConstraint2D {
    fn init_internal(&mut self, particle: &Particle2D);

    fn get_particle_id(&self) -> ParticleKey;

    fn apply(&mut self, particle: &mut Particle2D);
}

pub use pin_constraint::PinConstraint2D;
