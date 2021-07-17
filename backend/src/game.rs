pub(crate) mod board;
pub(crate) mod curve;

// detail of curve interpolation to find winding number and intersection point.
// TODO: could be determined analytically
const DETAIL: usize = 12;
