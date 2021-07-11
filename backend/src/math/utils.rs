use std::ops;

/// Projects `x` from `[a; b]` into `[c; d]`.
pub fn remap<
    // YES, generics look horrible in this case, but they solve device interoperability
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy,
>(
    x: T,
    a: T,
    b: T,
    c: T,
    d: T,
) -> T {
    x / (b - a) * (d - c) + c
}

/// Interpolates linearly from `a` to `b` given t in `[0; 1]`.
pub fn lerp<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy>(
    t: T,
    a: T,
    b: T,
) -> T {
    a + t * (b - a)
}

/// Interpolates linearly between `a`, `b` and `c` given t in `[0; 1]`.
pub fn bezier<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy>(
    t: T,
    a: T,
    b: T,
    c: T,
) -> T {
    lerp(t, lerp(t, a, b), lerp(t, b, c))
}
