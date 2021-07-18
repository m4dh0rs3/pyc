use crate::Float;

/// 2-Dimensional vector of `T` on `x` and `y`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Vec2D<Float> {
    /// Tests if a point is left, on or right of an infinite line.
    /// [Copyright 2001, 2012, 2021 Dan Sunday](http://web.archive.org/web/20210504233957/http://geomalgorithms.com/a03-_inclusion.html)
    // this code may be freely used and modified for any purpose
    // providing that this copyright notice is included with it
    // there is no warranty for this code, and the author of it cannot
    // be held liable for any real or imagined damage from its use
    // users of this code must verify correctness for their application
    pub fn is_left(&self, start: Vec2D<Float>, end: Vec2D<Float>) -> Float {
        (end.x - start.x) * (self.y - start.y) - (self.x - start.x) * (end.y - start.y)
    }
}

// conversion
impl From<Vec2D<i8>> for Vec2D<Float> {
    fn from(vec_2d: Vec2D<i8>) -> Self {
        Self {
            x: vec_2d.x as Float,
            y: vec_2d.y as Float,
        }
    }
}
