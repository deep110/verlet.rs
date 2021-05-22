use std::ops::{Add, Div, Mul, Sub};

/// A 2-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub struct Vec2D {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
}

impl Vec2D {
    /// Construct a new 2d vector, using the provided values.
    #[inline]
    pub fn new(x: f32, y: f32) -> Vec2D {
        Vec2D { x, y }
    }

    #[inline]
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    #[inline]
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y + self.y
    }

    #[inline]
    pub fn dot(&self, v: Vec2D) -> f32 {
        self.x * v.x + self.y * v.y
    }

    #[inline]
    pub fn zero() -> Self {
        Vec2D::new(0.0, 0.0)
    }
}

impl Default for Vec2D {
    fn default() -> Self {
        Vec2D::new(0.0, 0.0)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vec2D {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vec2D {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
