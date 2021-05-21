pub struct VerletPhysics2D {
    timestep: f32,
    num_iterations: i32,
    drag: f32,
}

impl VerletPhysics2D {
    pub fn new() -> VerletPhysics2D {
        VerletPhysics2D {
            timestep: 1.0,
            num_iterations: 50,
            drag: 0.0,
        }
    }

    pub fn new_with(timestep: f32, num_iterations: i32, drag: f32) -> VerletPhysics2D {
        VerletPhysics2D {
            timestep: timestep,
            num_iterations: num_iterations,
            drag: drag,
        }
    }

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
