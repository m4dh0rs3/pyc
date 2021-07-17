// ---generic because I can. f32 would suffice, since new wn algo is solid---
// not generic anymore, to reduce since of lib

/// `u8` version of an angle, cardinal directions.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

// # trigonometry

impl Direction {
    /// Return the sine of the direction.
    pub fn sin(&self) -> i8 {
        match self {
            Direction::North => 0,
            Direction::South => 0,
            Direction::West => -1,
            Direction::East => 1,
        }
    }

    /// Return the cosine of the direction.
    pub fn cos(&self) -> i8 {
        match self {
            Direction::North => 1,
            Direction::South => -1,
            Direction::West => 0,
            Direction::East => 0,
        }
    }
}

// # operations

use std::ops;

impl ops::Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::North, Self::North) => Self::North,
            (Self::North, Self::East) => Self::East,
            (Self::North, Self::South) => Self::South,
            (Self::North, Self::West) => Self::West,
            (Self::East, Self::North) => Self::East,
            (Self::East, Self::East) => Self::South,
            (Self::East, Self::South) => Self::West,
            (Self::East, Self::West) => Self::North,
            (Self::South, Self::North) => Self::South,
            (Self::South, Self::East) => Self::West,
            (Self::South, Self::South) => Self::North,
            (Self::South, Self::West) => Self::East,
            (Self::West, Self::North) => Self::West,
            (Self::West, Self::East) => Self::North,
            (Self::West, Self::South) => Self::East,
            (Self::West, Self::West) => Self::South,
        }
    }
}
