use crate::Float;

/// Interpolates linearly from `a` to `b` given t in `[0; 1]`.
// TODO: decide if inline should be used
// #[inline]
pub fn lerp(t: Float, a: Float, b: Float) -> Float {
    a + t * (b - a)
}

/// Interpolates linearly between `a`, `b` and `c` given t in `[0; 1]`.
/// [Quadratic Bezier Curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_B%C3%A9zier_curves)
// #[inline]
pub fn bezier(t: Float, a: Float, b: Float, c: Float) -> Float {
    lerp(t, lerp(t, a, b), lerp(t, b, c))
}
