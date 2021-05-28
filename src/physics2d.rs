use crate::behaviors::ConstantForceBehavior2D;
use crate::{Particle2D, ParticleBehaviour2D, ParticleConstraint2D, Spring2D};
use std::cell::RefCell;
use std::rc::Rc;

pub struct VerletPhysics2D {
    particles: Vec<Rc<RefCell<Particle2D>>>,
    springs: Vec<Spring2D>,
    behaviors: Vec<Box<dyn ParticleBehaviour2D>>,
    constraints: Vec<Box<dyn ParticleConstraint2D>>,
    particle_id_counter: i32,
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
            particles: Vec::new(),
            springs: Vec::new(),
            constraints: Vec::new(),
            behaviors,
            particle_id_counter: 0,
        }
    }

    pub fn set_drag(&mut self, drag: f32) {
        self.drag = drag;
    }

    // handle particle functions

    pub fn add_particle(&mut self, p: Rc<RefCell<Particle2D>>) -> i32 {
        self.particle_id_counter += 1;
        {
            let mut mut_p = p.borrow_mut();
            mut_p.id = self.particle_id_counter;
        }
        self.particles.push(p);
        return self.particle_id_counter;
    }

    pub fn remove_particle(&mut self, particle_id: i32) {
        for i in 0..self.particles.len() {
            if self.particles[i].borrow().id == particle_id {
                self.particles.swap_remove(i);
                break;
            }
        }
    }

    pub fn get_particles(&self) -> &Vec<Rc<RefCell<Particle2D>>> {
        &self.particles
    }

    #[inline(always)]
    pub(crate) fn update_particles(&mut self) {
        for p in self.particles.iter_mut() {
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
        match self.get_spring(spring.particle_a_id, spring.particle_b_id) {
            None => self.springs.push(spring),
            _ => (),
        }
    }

    pub fn get_spring(&self, particle_a_id: i32, particle_b_id: i32) -> Option<&Spring2D> {
        for s in self.springs.iter() {
            if particle_a_id == s.particle_a_id && particle_b_id == s.particle_b_id {
                return Some(s);
            }
        }
        return None;
    }

    #[inline(always)]
    pub(crate) fn update_springs(&mut self) {}

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
