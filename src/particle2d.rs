use slotmap::new_key_type;

use crate::vector2d::Vector2D;

new_key_type! { pub struct ParticleKey; }

pub struct Particle2D {
    id: ParticleKey,
    pub(crate) position: Vector2D,
    pub(crate) last_position: Vector2D,
    force: Vector2D,
    weight: f32,
    pub(crate) inv_weight: f32,
}

impl Particle2D {
    #[inline]
    pub(crate) fn new(id: ParticleKey, position: &Vector2D) -> Particle2D {
        Particle2D {
            id,
            position: Vector2D::new_with(position),
            last_position: Vector2D::new_with(position),
            force: Vector2D::zero(),
            weight: 1f32,
            inv_weight: 1f32,
        }
    }

    #[inline]
    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
        if weight != 0.0 {
            self.inv_weight = 1.0 / weight;
        } else {
            self.inv_weight = 0.0;
        }
    }

    #[inline]
    pub fn get_position(&self) -> &Vector2D {
        &self.position
    }

    #[inline]
    pub fn get_id(&self) -> ParticleKey {
        self.id
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    #[inline]
    pub(crate) fn add_force(&mut self, force: &Vector2D) {
        self.force += *force;
    }

    #[inline]
    pub(crate) fn update(&mut self, drag: f32) {
        // apply drag
        self.last_position += (self.position - self.last_position) * drag;

        // apply forces
        let new_pos_delta = (self.position - self.last_position) + self.force * self.weight;
        self.last_position.set_v(&self.position);
        self.position.set_v(&(self.position + new_pos_delta));
        self.force.clear();
    }
}
