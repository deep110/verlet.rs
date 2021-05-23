use crate::{Particle2D, Vector2D};

pub struct Spring2D {
    pub(crate) particle_a_id: i32,
    pub(crate) particle_b_id: i32,
    rest_length: f32,
    rest_length_sq: f32,
    stiffness: f32,
}

impl Spring2D {
    pub fn new(
        particle_a: &Particle2D,
        particle_b: &Particle2D,
        rest_length: f32,
        stiffness: f32,
    ) -> Self {
        Spring2D {
            particle_a_id: particle_a.id,
            particle_b_id: particle_b.id,
            rest_length,
            rest_length_sq: rest_length * rest_length,
            stiffness,
        }
    }

    pub fn set_rest_length(&mut self, rest_length: f32) {
        self.rest_length = rest_length;
        self.rest_length_sq = rest_length * rest_length;
    }

    pub fn get_rest_length(&self) -> f32 {
        self.rest_length
    }

    pub(crate) fn update(&self, a: &mut Particle2D, b: &mut Particle2D) {
        let delta = b.position - a.position;
        let dist = delta.magnitude_sq();

        let force_mag =
            (dist - self.rest_length_sq) / (dist * (a.inv_weight + b.inv_weight)) * self.stiffness;

        a.position += delta * force_mag * a.inv_weight;
        b.position -= delta * force_mag * b.inv_weight;
    }
}
