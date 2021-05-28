mod pin_constraint;

pub trait ParticleConstraint2D {
    fn apply(&mut self);
}

pub use pin_constraint::PinConstraint2D;
