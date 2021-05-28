use crate::Particle2D;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Spring2D {
    particle_a: Rc<RefCell<Particle2D>>,
    particle_b: Rc<RefCell<Particle2D>>,
    rest_length: f32,
    rest_length_sq: f32,
    stiffness: f32,
}

impl Spring2D {
    pub fn new(
        particle_a: &Rc<RefCell<Particle2D>>,
        particle_b: &Rc<RefCell<Particle2D>>,
        rest_length: f32,
        stiffness: f32,
    ) -> Self {
        Spring2D {
            particle_a: Rc::clone(particle_a),
            particle_b: Rc::clone(particle_b),
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

    #[inline(always)]
    pub fn get_particle_a_id(&self) -> i32 {
        self.particle_a.borrow().id
    }

    #[inline(always)]
    pub fn get_particle_b_id(&self) -> i32 {
        self.particle_b.borrow().id
    }

    pub(crate) fn update(&mut self) {
        let a = &mut *(self.particle_a.borrow_mut());
        let b = &mut *(self.particle_b.borrow_mut());

        let delta = b.position - a.position;
        let dist = delta.magnitude_sq();

        let force_mag =
            (dist - self.rest_length_sq) / (dist * (a.inv_weight + b.inv_weight)) * self.stiffness;

        a.position += delta * force_mag * a.inv_weight;
        b.position -= delta * force_mag * b.inv_weight;
    }
}
