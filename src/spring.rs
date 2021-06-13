use crate::{Particle2D, ParticleKey};

pub struct Spring2D {
    particle_a_id: ParticleKey,
    particle_b_id: ParticleKey,
    rest_length: f32,
    rest_length_sq: f32,
    stiffness: f32,
}

impl Spring2D {
    pub fn new(
        particle_a: &Particle2D,
        particle_b: &Particle2D,
        stiffness: f32,
        rest_length: Option<f32>,
    ) -> Self {
        let rs = match rest_length {
            Some(n) => n,
            None => {
                (*particle_a.get_position() - *particle_b.get_position()).magnitude()
            },
        };
        Spring2D {
            particle_a_id: particle_a.get_id(),
            particle_b_id: particle_b.get_id(),
            rest_length: rs,
            rest_length_sq: rs * rs,
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

    #[inline(always)]
    pub fn get_particle_a_id(&self) -> ParticleKey {
        self.particle_a_id
    }

    #[inline(always)]
    pub fn get_particle_b_id(&self) -> ParticleKey {
        self.particle_b_id
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
