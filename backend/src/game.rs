pub(crate) mod board;
pub(crate) mod curve;

use super::Float;

// detail of curve interpolation to find winding number and intersection point.
// TODO: could be determined analytically
const DETAIL: usize = 12;

// consts analytically determined with [GeoGebra](https://www.geogebra.org/calculator/p2ueduuy)
// the inverse of DETAIL;
const DELTA: Float = 0.1;
// point offsets withing poly-bezier
const CONVEX_2X1: Float = 0.06666667;
const CONVEX_3X2: Float = 0.02666667;
