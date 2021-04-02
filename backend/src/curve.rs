use crate::angle::Angle;
use math::Vec2D;

/// [`Tile`] with position.
#[derive(Debug)]
pub(crate) struct Curve {
    mid: Vec2D<i8>,
    start: Angle,
    end: Angle,
    turn: Turn,
}

/// Turn-direction.
#[derive(Debug)]
pub(crate) enum Turn {
    // Clockwise shorthand
    Positive,
    // Counterclockwise shorthand
    Negative,
}

impl Curve {
    /// Returns new [`Curve`].
    pub(crate) fn new(mid: Vec2D<i8>, start: Angle, end: Angle, turn: Turn) -> Self {
        Self {
            mid,
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
}
