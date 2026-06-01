use super::ParticleBehaviour2D;
use crate::{Particle2D, Vector2D};

pub struct ConstantForceBehavior2D {
    force: Vector2D,
    scaled_force: Vector2D,
    timestep: f32,
}

impl ConstantForceBehavior2D {
    #[inline]
    pub fn new(force_x: f32, force_y: f32) -> Box<dyn ParticleBehaviour2D> {
        Box::new(ConstantForceBehavior2D {
            force: Vector2D::new(force_x, force_y),
            timestep: 1.0,
            scaled_force: Vector2D::default(),
        })
    }

    #[inline]
    pub fn set_force(&mut self, force: &Vector2D) {
        self.force.set(force.x, force.y);
        self.scaled_force = self.force * self.timestep;
    }
}

impl ParticleBehaviour2D for ConstantForceBehavior2D {
    fn apply(&self, p: &mut Particle2D) {
        p.add_force(&self.scaled_force);
    }
    fn configure(&mut self, timestep: f32) {
        self.timestep = timestep;
        self.scaled_force = self.force * timestep;
    }
}
