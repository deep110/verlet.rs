use crate::behaviors::ConstantForceBehavior2D;
use crate::{ParticleBehaviour2D, VerletObject2D};

pub struct VerletPhysics2D {
    timestep: f32,
    num_iterations: u32,
    drag: f32,
    behaviors: Vec<Box<dyn ParticleBehaviour2D>>,
    objects: Vec<VerletObject2D>,
}

impl VerletPhysics2D {
    /// Initializes a Verlet physics engine instance
    pub fn new() -> VerletPhysics2D {
        VerletPhysics2D::new_with(1.0, 50, 0.01, None)
    }

    /// Initializes a Verlet physics engine instance
    pub fn new_with(
        timestep: f32,
        num_iterations: u32,
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
            behaviors,
            objects: Vec::new(),
        }
    }

    pub fn set_drag(&mut self, drag: f32) {
        self.drag = drag;
    }

    // handle behavior functions

    pub fn add_behavior(&mut self, mut b: Box<dyn ParticleBehaviour2D>) {
        b.configure(self.timestep);
        self.behaviors.push(b);
    }

    pub fn add_verlet_object(&mut self, object: VerletObject2D) {
        self.objects.push(object);
    }

    pub fn get_verlet_objects(&self) -> &Vec<VerletObject2D>{
        &self.objects
    }

    /// run the engine for a single step
    pub fn update(&mut self) {
        for o in self.objects.iter_mut() {
            o.update(self.num_iterations, self.drag, &self.behaviors);
        }
    }

    pub fn clear(&mut self) {
        self.behaviors.clear();

        for o in self.objects.iter_mut() {
            o.clear();
        }
    }
}
