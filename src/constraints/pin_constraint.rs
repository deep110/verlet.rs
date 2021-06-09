use super::ParticleConstraint2D;
use crate::{Particle2D, ParticleKey, Vector2D};

#[derive(Clone)]
pub struct PinConstraint2D {
    position: Vector2D,
    particle_id: ParticleKey,
}

impl PinConstraint2D {
    pub fn new(particle_id: &ParticleKey) -> Box<dyn ParticleConstraint2D> {
        Box::new(PinConstraint2D {
            position: Vector2D::default(),
            particle_id: particle_id.clone(),
        })
    }
}

impl ParticleConstraint2D for PinConstraint2D {
    fn init_internal(&mut self, p: &Particle2D) {
        self.position.set_v(p.get_position());
    }

    fn apply(&mut self, p: &mut Particle2D) {
        p.position.set_v(&self.position);
        p.last_position.set_v(&self.position);
    }

    #[inline(always)]
    fn get_particle_id(&self) -> ParticleKey {
        self.particle_id
    }
}
