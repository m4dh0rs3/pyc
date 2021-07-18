pub(crate) mod board;
pub(crate) mod curve;

use super::Float;

// detail of curve interpolation to find winding number and intersection point.
// TODO: could be determined analytically
const DETAIL: usize = 12;
// the inverse of DETAIL;
const DELTA: Float = 1.0 / (DETAIL as Float);
