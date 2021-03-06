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
#[derive(Clone)] // , PartialEq, Eq)] // is clone, because `Board` must be clone
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
// pub(crate) type Path = Box<dyn Iterator<Item = Vec2D<Float>>>; // [What is the correct way to return an Iterator](https://stackoverflow.com/q/27535289)
pub(crate) type Path = Vec<Vec2D<Float>>;

// parameter describing intersection of two curves
pub(crate) type Intersection = (Float, Float);

// axis aligned bounding box: (bottom left, top right)
type AABB = (Vec2D<Float>, Vec2D<Float>);

use super::DETAIL;

impl Curve {
    /// Generate point on bezier curve from t in `[0; 1]`.
    pub fn point(&self, t: Float) -> Vec2D<Float> {
        Vec2D::<Float>::bezier(t, self.start.into(), self.mid.into(), self.end.into())
    }

    /// Minimal resolution vertices of curve.
    pub fn minimal_path(&self) -> Path {
        let start: Vec2D<Float> = self.start.into();
        let mid: Vec2D<Float> = self.mid.into();
        let end: Vec2D<Float> = self.end.into();

        // bezier size
        let d = (self.end.x - self.start.x)
            .abs()
            .max((self.end.y - self.start.y).abs()) as usize;

        (0..d)
            .map(move |n| {
                // not using self.point() because into
                Vec2D::<Float>::bezier(lerp(n as Float / d as Float, 0.0, 1.0), start, mid, end)
            })
            .collect()
    }

    /// Vertices of a bezier curve between `t in [t_1, t_2]`, including the start point, excluding the end point.
    pub fn path(&self, t1: Float, t2: Float) -> Path {
        let start: Vec2D<Float> = self.start.into();
        let mid: Vec2D<Float> = self.mid.into();
        let end: Vec2D<Float> = self.end.into();

        (0..DETAIL)
            .map(move |n| {
                // not using self.point() because into
                Vec2D::<Float>::bezier(lerp(n as Float / DETAIL as Float, t1, t2), start, mid, end)
            })
            .collect()

        /* Box::new((0..DETAIL).map(move |n| {
            // not using self.point() because into
            Vec2D::<Float>::bezier(lerp(n as Float / DETAIL as Float, t1, t2), start, mid, end)
        })) */
    }

    /// Compute axis aligned bounding box assuming curve is y-monotone.
    pub fn aabb(&self, start: Float, end: Float) -> AABB {
        // not pre computing the conversions again for only two points
        let start = self.point(start);
        let end = self.point(end);

        // TODO: optimize elimination process
        (
            Vec2D {
                x: start.x.min(end.x),
                y: start.y.min(end.y),
            },
            Vec2D {
                x: start.x.max(end.x),
                y: start.y.max(end.y),
            },
        )
    }

    /// Compute all parameters for intersections assuming curve is y-monotone
    pub fn intersects(&self, other: &Self) -> Vec<Intersection> {
        // intersections
        let mut ints = Vec::new();

        if self.mid != other.mid {
            if self.start == other.start {
                ints.push((0.0, 0.0));
            } else if self.start == other.end {
                ints.push((0.0, 1.0))
            }
            // ↕ these two could be chained, if the first condition wasn't there
            if self.end == other.end {
                ints.push((1.0, 1.0))
            } else if self.end == other.start {
                ints.push((1.0, 0.0))
            }
        }

        if !((self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start))
        {
            ints.append(&mut self.recursive_ints(other, 0.0, 0.0, 0));
        }

        // not an iterator, because at first it will be
        // a vec anyway
        // TODO: filter doubles
        ints
    }

    /// Recursively subdivide the bounding boxes of the y-monotone curves to find intersection parameters.
    fn recursive_ints(
        &self,
        other: &Self,
        self_t: Float,
        other_t: Float,
        n: usize,
    ) -> Vec<Intersection> {
        let offset = (2 as Float).powi(-(n as i32));

        let self_aabb = self.aabb(self_t, self_t + offset);
        let other_aabb = other.aabb(other_t, other_t + offset);

        if aabb_intersect(self_aabb, other_aabb) {
            let next_offset = offset / 2.0; // = (2 as Float).powi(-(n as i32) - 1)

            // removed detail function overhead, use const instead
            if n >= DETAIL {
                return vec![(self_t + next_offset, other_t + next_offset)];
            } else {
                let mut ints = Vec::new();

                ints.append(&mut self.recursive_ints(other, self_t, other_t, n + 1));
                ints.append(&mut self.recursive_ints(other, self_t + next_offset, other_t, n + 1));
                ints.append(&mut self.recursive_ints(other, self_t, other_t + next_offset, n + 1));
                ints.append(&mut self.recursive_ints(
                    other,
                    self_t + next_offset,
                    other_t + next_offset,
                    n + 1,
                ));

                ints
            }
        } else {
            vec![]
        }
    }
}

// Test if two axis aligned bounding boxes intersect, or lay on two edges.
fn aabb_intersect(a: AABB, b: AABB) -> bool {
    a.0.x < b.1.x && a.1.x >= b.0.x && a.0.y < b.1.y && a.1.y >= b.0.y
}

impl Curve {
    #[rustfmt::skip]
    pub fn convex_4x3() -> Vec<Self> {
        vec![
            // note that y axis if flipped in screen space!
            // up left
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 3, end: Vec2D { x: -1, y: -1 } * 3 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 2, end: Vec2D { x: -1, y: -1 } * 2 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 1, end: Vec2D { x: -1, y: -1 } * 1 },

            // up right
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 1, end: Vec2D { x:  1, y: -1 } * 1 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 2, end: Vec2D { x:  1, y: -1 } * 2 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y: -1 } * 3, end: Vec2D { x:  1, y: -1 } * 3 },

            // down left
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 3, end: Vec2D { x: -1, y:  1 } * 3 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 2, end: Vec2D { x: -1, y:  1 } * 2 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 1, end: Vec2D { x: -1, y:  1 } * 1 },

            // down right
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 1, end: Vec2D { x:  1, y:  1 } * 1 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 2, end: Vec2D { x:  1, y:  1 } * 2 },
            Self { start: Vec2D::zero(), mid: Vec2D { x: 0, y:  1 } * 3, end: Vec2D { x:  1, y:  1 } * 3 },
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
