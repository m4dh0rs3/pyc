use super::{bezier, lerp};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        Self { x, y, }
    }

    pub(crate) fn from_polar(a: f64, r: f64) -> Self {
        Self {
            x: r * a.cos(),
            y: r * a.sin(),
        }
    }

    pub(crate) fn maq(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub(crate) fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub(crate) fn norm(&mut self) {
        let maq = self.maq();

        self.x /= maq;
        self.y /= maq;
    }

    pub(crate)fn cross_zero(&self, rhs: &Self) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub(crate) fn intersect(p1: Self, p2: Self, o1: Self, o2: Self) -> Option<Self> {
        let s1 = p2 - p1;
        let s2 = o2 - o1;

        let k = s1.cross_zero(&s2);

        if k == 0.0 {
            return None
        }

        let s = (- s1.y * (p1.x - o1.x) + s1.x * (p1.y - o1.y)) / k;
        let t = (  s2.x * (p1.y - o1.y) - s2.y * (p1.x - o1.x)) / k;

        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            Some(Self {
                x: p1.x + t * s1.x,
                y: p1.y + t * s1.y,
            })
        } else {
            None
        }
    }

    pub(crate) fn lerp(t: f64, a: Self, b: Self) -> Self {
        Self {
            x: lerp(t, a.x, b.x),
            y: lerp(t, a.y, b.y),
        }
    }

    pub(crate) fn bezier(t: f64, a: Self, b: Self, c: Self) -> Self {
        Self {
            x: bezier(t, a.x, b.x, c.x),
            y: bezier(t, a.y, b.y, c.y),
        }
    }
}

use std::ops;

impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::MulAssign for Point {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::Div for Point {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Div<Point> for f64 {
    type Output = Point;

    fn div(self, rhs: Point) -> Self::Output {
        Point {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}