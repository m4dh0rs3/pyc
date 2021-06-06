use crate::math::prelude::*;

/// A tile of an polycentric curve.
#[derive(Clone)]
pub struct Curve {
    // as `Arrow` can move out of border
    pub mid: Vec2D<i8>,
    pub radius: u8,
    // "historical" start, not the smaller angle, nor a side of the curve
    // not using enums to reduce complexity of intersections tests
    pub start: Angle,
    // the offset, instead of second angle reuces a `Direction` enum
    pub off: Angle,
}

impl Curve {
    /// Checks if [`Curve`] contains an [`Angle`].
    /// This can't be done with simple comparisons,
    /// as [`Curve`] is an modulo-intervall.
    fn contains(&self, angle: Angle) -> bool {
        let end = (self.start + self.off).normal();

        if self.off.0 > 0.0 {
            if self.start > end {
                !(angle > end && angle < self.start)
            } else {
                angle >= self.start && angle <= end
            }
        } else {
            if self.start < end {
                !(angle > self.start && angle < end)
            } else {
                angle >= end && angle <= self.start
            }
        }
    }

    /// Check intersection with another [`Curve`].
    /// Based on: [Intersection of two circles](http://paulbourke.net/geometry/circlesphere/)
    pub(crate) fn intersects(&self, other: &Curve) -> Vec<(Vec2D<f64>, Angle, Angle)> {
        // offsets between circle centers
        let off: Vec2D<f64> = (other.mid - self.mid).into();

        // straight line distance between the centers
        let dist = off.maq();

        // check for solvability, circles should neither be contained, nor distant
        if dist <= (self.radius + other.radius) as f64
            && dist >= (self.radius as f64 - other.radius as f64).abs()
        {
            // distance from point 0 to point 2
            let tan = ((self.radius as f64).powi(2) - (other.radius as f64).powi(2) + dist.powi(2))
                / (2.0 * dist);

            let mid: Vec2D<f64> = Into::<Vec2D<f64>>::into(self.mid) + (off * (tan / dist));

            // distance from point 2 to either of the intersection points
            let height = ((self.radius as f64).powi(2) - tan.powi(2)).sqrt();

            // offsets of the intersection points from point 2
            let hypot = Vec2D::new(-off.y * (height / dist), off.x * (height / dist));

            (if hypot == Vec2D::zero() {
                vec![mid]
            } else {
                vec![mid - hypot, mid + hypot]
            })
            // the declarative is as big as the hard-coded solution, but fancier, so here you go
            .into_iter()
            .map(|point| {
                (
                    point,
                    (point - Into::<Vec2D<f64>>::into(self.mid)).angle(),
                    (point - Into::<Vec2D<f64>>::into(other.mid)).angle(),
                )
            })
            .filter(|(_, self_angle, other_angle)| {
                self.contains(*self_angle) && other.contains(*other_angle)
            })
            .collect()
        } else {
            Vec::new()
        }
    }

    #[rustfmt::skip]
    pub(crate) fn convex_4x3() -> Vec<Self> {
        vec![
            Self { mid: Vec2D::zero(), radius: 3, start: Angle::quarter(), off: Angle::zero() - Angle::quarter() },
            Self { mid: Vec2D::zero(), radius: 2, start: Angle::quarter(), off: Angle::zero() - Angle::quarter() },
            Self { mid: Vec2D::zero(), radius: 1, start: Angle::quarter(), off: Angle::zero() - Angle::quarter() },

            Self { mid: Vec2D::zero(), radius: 1, start: Angle::three_quarter(), off: Angle::quarter() },
            Self { mid: Vec2D::zero(), radius: 2, start: Angle::three_quarter(), off: Angle::quarter() },
            Self { mid: Vec2D::zero(), radius: 3, start: Angle::three_quarter(), off: Angle::quarter() },

            Self { mid: Vec2D::zero(), radius: 3, start: Angle::quarter(), off: Angle::quarter() },
            Self { mid: Vec2D::zero(), radius: 2, start: Angle::quarter(), off: Angle::quarter() },
            Self { mid: Vec2D::zero(), radius: 1, start: Angle::quarter(), off: Angle::quarter() },

            Self { mid: Vec2D::zero(), radius: 1, start: Angle::three_quarter(), off: Angle::zero() - Angle::quarter() },
            Self { mid: Vec2D::zero(), radius: 2, start: Angle::three_quarter(), off: Angle::zero() - Angle::quarter() },
            Self { mid: Vec2D::zero(), radius: 3, start: Angle::three_quarter(), off: Angle::zero() - Angle::quarter() },
        ]
    }
}

use std::fmt;

impl fmt::Debug for Curve {
    // name curves, shall removed later
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}x",
            if self.start.0 < 0.5 { "Left" } else { "Right" },
            if (self.start + self.off).normal().0 < 0.5 {
                "Up"
            } else {
                "Down"
            },
            self.radius,
        )
    }
}
