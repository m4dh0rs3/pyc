use crate::math::prelude::*;
use crate::Float;

/// A tile of an polycentric curve.
/// The quadratic Bezier curve needs to be y-monotone!
// not an circular arc, but a quadratic bezier curve,
// because angle math is so f*ckn hard. Problems i faced:
// test if angle is within the arc, test if two arcs intersect
// making trigonometric functions intuitively continuos
// i could not solve the last one. may be found under tag:
// [v0.4.0](https://github.com/m4dh0rs3/pyc/tree/v0.4.0)
// to keep this type, i can't optimize the curve path!
#[derive(Clone)] // is clone, because `Board` must be clone
pub struct Curve {
    // as `Arrow` can move out of border
    // these all lie on a board point, so i8 is sufficient
    pub start: Vec2D<i8>,
    pub mid: Vec2D<i8>,
    pub end: Vec2D<i8>,
    // _res: PhantomData<T>,
}

// iter instead of vec to reduce allocation size
// also it is really nice
// TODO: maybe remove the box somehow? But vec is also box, so...
type Path = Box<dyn Iterator<Item = Vec2D<Float>>>;

impl Curve {
    /// Generate point on bezier curve from t in `[0; 1]`.
    pub fn point(&self, t: Float) -> Vec2D<Float> {
        Vec2D::<Float>::bezier(t, self.start.into(), self.mid.into(), self.end.into())
    }

    /// Vertices of a bezier curve between `t in [t_1, t_2]`, including the start point, excluding the end point.
    pub fn path(&self, detail: usize, t1: Float, t2: Float) -> Path {
        let start: Vec2D<Float> = self.start.into();
        let mid: Vec2D<Float> = self.mid.into();
        let end: Vec2D<Float> = self.end.into();

        Box::new((0..detail).map(move |n| {
            // not using self.point() because into
            Vec2D::<Float>::bezier(lerp(n as Float / detail as Float, t1, t2), start, mid, end)
        }))
    }
}

impl Curve {
    #[rustfmt::skip]
    pub fn convex_4x3() -> Vec<Self> {
        vec![
            // note that y axis if flipped in screen space!
            // up right
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 3, end: Vec2D { x:  1, y: -1 } * 3 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 2, end: Vec2D { x:  1, y: -1 } * 2 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 1, end: Vec2D { x:  1, y: -1 } * 1 },

            // up left
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 1, end: Vec2D { x: -1, y: -1 } * 1 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 2, end: Vec2D { x: -1, y: -1 } * 2 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 3, end: Vec2D { x: -1, y: -1 } * 3 },

            // down right
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 3, end: Vec2D { x:  1, y:  1 } * 3 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 2, end: Vec2D { x:  1, y:  1 } * 2 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 1, end: Vec2D { x:  1, y:  1 } * 1 },

            // down left
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 1, end: Vec2D { x: -1, y:  1 } * 1 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 2, end: Vec2D { x: -1, y:  1 } * 2 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 3, end: Vec2D { x: -1, y:  1 } * 3 },
        ]
    }
}

use std::fmt;

impl fmt::Debug for Curve {
    // DEBUG VIEW
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
