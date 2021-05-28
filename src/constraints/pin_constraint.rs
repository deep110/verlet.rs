use super::ParticleConstraint2D;
use crate::{Particle2D, Vector2D};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct PinConstraint2D {
    position: Vector2D,
    particle: Rc<RefCell<Particle2D>>,
}

impl PinConstraint2D {
    pub fn new(particle: &Rc<RefCell<Particle2D>>) -> Box<dyn ParticleConstraint2D> {
        Box::new(PinConstraint2D {
            position: particle.borrow().position.clone(),
            particle: Rc::clone(particle),
        })
    }
}

impl ParticleConstraint2D for PinConstraint2D {
    fn apply(&mut self) {
        let mut mut_p = self.particle.borrow_mut();
        mut_p.position.set_v(&self.position);
        mut_p.last_position.set_v(&self.position);
    }
}
