use crate::math::prelude::*;

/// A tile of an polycentric curve.
// not an circular arc, but a quadratic bezier curve,
// because angle math is so f*ckn hard. Problems i faced:
// test if angle is within the arc, test if two arcs intersect
// making trigonometric functions intuitively continuos
// i could not solve the last one. may be found under tag:
// [v0.4.0](https://github.com/m4dh0rs3/pyc/tree/v0.4.0)
#[derive(Clone)]
pub struct Curve {
    // as `Arrow` can move out of border
    // these all lie on a board point, so i8 is sufficient
    pub start: Vec2D<i8>,
    pub mid: Vec2D<i8>,
    pub end: Vec2D<i8>,
    // _res: PhantomData<T>,
}

// TODO: make this generic, maybe use phantom data to constrain the conversion type
macro_rules! curve_trig {
    ($Float: ty) => {
        impl Curve {
            /// Generate point on bezier curve from t in `[0; 1]`.
            pub fn point(&self, t: $Float) -> Vec2D<$Float> {
                Vec2D::<$Float>::bezier(t, self.start.into(), self.mid.into(), self.end.into())
            }

            /// Vertecies of bezier curve, including the start point, excluding the end point.
            // TODO: optimize by returning an iterator
            pub fn path(&self, detail: usize) -> Vec<Vec2D<$Float>> {
                // interpolate
                (0..detail)
                    .map(|n| self.point(n as $Float / detail as $Float))
                    .collect()
            }

            /// Compute all parameters for intersections.
            // TODO: optimize with:
            // http://web.archive.org/web/20090521080353/http://cagd.cs.byu.edu/~557/text/ch7.pdf
            // https://pomax.github.io/bezierinfo/#curveintersection
            pub fn intersects(&self, other: &Self, detail: usize) -> Vec<(usize, usize)> {
                let mut intersections = Vec::new();

                if self.start == other.start {
                    intersections.push((0, 0));
                }

                if self.end == other.end {
                    intersections.push((detail, detail));
                }

                if self.start == other.start && self.end == other.end {
                    intersections
                } else {
                    let self_path = self.path(detail);
                    let other_path = other.path(detail);

                    // TODO: optimize with `Peekable`
                    for i in 0..self_path.len() - 1 {
                        for j in 0..other_path.len() - 1 {
                            if let Some(_) = Vec2D::intersect(
                                self_path[i],
                                self_path[i + 1],
                                other_path[j],
                                other_path[j + 1],
                            ) {
                                intersections.push((i, j))
                            }
                        }
                    }

                    intersections
                }
            }
        }
    };
}

curve_trig!(f32);
// curve_trig!(f64);

impl Curve {
    #[rustfmt::skip]
    pub fn convex_4x3() -> Vec<Self> {
        vec![
            // up right
            Self { start: Vec2D::zero(), mid: Vec2D::new(0, -1) * 3, end: Vec2D::new( 1, -1) * 3, },
            Self { start: Vec2D::zero(), mid: Vec2D::new(0, -1) * 2, end: Vec2D::new( 1, -1) * 2, },
            Self { start: Vec2D::zero(), mid: Vec2D::new(0, -1) * 1, end: Vec2D::new( 1, -1) * 1, },

            // up left
            Self { start: Vec2D::zero(), mid: Vec2D::new(0, -1) * 1, end: Vec2D::new(-1, -1) * 1, },
            Self { start: Vec2D::zero(), mid: Vec2D::new(0, -1) * 2, end: Vec2D::new(-1, -1) * 2, },
            Self { start: Vec2D::zero(), mid: Vec2D::new(0, -1) * 3, end: Vec2D::new(-1, -1) * 3, },

            // down right
            Self { start: Vec2D::zero(), mid: Vec2D::new(0,  1) * 3, end: Vec2D::new( 1,  1) * 3, },
            Self { start: Vec2D::zero(), mid: Vec2D::new(0,  1) * 2, end: Vec2D::new( 1,  1) * 2, },
            Self { start: Vec2D::zero(), mid: Vec2D::new(0,  1) * 1, end: Vec2D::new( 1,  1) * 1, },

            // down right
            Self { start: Vec2D::zero(), mid: Vec2D::new(0,  1) * 1, end: Vec2D::new(-1,  1) * 1, },
            Self { start: Vec2D::zero(), mid: Vec2D::new(0,  1) * 2, end: Vec2D::new(-1,  1) * 2, },
            Self { start: Vec2D::zero(), mid: Vec2D::new(0,  1) * 3, end: Vec2D::new(-1,  1) * 3, },
        ]
    }
}

use std::fmt;

impl fmt::Debug for Curve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}x",
            if self.end.x > 0 { "Right" } else { "Left" },
            if self.mid.y > 0 { "Down" } else { "Up" },
            self.end.x.abs()
        )
    }
}
