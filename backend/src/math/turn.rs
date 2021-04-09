use std::f64::consts::{PI, TAU};

/// Holds an value of turns.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Turn(f64);

impl Turn {
    /// Returns a new turn.
    fn new(turn: f64) -> Self {
        Self(turn)
    }

    /// Returns 0 turns.
    pub(crate) fn zero() -> Self {
        Self(0.0)
    }

    /// Returns turn given radians [-π, π].
    pub(crate) fn from_pi(rad_pi: f64) -> Self {
        Self((rad_pi + PI) / TAU)
    }

    /// Returns turn given radians [0, τ].
    fn from_tau(rad_tau: f64) -> Self {
        Self(rad_tau / TAU)
    }

    /// Returns a normalized turn in `[0; 1]`.
    pub(crate) fn normal(&self) -> Self {
        Self(self.0.fract())
    }

    /// Returns the turn as radians [0, τ].
    fn into_tau(&self) -> f64 {
        self.0 * TAU
    }

    /// Returns the turn as radians [-π, π].
    fn into_pi(&self) -> f64 {
        self.0 * TAU - PI
    }

    /// Returns a quarter turn.
    pub(crate) fn quarter() -> Self {
        Self(0.25)
    }

    /// Returns a half turn.
    pub(crate) fn half() -> Self {
        Self(0.5)
    }

    /// Returns a three quarter turn.
    pub(crate) fn three_quarter() -> Self {
        Self(0.75)
    }

    /// Returns the sine of the turn.
    pub(crate) fn sin(&self) -> f64 {
        self.into_tau().sin()
    }

    /// Returns the cosine of the turn.
    pub(crate) fn cos(&self) -> f64 {
        self.into_tau().cos()
    }

    /// Returns a straigth turn
    pub(crate) fn straight() -> Self {
        Self(0.5)
    }
}

use std::ops;

impl ops::Add for Turn {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::Sub for Turn {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
