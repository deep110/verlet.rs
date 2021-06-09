use crate::behaviors::ConstantForceBehavior2D;
use crate::{Particle2D, ParticleBehaviour2D, ParticleConstraint2D, ParticleKey, Spring2D};
use std::cell::RefCell;
use std::rc::Rc;

use slotmap::SlotMap;

pub struct VerletPhysics2D {
    particles: SlotMap<ParticleKey, Rc<RefCell<Particle2D>>>,
    springs: Vec<Spring2D>,
    behaviors: Vec<Box<dyn ParticleBehaviour2D>>,
    constraints: Vec<Box<dyn ParticleConstraint2D>>,
    timestep: f32,
    num_iterations: i32,
    drag: f32,
}

impl VerletPhysics2D {
    /// Initializes a Verlet physics engine instance
    pub fn new() -> VerletPhysics2D {
        VerletPhysics2D::new_with(1.0, 50, 0.0, None)
    }

    pub fn new_with(
        timestep: f32,
        num_iterations: i32,
        drag: f32,
        gravity: Option<f32>,
    ) -> VerletPhysics2D {
        let behaviors = match gravity {
            Some(g_value) => {
                vec![ConstantForceBehavior2D::new(0., g_value)]
            }
            None => Vec::new(),
        };
        VerletPhysics2D {
            timestep,
            num_iterations,
            drag,
            particles: SlotMap::with_key(),
            springs: Vec::new(),
            constraints: Vec::new(),
            behaviors,
        }
    }

    pub fn set_drag(&mut self, drag: f32) {
        self.drag = drag;
    }

    // handle particle functions

    #[inline]
    fn handle_key_insert(p: Rc<RefCell<Particle2D>>, id: ParticleKey) -> Rc<RefCell<Particle2D>> {
        p.borrow_mut().id = Some(id);
        return p;
    }

    pub fn add_particle(&mut self, p: Rc<RefCell<Particle2D>>) -> ParticleKey {
        return self
            .particles
            .insert_with_key(|k| (VerletPhysics2D::handle_key_insert(p, k)));
    }

    pub fn remove_particle(&mut self, particle_id: ParticleKey) {
        self.particles.remove(particle_id);
    }

    pub fn get_particles(&self) -> Vec<&Rc<RefCell<Particle2D>>> {
        self.particles.values().collect()
    }

    #[inline(always)]
    pub(crate) fn update_particles(&mut self) {
        for p in self.particles.values_mut() {
            let mut mut_p = p.borrow_mut();
            // apply all behaviors to each particle
            for b in self.behaviors.iter() {
                b.apply(&mut *mut_p);
            }

            // update particle's position due to external forces like
            // - behaviors
            // - drag
            mut_p.update(self.drag);
        }
    }

    // handle behavior functions

    pub fn add_behavior(&mut self, mut b: Box<dyn ParticleBehaviour2D>) {
        b.configure(self.timestep);
        self.behaviors.push(b);
    }

    // handle spring functions

    /// Add spring to physics engine
    ///
    /// Does not allow to add already added spring again. So at a time only
    /// a single connection can exist between two unique particles
    pub fn add_spring(&mut self, spring: Spring2D) {
        match self.get_spring(spring.get_particle_a_id(), spring.get_particle_b_id()) {
            None => self.springs.push(spring),
            _ => (),
        }
    }

    pub fn add_springs(&mut self, mut springs: Vec<Spring2D>) {
        for _ in 0..springs.len() {
            match springs.pop() {
                Some(s) => self.add_spring(s),
                None => (),
            }
        }
    }

    pub fn get_spring(
        &self,
        particle_a_id: ParticleKey,
        particle_b_id: ParticleKey,
    ) -> Option<&Spring2D> {
        for s in self.springs.iter() {
            if particle_a_id == s.get_particle_a_id() && particle_b_id == s.get_particle_b_id() {
                return Some(s);
            }
        }
        return None;
    }

    #[inline(always)]
    pub(crate) fn update_springs(&mut self) {
        for _ in 0..self.num_iterations {
            for s in self.springs.iter_mut() {
                s.update();
            }
        }
    }

    // handle constraints
    pub fn add_constraint(&mut self, c: Box<dyn ParticleConstraint2D>) {
        self.constraints.push(c);
    }

    #[inline(always)]
    pub(crate) fn apply_constraints(&mut self) {
        for c in self.constraints.iter_mut() {
            c.apply();
        }
    }

    /// run the engine for a single step
    pub fn update(&mut self) {
        self.update_particles();
        self.update_springs();
        self.apply_constraints();
    }

    pub fn clear(&mut self) {
        self.particles.clear();
        self.behaviors.clear();
        self.springs.clear();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
