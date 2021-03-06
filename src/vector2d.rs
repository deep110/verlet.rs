use core::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// A 2-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub struct Vector2D {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
}

impl Vector2D {
    /// Construct a new 2d vector, using the provided values.
    #[inline]
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }

    #[inline]
    pub fn new_with(p: &Vector2D) -> Vector2D {
        Vector2D { x: p.x, y: p.y }
    }

    #[inline]
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    #[inline]
    pub fn set_v(&mut self, v: &Vector2D) {
        self.x = v.x;
        self.y = v.y;
    }

    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.magnitude_sq().sqrt()
    }

    #[inline]
    pub fn magnitude_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn clear(&mut self) {
        self.x = 0.;
        self.y = 0.;
    }

    #[inline]
    pub fn dot(&self, v: Vector2D) -> f32 {
        self.x * v.x + self.y * v.y
    }

    #[inline]
    pub fn zero() -> Self {
        Vector2D::new(0.0, 0.0)
    }
}

impl Default for Vector2D {
    fn default() -> Self {
        Vector2D::new(0.0, 0.0)
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul for Vector2D {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl Div for Vector2D {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f32> for Vector2D {
    type Output = Self;

    fn div(self, s: f32) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2d_create_new() {
        let v = Vector2D::new(1., 2.);
        assert_eq!(v.x, 1.);
        assert_eq!(v.y, 2.);
    }

    #[test]
    fn test_vector2d_assign() {
        let mut v = Vector2D::new(1., 2.);
        v.set(3., 4.);
        assert_eq!(v.x, 3.);
        assert_eq!(v.y, 4.);
    }

    #[test]
    fn test_vector2d_magnitude() {
        let v = Vector2D::new(10., 15.);
        assert_eq!(v.magnitude(), 18.027756);
    }
}
