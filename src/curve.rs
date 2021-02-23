use crate::utils::{point::Point, remap};

pub struct Curve(Vec<Point>);

impl Curve {
    pub fn bezier(start: Point, mid: Point, end: Point, res: usize) -> Self {
        let mut path = Vec::with_capacity(res + 1);

        for n in 0..=res {
            path.push(Point::bezier(
                remap(n as f64, 0.0, res as f64, 0.0, 1.0),
                start,
                mid,
                end,
            ));
        }

        Curve(path)
    }

    pub fn circle(res: usize, radius: f64, mid: Point, start: f64, end: f64) -> Self {
        let mut path = Vec::with_capacity(res + 1);

        for n in 0..=res {
            path.push(
                mid + Point::from_polar(
                    remap(n as f64, 0.0, res as f64, start, end),
                    radius
            ));
        }

        Curve(path)
    }

    pub fn first(self) -> Point {
        *self.0.first().unwrap()
    }

    pub fn last(self) -> Point {
        *self.0.last().unwrap()
    }
}