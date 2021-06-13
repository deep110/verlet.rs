use super::VerletObject2D;
use crate::constraints::PinConstraint2D;
use crate::{ParticleKey, Spring2D, Vector2D};

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
    let mut particle_ids = Vec::<ParticleKey>::with_capacity(num_particles);

    for i in 0..num_particles {
        let pos = start + direction * (gap * i as f32);
        let p = verlet_object.create_particle(&pos);
        if i == 0 {
            let pin_c = PinConstraint2D::new(&p);
            verlet_object.add_constraint(pin_c);
        }
        particle_ids.push(p);

        // add spring connection
        if i > 0 {
            let s = Spring2D::new(&particle_ids[i - 1], &particle_ids[i], stiffness, Some(gap));
            verlet_object.add_spring(s);
        }
    }
}

pub fn create_line_from_points(
    verlet_object: &mut VerletObject2D,
    points: &[Vector2D],
    stiffness: f32,
) {
    let num_particles = points.len();
    let mut particle_ids = Vec::<ParticleKey>::with_capacity(num_particles);

    for i in 0..num_particles {
        let p = verlet_object.create_particle(&points[i]);
        if i == 0 {
            let pin_c = PinConstraint2D::new(&p);
            verlet_object.add_constraint(pin_c);
        }
        particle_ids.push(p);

        // add spring connection
        if i > 0 {
            let s = Spring2D::new(&particle_ids[i - 1], &particle_ids[i], stiffness, None);
            verlet_object.add_spring(s);
        }
    }
}

pub fn create_sheet(
    verlet_object: &mut VerletObject2D,
    center: Vector2D,
    width: f32,
    height: f32,
    num_segments: u32,
    stiffness: f32,
) {
    let np = num_segments as usize + 1;
    let stride_x = width / num_segments as f32;
    let stride_y = height / num_segments as f32;

    let mut particle_ids = Vec::<ParticleKey>::with_capacity(np * np);

    for j in 0..np {
        for i in 0..np {
            let px = center.x + stride_x * i as f32 - width / 2. + stride_x / 2.;
            let py = center.y + stride_y * j as f32 - height / 2. + stride_y / 2.;

            let s = verlet_object.create_particle(&Vector2D::new(px, py));
            if j == 0 && (i == 0 || i == np - 1) {
                let pin_c = PinConstraint2D::new(&s);
                verlet_object.add_constraint(pin_c);
            }
            particle_ids.push(s);

            // add spring connections
            if j > 0 {
                let s1 = Spring2D::new(
                    &particle_ids[(j - 1) * np + i],
                    &particle_ids[j * np + i],
                    stiffness,
                    Some(stride_y),
                );
                verlet_object.add_spring(s1);
            }
            if i > 0 {
                let s2 = Spring2D::new(
                    &particle_ids[j * np + i - 1],
                    &particle_ids[j * np + i],
                    stiffness,
                    Some(stride_x),
                );
                verlet_object.add_spring(s2);
            }
        }
    }
}
