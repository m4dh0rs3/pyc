//! # Point struct

use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
};

use crate::utils::{lerp, bezier};

/// `Point` is 2D Vector of `f64`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Implements `util::lerp` for Point
    pub fn lerp(t: f64, a: &Point, b: &Point) -> Point {
        Point {
            x: lerp(t, a.x, b.x),
            y: lerp(t, a.y, b.y),
        }
    }

    /// Implements `util::bezier` for Point
    pub fn bezier(t: f64, a: &Point, b: &Point, c: &Point) -> Point {
        Point {
            x: bezier(t, a.x, b.x, c.x),
            y: bezier(t, a.y, b.y, c.y),
        }
    }

    /// Return intersection between 2 line segments
    pub fn intersect(p1: Point, p2: Point, o1: Point, o2: Point) -> Option<Point> {
        let s1 = p2 - p1;
        let s2 = o2 - o1;

        let k = - s2.x * s1.y + s1.x * s2.y;

        if k == 0.0 {
            return None
        }

        let s = (- s1.y * (p1.x - o1.x) + s1.x * (p1.y - o1.y)) / k;
        let t = (  s2.x * (p1.y - o1.y) - s2.y * (p1.x - o1.x)) / k;
    
        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            return Some(Point::new(
                p1.x + (t * s1.x),
                p1.y + (t * s1.y),
            ))
        }

        return None
    }

    /// Construct `Point` on cirlce of radius 1 by angle
    pub fn dir(a: f64) -> Self {
        Self { x: a.cos(), y: a.sin() }
    }

    /// Construct `Point` from polar-coordinates
    pub fn polar(a: f64, r: f64) -> Self {
        Self { x: a.cos() * r, y: a.sin() * r }
    }

    /// Returns maqnitude, or length, of `Point`
    pub fn maq(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Normalizes the point (`v:|v|`)
    /// so it lies on circle of radius 1
    pub fn norm(&mut self) {
        // first, calculate maqnitude, or length, of point
        let maq = self.maq();
        // then divide by it
        self.x /= maq;
        self.y /= maq;
    }
}

// Comparison by length

/* impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.maq().cmp(other.maq())
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for Point {} */

// All operator implementations

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output { x: rhs.x * self, y: rhs.y * self }
    }
}

impl Div for Point {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl DivAssign for Point {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn point_maq() {
        let p1 = Point::new(3.0, 4.0);
        let p2 = Point::polar(0.0, 1.0);

        assert_eq!(p1.maq(), 5.0);
        assert_eq!(p2.maq(), 1.0);
    }

    #[test]
    fn point_ops() {
        let p1 = Point::new(3.0, 4.0);
        let p2 = Point::polar(0.0, 1.0);

        assert_eq!(p1 + p2, Point::new(4.0, 4.0));
        assert_eq!(p1 - p2, Point::new(2.0, 4.0));
        assert_eq!(p1 * p2, Point::new(3.0, 0.0));
    }

    #[test]
    fn point_scale() {
        let p1 = Point::new(3.0, 4.0);
        let k = 3.0;

        assert_eq!(p1 * k, Point::new(9.0, 12.0));
        assert_eq!(k * p1, Point::new(9.0, 12.0));

        assert_eq!(p1 / k, Point::new(1.0, 4.0 / 3.0));
    }

    #[test]
    fn intersect() {
        assert_eq!(Point::intersect(
            Point::new(-1.0,  0.0),
            Point::new( 1.0,  0.0),
            Point::new( 0.0,  1.0),
            Point::new( 0.0, -1.0),
        ), Some(Point::new(0.0, 0.0)));
    }

    #[test]
    fn lerp() {
        assert_eq!(
            Point::lerp(0.5,
                &Point::new(-1.0,  1.0),
                &Point::new( 1.0, -1.0)
            ),
            Point::new(0.0, 0.0)
        );

        assert_eq!(
            Point::lerp(0.0,
                &Point::new(-1.0,  1.0),
                &Point::new( 1.0, -1.0)
            ),
            Point::new(-1.0,  1.0)
        );

        assert_eq!(
            Point::lerp(1.0,
                &Point::new(-1.0,  1.0),
                &Point::new( 1.0, -1.0)
            ),
            Point::new( 1.0, -1.0)
        );
    }
}