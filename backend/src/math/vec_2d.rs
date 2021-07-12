/// 2-Dimensional vector of `T` on `x` and `y`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T> {
    /// Creates new [`Vec2D`].
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

use std::ops;

// # addition

// impl only for `T`, that result in `T` after op
impl<T: ops::Add<Output = T>> ops::Add for Vec2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// impl for copy `T` aswell
impl<T: ops::Add<Output = T> + Copy> ops::Add<T> for Vec2D<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T: ops::AddAssign> ops::AddAssign for Vec2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: ops::AddAssign + Copy> ops::AddAssign<T> for Vec2D<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

// # subtraction

impl<T: ops::Sub<Output = T>> ops::Sub for Vec2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub<T> for Vec2D<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T: ops::SubAssign> ops::SubAssign for Vec2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: ops::SubAssign + Copy> ops::SubAssign<T> for Vec2D<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

// # multiplication

impl<T: ops::Mul<Output = T>> ops::Mul for Vec2D<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Vec2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: ops::MulAssign> ops::MulAssign for Vec2D<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: ops::MulAssign + Copy> ops::MulAssign<T> for Vec2D<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

// # devision

impl<T: ops::Div<Output = T>> ops::Div for Vec2D<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Vec2D<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// TODO: conflicting implementations of trait `std::ops::DivAssign<_>` for type `math::vec_2d::Vec2D<_>`
// conflicting implementation for `math::vec_2d::Vec2D<_>`
/* impl<T: ops::DivAssign> ops::DivAssign<T> for Vec2D<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
} */

impl<T: ops::DivAssign + Copy> ops::DivAssign<T> for Vec2D<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// # ops with [`f64`]
// TODO: understand why this collides with impls of [`f64`]

/* impl ops::Mul<Vec2D<f64>> for f64 {
    type Output = Vec2D<f64>;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
} */

/* impl ops::Div<Vec2D<f64>> for f64 {
    type Output = Vec2D<f64>;
    fn div(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
} */

// # trigonometry

use super::angle::Angle;

// cannot be generic because there is no(?) trait for trigonmetric types, "Float" would suffice
macro_rules! vec_2d_trig {
    ($Float: ty) => {
        impl Vec2D<$Float> {
            /// Returns the length of the vector.
            pub fn maq(&self) -> $Float {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }

            /// Returns the normal of the vector.
            fn normal(&mut self) -> Self {
                let maq = self.maq();

                Self {
                    x: self.x / maq,
                    y: self.y / maq,
                }
            }

            /// Returns a vector, rounded to integers.
            fn round(&self) -> Self {
                Self {
                    x: self.x.round(),
                    y: self.y.round(),
                }
            }

            /// Creates a vector from polar coordinates.
            pub fn from_polar(angle: Angle<$Float>, radius: $Float) -> Self {
                Self {
                    // clockwise direction with increasing angle
                    x: radius * angle.cos() as $Float,
                    y: radius * angle.sin() as $Float,
                }
            }

            /// Returns [`Angle`] around the origin.
            pub fn angle(&self) -> Angle<$Float> {
                // notice that the y axis of the viewport is flipped
                // [](https://en.wikipedia.org/wiki/Atan2#East-counterclockwise,_north-clockwise_and_south-clockwise_conventions,_etc.)
                self.y.atan2(self.x).into()
            }

            /// Rotate [`Vec2D`] around the origin.
            /// [Rotaion Matrix](https://en.wikipedia.org/wiki/Rotation_matrix)
            pub fn rotate(&self, angle: Angle<$Float>) -> Self {
                let rot_vec: Vec2D<$Float> = angle.into();
                Self {
                    x: self.x * rot_vec.x - self.y * rot_vec.y,
                    y: self.x * rot_vec.y + self.y * rot_vec.x,
                }
            }
        }

        impl From<Angle<$Float>> for Vec2D<$Float> {
            fn from(angle: Angle<$Float>) -> Self {
                Self::from_polar(angle, 1 as $Float)
            }
        }
    };
}

vec_2d_trig!(f32);
// vec_2d_trig!(f64);

// # wining number helpers

impl<T: ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> Vec2D<T> {
    /// Tests if a point is left, on or right of an infinite line.
    /// [Copyright 2001, 2012, 2021 Dan Sunday](http://web.archive.org/web/20210504233957/http://geomalgorithms.com/a03-_inclusion.html)
    // this code may be freely used and modified for any purpose
    // providing that this copyright notice is included with it
    // there is no warranty for this code, and the author of it cannot
    // be held liable for any real or imagined damage from its use
    // users of this code must verify correctness for their application
    pub fn is_left(&self, start: &Vec2D<T>, end: &Vec2D<T>) -> T {
        (end.x - start.x) * (self.y - start.y) - (self.x - start.x) * (end.y - start.y)
    }
}

impl<
        T: ops::Mul<Output = T>
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + PartialEq
            + PartialOrd
            + From<i8>
            + Copy,
    > Vec2D<T>
{
    /// Returns the 2-dimensional cross product of the vector.
    fn cross_zero(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Returns the point of intersection of to line segments.
    /// Not if they touch!
    // TODO: possibly optimize with bounding box test
    pub fn intersect(p1: Self, p2: Self, r1: Self, r2: Self) -> Option<Self> {
        let s1 = p2 - p1;
        let s2 = r2 - r1;

        let k = s1.cross_zero(&s2);

        // TODO: solve generic horror of ZERO (maybe bitwise ops?)
        if k == 0.into() {
            return None;
        }

        let d = p1 - r1;

        let s = s1.cross_zero(&d) / k;
        let t = s2.cross_zero(&d) / k;

        // greater/smaller equal are often used, but touching segments do not intersect!
        if s > 0.into() && s < 1.into() && t > 0.into() && t < 1.into() {
            Some(Self {
                x: p1.x + t * s1.x,
                y: p1.y + t * s1.y,
            })
        } else {
            None
        }
    }
}

/// Tests if two Axis Aligned Bounding Boxes intersect.
fn aabb_intersection<T: Ord>(p1: Vec2D<T>, p2: Vec2D<T>, r1: Vec2D<T>, r2: Vec2D<T>) -> bool {
    p1.x <= r2.x && p2.x >= r1.x && p1.y <= r2.y && p2.y >= r1.y
}

// # bezier

use super::utils::{bezier, lerp};

impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> Vec2D<T> {
    /// Interpolates linearly from `a` to `b` given t in `[0; 1]`.
    pub fn lerp(t: T, a: Self, b: Self) -> Self {
        Self {
            x: lerp(t, a.x, b.x),
            y: lerp(t, a.y, b.y),
        }
    }

    /// Interpolates linearly between `a`, `b` and `c` given t in `[0; 1]`.
    pub fn bezier(t: T, a: Self, b: Self, c: Self) -> Self {
        Self {
            x: bezier(t, a.x, b.x, c.x),
            y: bezier(t, a.y, b.y, c.y),
        }
    }
}

// # conversion
// float and integral are hyperboles, the first is just of higher resolution, the later of less

macro_rules! from_float {
    ($Float: ty, $Integral: ty) => {
        impl From<Vec2D<$Float>> for Vec2D<$Integral> {
            fn from(vec_2d: Vec2D<$Float>) -> Self {
                Self {
                    // round down to less precise
                    x: vec_2d.x.round() as $Integral,
                    y: vec_2d.y.round() as $Integral,
                }
            }
        }
    };
}

macro_rules! from_integral {
    ($Integral: ty, $Float: ty) => {
        impl From<Vec2D<$Integral>> for Vec2D<$Float> {
            fn from(vec_2d: Vec2D<$Integral>) -> Self {
                Self {
                    x: vec_2d.x as $Float,
                    y: vec_2d.y as $Float,
                }
            }
        }
    };
}

from_integral!(f32, f64);
from_integral!(f64, f32);

from_float!(f64, i8);
from_float!(f64, i16);
from_float!(f64, i32);
from_float!(f64, i64);

from_float!(f32, i8);
from_float!(f32, i16);
from_float!(f32, i32);
from_float!(f32, i64);

from_integral!(i8, f64);
from_integral!(i16, f64);
from_integral!(i32, f64);
from_integral!(i64, f64);

from_integral!(i8, f32);
from_integral!(i16, f32);
from_integral!(i32, f32);
from_integral!(i64, f32);

// little cheat to address all zeroable types: `From<i8>` or similar
// TODO: impl when `!(not)NonZero` exists (currently only nightly)
impl<T: From<i8>> Vec2D<T> {
    /// Create new vector at origin.
    pub fn zero() -> Self {
        Self {
            x: 0.into(),
            y: 0.into(),
        }
    }
}
