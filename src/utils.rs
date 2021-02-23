pub(crate) mod point;
pub use point::Point;

pub fn remap(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    x / (b - a) * (d - c) + c
}

pub fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

pub fn bezier(t: f64, a: f64, b: f64, c: f64) -> f64 {
    lerp(t, lerp(t, a, b), c)
}