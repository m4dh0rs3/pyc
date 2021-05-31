// why is this not generic? Because floats differ in resolution and size
// but dont really constrain compile time decisions. `f64` may have the
// only suitiable resolution for close winding numbers
// TODO: test if `f32` would suffice, because wasm loves f32
use std::f64::consts::{PI, TAU};

/// Holds an value of turns `Turn in [0, 1]` (modolu-intervall).
// the docs call it a turn, but internally it is refered to as angle
// keep in mind that a full turn is `1`, not `π`, nor `360`
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(pub f64);

impl Angle {
    /// Returns a new turn.
    fn new(angle: f64) -> Self {
        Self(angle)
    }

    /// Returns `0` turns.
    pub fn zero() -> Self {
        Self(0.0)
    }

    /// Returns turn given radians `[-π, π]`.
    pub fn from_pi(rad_pi: f64) -> Self {
        Self((rad_pi + PI) / TAU)
    }

    /// Returns turn given radians `[0, τ]`.
    fn from_tau(rad_tau: f64) -> Self {
        Self(rad_tau / TAU)
    }

    /// Returns a normalized turn in `[0; 1]`.
    pub fn normal(&self) -> Self {
        Self(self.0.fract())
    }

    /// Returns the turn as radians `[0, τ]`.
    fn into_tau(&self) -> f64 {
        self.0 * TAU
    }

    /// Returns the turn as radians `[-π, π]`.
    pub fn into_pi(&self) -> f64 {
        self.0 * TAU - PI
    }

    /// Returns the turn as degree `[0, 360]`.
    pub fn into_deg(&self) -> f64 {
        self.0 * 360.0
    }

    /// Returns a quarter turn.
    pub fn quarter() -> Self {
        Self(0.25)
    }

    /// Returns a half turn.
    pub fn half() -> Self {
        Self(0.5)
    }

    /// Returns a straight turn
    pub fn straight() -> Self {
        Self(0.5)
    }

    /// Returns a three quarter turn.
    pub fn three_quarter() -> Self {
        Self(0.75)
    }

    /// Returns the sine of the turn.
    pub fn sin(&self) -> f64 {
        self.into_tau().sin()
    }

    /// Returns the cosine of the turn.
    pub fn cos(&self) -> f64 {
        self.into_tau().cos()
    }
}

use std::ops;

impl ops::Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
