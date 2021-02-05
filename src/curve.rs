//! # Curve struct

use crate::point::Point;
use crate::utils::remap;

/// `Curve` represents path of type `CurveKind`
pub struct Curve {
    // Must contain at least 2 Points
    pub path: Vec<Point>,
    pub kind: CurveKind,
}

/// `CurveKind` classifies `Curve`
pub enum CurveKind {
    // Short for circular arc.
    // Denoted by start- and end angles
    Circle,
    // Bezier-curve
    // Denoted by start-, mid and endpoints
    Bezier,
    // Line between 2 Points
    Linear,
}

impl Curve {
    /// Constructs a Curve with a path of length 2,
    /// which is a line
    pub fn line(start: Point, end: Point) -> Self {
        Curve {
            // Values are intentionally moved
            path: vec![start, end],
            kind: CurveKind::Linear,
        }
    }

    /// Constructs a bezier-curve
    pub fn bezier(res: usize, start: Point, mid: Point, end: Point) -> Self {
        let mut path = Vec::with_capacity(res + 1);
        for n in 0..=res {
            path.push(Point::bezier(
                remap(n as f64, 0.0, res as f64, 0.0, 1.0),
                &start,
                &mid,
                &end,
            ));
        }

        Curve {
            path,
            kind: CurveKind::Bezier,
        }
    }

    /// Constructs a circular arc
    pub fn circle(res: usize, radius: f64, mid: Point, start: f64, end: f64) -> Self {
        let mut path = Vec::with_capacity(res + 1);
        let mut angle: f64 = 0.0;
        
        for n in 0..=res {
            angle = remap(n as f64, 0.0, res as f64, start, end);
            path.push(Point::new(
                mid.x + radius * angle.cos(),
                mid.y + radius * angle.sin(),
            ));
        }

        Curve {
            path,
            kind: CurveKind::Circle,
        }
    }

    /// Returns all intersection points of
    /// 2 curves
    pub fn intersects_at(&self, other: &Self) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        
        let mut p2: &Point = &self.path[0];
        let mut o2: &Point = &other.path[0];

        for (i, p1) in self.path.iter().enumerate() {
            if i != 0 {
                for (j, o1) in other.path.iter().enumerate() {
                    if j != 0 {
                        if let Some(point) = Point::intersect(
                            p1.clone(), p2.clone(), o1.clone(), o2.clone()) {
                            intersections.push(Intersection {point, i, j});
                        }
                    }

                    o2 = o1;
                }
            }

            p2 = p1;
        }

        intersections
    }

    /// Returns refernce to start-point of curve
    pub fn start<'a>(&'a self) -> &'a Point {
        self.path.first().unwrap()
    }
    
    /// Returns reference to end-point of curve
    pub fn end<'a>(&'a self) -> &'a Point {
        self.path.last().unwrap()
    }
}

pub struct Intersection {
    pub point: Point,
    pub i: usize,
    pub j: usize,
}

impl Intersection {
    pub fn new() -> Option<Self> {
        
    }
}