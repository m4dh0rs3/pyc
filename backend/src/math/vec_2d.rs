use crate::Float;

/// 2-Dimensional vector of `T` on `x` and `y`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl Vec2D<i8> {
    /// Create new vector at origin.
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

// # operations

use std::ops;

// addition
impl ops::Add for Vec2D<i8> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// scaling
impl ops::Mul<i8> for Vec2D<i8> {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// # trigonometry

use super::angle::Direction;

impl Vec2D<i8> {
    /// Rotate [`Vec2D`] around the origin.
    /// [Rotation Matrix](https://en.wikipedia.org/wiki/Rotation_matrix)
    pub fn rotate(&self, dir: Direction) -> Self {
        let cos = dir.cos();
        let sin = dir.sin();

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

// # bezier

use super::utils::bezier;

impl Vec2D<Float> {
    /// Interpolates linearly between `a`, `b` and `c` given t in `[0; 1]`.
    /// [Quadratic Bezier Curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_B%C3%A9zier_curves)
    pub fn bezier(t: Float, a: Self, b: Self, c: Self) -> Self {
        Self {
            x: bezier(t, a.x, b.x, c.x),
            y: bezier(t, a.y, b.y, c.y),
        }
    }
}

// # conversion

impl From<Vec2D<i8>> for Vec2D<Float> {
    fn from(vec_2d: Vec2D<i8>) -> Self {
        Self {
            x: vec_2d.x as Float,
            y: vec_2d.y as Float,
        }
    }
}
