//use crate::arrow::Rotation
use math::{remap, Vec2D};

#[derive(Debug, Clone)]
pub struct Curve {
    pub(crate) path: Vec<Vec2D<f64>>,
    pub(crate) start: Vec2D<i8>,
    pub(crate) mid: Vec2D<i8>,
    pub(crate) end: Vec2D<i8>,
}

impl Curve {
    pub fn bezier(start: Vec2D<i8>, mid: Vec2D<i8>, end: Vec2D<i8>, res: usize) -> Self {
        let mut path = Vec::with_capacity(res + 1);

        for n in 0..=res {
            path.push(Vec2D::bezier(
                remap(n as f64, 0.0, res as f64, 0.0, 1.0),
                start.into(),
                mid.into(),
                end.into(),
            ));
        }

        Curve {
            path,
            start,
            mid,
            end,
        }
    }

    /* pub fn circle(res: usize, radius: u8, mid: Vec2D<i8>, start: Rotation, end: Rotation) -> Self {
        let mut path: Vec<Vec2D<f64>> = Vec::with_capacity(res + 1);

        let mid: Vec2D<f64> = mid.into();
        let start = start.into();
        let end = end.into();
        let radius = radius.into();

        for n in 0..=res {
            path.push(
                mid + Vec2D::from_polar(remap(n as f64, 0.0, res as f64, start, end), radius),
            );
        }

        let start = path.first().unwrap().clone().into();
        let end = path.last().unwrap().clone().into();

        Curve {
            path,
            start,
            mid: mid.into(),
            end,
        }
    } */

    pub fn first(&self) -> Vec2D<f64> {
        self.path.first().unwrap().clone()
    }

    pub fn last(&self) -> Vec2D<f64> {
        self.path.last().unwrap().clone()
    }

    pub fn path(&self) -> &Vec<Vec2D<f64>> {
        &self.path
    }

    pub(crate) fn intersections(&self, other: &Self) -> Vec<Intersection> {
        if self == other {
            return vec![];
        }

        let mut p1: Vec2D<f64> = self.first();
        let mut p2: Vec2D<f64> = other.first();

        let mut intersections = Vec::new();

        for (i, o1) in (&self.path).iter().enumerate() {
            for (j, o2) in (&other.path).iter().enumerate() {
                if i > 0 && j > 0 && i + 1 < self.path.len() && j + 1 < other.path.len() {
                    if let Some(intersection) = Vec2D::intersect(p1, *o1, p2, *o2) {
                        intersections.push(Intersection {
                            at: intersection,
                            i,
                            j,
                        });
                    }
                }

                p2 = *o2;
            }

            p1 = *o1;
        }

        intersections
    }

    /* pub fn first_intersection(&self, rhs: &Self) -> Option<Intersection> {
        let mut p1: Vec2D<f64> = self.first();
        let mut p2: Vec2D<f64> = rhs.first();

        for (i, o1) in self.path.iter().skip(1).enumerate() {
            for (j, o2) in rhs.path.iter().skip(1).enumerate() {
                if let Some(intersection) = Vec2D::intersect(p1, *o1, p2, *o2) {
                    return Some(Intersection {
                        at: intersection,
                        i,
                        j,
                    });
                }

                p2 = *o2;
            }

            p1 = *o1;
        }

        None
    } */
}

#[derive(Clone, Copy)]
pub(crate) struct Intersection {
    pub(crate) at: Vec2D<f64>,
    pub(crate) i: usize,
    pub(crate) j: usize,
}

impl std::cmp::PartialEq for Curve {
    fn eq(&self, other: &Self) -> bool {
        self.mid == other.mid
            && ((self.start == other.start && self.end == other.end)
                || (self.start == other.end && self.end == other.start))
    }
}
