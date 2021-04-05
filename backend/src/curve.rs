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

    /// Generates the 12 curve tiles.
    #[rustfmt::skip]
    pub fn convex_4x3() -> Vec<Self> {
        use std::f64::consts::{FRAC_PI_2, PI};

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
            "{} {} {}",
            if *self.end < 0.0 { "Up" } else { "Down" },
            if *self.start < 0.0 { "Left" } else { "Right" },
            self.radius,
        )
    }
}
