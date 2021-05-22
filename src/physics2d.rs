use crate::behaviors::ConstantForceBehavior2D;
use crate::{Particle2D, ParticleBehaviour2D};

pub struct VerletPhysics2D {
    particles: Vec<Particle2D>,
    behaviors: Vec<Box<dyn ParticleBehaviour2D>>,
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
            behaviors,
            particle_id_counter: 0,
        }
    }

    pub fn add_particle(&mut self, mut p: Particle2D) {
        self.particle_id_counter += 1;
        p.id = self.particle_id_counter;
        self.particles.push(p);
    }

    pub fn remove_particle(&mut self, p: &Particle2D) {
        let r_id = p.id;
        for i in 0..self.particles.len() {
            if self.particles[i].id == r_id {
                self.particles.swap_remove(i);
            }
        }
    }

    pub fn get_particles(&self) -> &Vec<Particle2D> {
        &self.particles
    }

    pub fn add_behavior(&mut self, b: Box<dyn ParticleBehaviour2D>) {
        self.behaviors.push(b);
    }

    pub fn update(&mut self) {}

    pub fn clear(&mut self) {
        self.particles.clear();
        self.behaviors.clear();
    }

    pub fn set_drag(&mut self, drag: f32) {
        self.drag = drag;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
