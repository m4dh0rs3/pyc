pub(crate) mod point;

pub(crate) fn remap(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    x / (b - a) * (d - c) + c
}

pub(crate) fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

pub(crate) fn bezier(t: f64, a: f64, b: f64, c: f64) -> f64 {
    lerp(t, lerp(t, a, b), c)
}