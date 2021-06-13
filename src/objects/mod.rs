mod utils;

use crate::Vector2D;
use crate::{Particle2D, ParticleBehaviour2D, ParticleConstraint2D, ParticleKey, Spring2D};

use slotmap::SlotMap;

pub struct VerletObject2D {
    tag: &'static str,
    particles: SlotMap<ParticleKey, Particle2D>,
    springs: Vec<Spring2D>,
    constraints: Vec<Box<dyn ParticleConstraint2D>>,
}

impl VerletObject2D {
    pub fn new(tag: &'static str) -> Self {
        VerletObject2D {
            tag,
            particles: SlotMap::with_key(),
            springs: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn get_tag(&self) -> &'static str {
        self.tag
    }

    // handle particle functions

    pub fn create_particle(&mut self, position: &Vector2D) -> ParticleKey {
        return self
            .particles
            .insert_with_key(|k| (Particle2D::new(k, position)));
    }

    pub fn remove_particle(&mut self, particle_id: ParticleKey) {
        self.particles.remove(particle_id);
    }

    pub fn get_particles(&self) -> Vec<&Particle2D> {
        self.particles.values().collect()
    }

    #[inline(always)]
    pub(crate) fn update_particles(
        &mut self,
        drag: f32,
        behaviors: &Vec<Box<dyn ParticleBehaviour2D>>,
    ) {
        for p in self.particles.values_mut() {
            // apply all behaviors to each particle
            for b in behaviors.iter() {
                b.apply(p);
            }

            // update particle's position due to external forces like
            // - behaviors
            // - drag
            p.update(drag);
        }
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

    pub fn get_springs(&self) -> &Vec<Spring2D> {
        &self.springs
    }

    #[inline(always)]
    pub(crate) fn update_springs(&mut self, num_iterations: u32) {
        for _ in 0..num_iterations {
            for s in self.springs.iter() {
                let maybe_ab = self
                    .particles
                    .get_disjoint_mut([s.get_particle_a_id(), s.get_particle_b_id()]);
                match maybe_ab {
                    Some([a, b]) => {
                        s.update(a, b);
                    }
                    None => (),
                }
            }
        }
    }

    // handle constraints
    pub fn add_constraint(&mut self, mut c: Box<dyn ParticleConstraint2D>) {
        let particle_id = c.get_particle_id();
        c.init_internal(&self.particles[particle_id]);

        self.constraints.push(c);
    }

    #[inline(always)]
    pub(crate) fn apply_constraints(&mut self) {
        for c in self.constraints.iter_mut() {
            let particle_id = c.get_particle_id();
            c.apply(&mut self.particles[particle_id]);
        }
    }

    /// update verlet object a single step
    pub(crate) fn update(
        &mut self,
        num_iterations: u32,
        drag: f32,
        behaviors: &Vec<Box<dyn ParticleBehaviour2D>>,
    ) {
        self.update_particles(drag, behaviors);
        self.apply_constraints();
        self.update_springs(num_iterations);
        self.apply_constraints();
    }

    pub fn clear(&mut self) {
        self.particles.clear();
        self.springs.clear();
        self.constraints.clear();
    }
}

pub use utils::create_line_from_endpoints;
