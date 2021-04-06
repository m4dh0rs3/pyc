use math::{angle::Angle, vec2d::Vec2D};

use crate::arrow::Arrow;

/// [`Tile`] with position.
#[derive(Clone)]
pub struct Curve {
    pub(crate) mid: Vec2D<i8>,
    pub(crate) radius: u8,
    pub(crate) start: Angle,
    pub(crate) end: Angle,
    pub(crate) turn: Turn,
}

/// Turn-direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Turn {
    // Clockwise shorthand
    Positive,
    // Counterclockwise shorthand
    Negative,
}

impl Curve {
    /// Returns new [`Curve`].
    pub(crate) fn new(mid: Vec2D<i8>, radius: u8, start: Angle, end: Angle, turn: Turn) -> Self {
        Self {
            mid,
            radius,
            start,
            end,
            turn,
        }
    }

    /// Checks if [`Curve`] contains an [`Angle`].
    /// This can't be made with simple comparisons,
    /// as [`Curve`] is an modulo-intervall.
    pub(crate) fn contains(&self, angle: Angle) -> bool {
        match self.turn {
            Turn::Positive => {
                if *self.start > *self.end {
                    !(*angle > *self.end && *angle < *self.start)
                } else {
                    *angle >= *self.start && *angle <= *self.end
                }
            }
            Turn::Negative => {
                if *self.start < *self.end {
                    !(*angle > *self.start && *angle < *self.end)
                } else {
                    *angle >= *self.end && *angle <= *self.start
                }
            }
        }
    }

    /// Check intersection with another [`Curve`].
    /// Based on: [Intersection of two circles](http://paulbourke.net/geometry/circlesphere/)
    pub(crate) fn intersect(&self, other: &Curve) -> Vec<(Vec2D<f64>, Angle, Angle)> {
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

    /// Splits [`Curve`] at [`Angle`], assuming it contains it.
    pub(crate) fn split_at(self, angle: Angle) -> (Self, Self) {
        (
            Self::new(self.mid, self.radius, self.start, angle, self.turn),
            Self::new(self.mid, self.radius, angle, self.end, self.turn),
        )
    }

    /// Generates the 12 curve tiles.
    #[rustfmt::skip]
    pub fn convex_4x3() -> Vec<Self> {
        vec![
            Curve::new(Vec2D::zero(), 1, Angle::right(), Angle::up(), Turn::Negative),
            Curve::new(Vec2D::zero(), 2, Angle::right(), Angle::up(), Turn::Negative),
            Curve::new(Vec2D::zero(), 3, Angle::right(), Angle::up(), Turn::Negative),

            Curve::new(Vec2D::zero(), 1, Angle::right(), Angle::down(), Turn::Positive),
            Curve::new(Vec2D::zero(), 2, Angle::right(), Angle::down(), Turn::Positive),
            Curve::new(Vec2D::zero(), 3, Angle::right(), Angle::down(), Turn::Positive),

            Curve::new(Vec2D::zero(), 1,  Angle::left(), Angle::up(), Turn::Positive),
            Curve::new(Vec2D::zero(), 2,  Angle::left(), Angle::up(), Turn::Positive),
            Curve::new(Vec2D::zero(), 3,  Angle::left(), Angle::up(), Turn::Positive),

            Curve::new(Vec2D::zero(), 1,  Angle::left(), Angle::down(), Turn::Negative),
            Curve::new(Vec2D::zero(), 2,  Angle::left(), Angle::down(), Turn::Negative),
            Curve::new(Vec2D::zero(), 3,  Angle::left(), Angle::down(), Turn::Negative),
        ]
    }

    /// Returns the midpoint of [`Curve`].
    pub fn get_mid(&self) -> Vec2D<i8> {
        self.mid
    }

    /// Returns the start angle of [`Curve`].
    pub fn get_start(&self) -> Angle {
        self.start
    }

    /// Returns the end angle of [`Curve`].
    pub fn get_end(&self) -> Angle {
        self.end
    }

    /// Returns the radius of [`Curve`].
    pub fn get_radius(&self) -> u8 {
        self.radius
    }

    /// Returns the turn direction of [`Curve`].
    pub fn get_turn(&self) -> Turn {
        self.turn
    }
}

use std::fmt;

impl fmt::Debug for Curve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}x",
            if *self.start < 0.0 { "left" } else { "right" },
            if *self.end < 0.0 { "up" } else { "down" },
            self.radius,
        )
    }
}
