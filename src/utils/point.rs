#[derive(Debug, Clone, PartialEq)]
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

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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

impl ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
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

impl ops::Div<Point> for f64 {
    type Output = Point;

    fn div(self, rhs: Point) -> Self::Output {
        Point {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}