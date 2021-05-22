use crate::Particle2D;

pub struct VerletPhysics2D {
    particles: Vec<Particle2D>,
    timestep: f32,
    num_iterations: i32,
    drag: f32,
}

impl VerletPhysics2D {
    /// Initializes a Verlet physics engine instance
    pub fn new() -> VerletPhysics2D {
        VerletPhysics2D::new_with(1.0, 50, 0.0)
    }

    pub fn new_with(timestep: f32, num_iterations: i32, drag: f32) -> VerletPhysics2D {
        VerletPhysics2D {
            timestep: timestep,
            num_iterations: num_iterations,
            drag: drag,
            particles: Vec::new(),
        }
    }

    pub fn update(&mut self) {}

    pub fn clear() {}

    pub fn set_gravity(&mut self) {}

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
