use super::VerletObject2D;
use crate::constraints::PinConstraint2D;
use crate::{Spring2D, Vector2D};

pub fn create_line_from_endpoints(
    verlet_object: &mut VerletObject2D,
    start: Vector2D,
    end: Vector2D,
    num_segments: u32,
    stiffness: f32,
) {
    let num_particles = num_segments as usize + 1;
    let mut direction = end - start;
    let distance = direction.magnitude();
    direction = direction / distance;

    let gap = distance / num_segments as f32;

    for i in 0..num_particles {
        let s = start + direction * (gap * i as f32);
        let p = verlet_object.create_particle(&s);
        if i == 0 {
            let pin_c = PinConstraint2D::new(&p);
            verlet_object.add_constraint(pin_c);
        }
    }

    // add spring connections
    let mut xs: Vec<Spring2D> = Vec::with_capacity(num_particles);
    let particles = verlet_object.get_particles();
    for i in 1..particles.len() {
        let s = Spring2D::new(particles[i - 1], particles[i], stiffness, Some(gap));
        xs.push(s);
    }
    verlet_object.add_springs(xs);
}

pub fn create_line_from_points(
    verlet_object: &mut VerletObject2D,
    points: &[Vector2D],
    stiffness: f32,
) {
    let num_particles = points.len();

    for i in 0..num_particles {
        let p = verlet_object.create_particle(&points[i]);
        if i == 0 {
            let pin_c = PinConstraint2D::new(&p);
            verlet_object.add_constraint(pin_c);
        }
    }

    // add spring connections
    let mut xs: Vec<Spring2D> = Vec::with_capacity(num_particles);
    let particles = verlet_object.get_particles();
    for i in 1..particles.len() {
        let s = Spring2D::new(particles[i - 1], particles[i], stiffness, None);
        xs.push(s);
    }
    verlet_object.add_springs(xs);
}
