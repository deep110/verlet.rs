use super::ParticleConstraint2D;
use crate::{Particle2D, Vector2D};

pub struct PinConstraint2D {
    position: Vector2D,
}

impl PinConstraint2D {
    pub fn new(position: &Vector2D) -> Box<dyn ParticleConstraint2D> {
        Box::new(PinConstraint2D {
            position: position.clone(),
        })
    }
}

impl ParticleConstraint2D for PinConstraint2D {
    fn apply(&self, p: &mut Particle2D) {
        p.position.set_v(&self.position);
    }
}
