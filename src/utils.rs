//! # Utility functions
//!
//! The utility module consists of
//! linear, multi-variable functions
//! on `f64`

/// Maps `x` from `[a, b]` to `[c, d]`
pub fn remap(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    x / (b - a) * (d - c) + c
}

/// Interpolate between `a` and `b` in `[a, b]`
/// via `t in [0, 1]`, so `[0, 1]~[a, b]`
///
/// `f(t) = a + t * (b - a)`
pub fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + (b - a) * t
}

/// Interpolate between `a` and `c` over `b`
///
/// `f(t) = (a + t * (b - a)) + t * (c - (a + t * (b - a)))`
pub fn bezier(t: f64, a: f64, b: f64, c: f64) -> f64 {
    lerp(t, lerp(t, a, b), c)
}