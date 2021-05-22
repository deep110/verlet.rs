use crate::vector2d::Vector2D;

#[derive(Copy, Clone)]
pub struct Particle2D {
    pub(crate) id: i32,
    position: Vector2D,
    velocity: Vector2D,
    weight: f32,
    inv_weight: f32,
}

impl Particle2D {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        let mut p = Particle2D::default();
        p.position.set(x, y);

        return p;
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

    pub fn get_position(&self) -> &Vector2D {
        &self.position
    }

    #[inline]
    pub(crate) fn add_force(&mut self, force: &Vector2D) {}

    #[inline]
    pub(crate) fn update() {}
}

impl Default for Particle2D {
    fn default() -> Self {
        Particle2D {
            id: 0,
            position: Vector2D::default(),
            velocity: Vector2D::default(),
            weight: 1f32,
            inv_weight: 1f32,
        }
    }
}
