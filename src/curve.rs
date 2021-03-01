use math::{remap, Vec2D};

pub struct Curve(Vec<Vec2D>);

impl Curve {
    pub fn bezier(start: Vec2D, mid: Vec2D, end: Vec2D, res: usize) -> Self {
        let mut path = Vec::with_capacity(res + 1);

        for n in 0..=res {
            path.push(Vec2D::bezier(
                remap(n as f64, 0.0, res as f64, 0.0, 1.0),
                start,
                mid,
                end,
            ));
        }

        Curve(path)
    }

    pub fn circle(res: usize, radius: f64, mid: Vec2D, start: f64, end: f64) -> Self {
        let mut path = Vec::with_capacity(res + 1);

        for n in 0..=res {
            path.push(
                mid + Vec2D::from_polar(remap(n as f64, 0.0, res as f64, start, end), radius),
            );
        }

        Curve(path)
    }

    pub fn first(&self) -> Vec2D {
        self.0.first().unwrap().clone()
    }

    pub fn last(&self) -> Vec2D {
        self.0.last().unwrap().clone()
    }

    pub fn intersect(&self, rhs: &Self) -> Vec<Intersection> {
        let mut p1: Vec2D = self.first();
        let mut o1: Vec2D = rhs.first();

        let mut intersections = Vec::new();

        for (i, p2) in self.0.iter().enumerate() {
            for (j, o2) in rhs.0.iter().enumerate() {
                if let Some(s) = Vec2D::intersect(p1, p2.clone(), o1, o2.clone()) {
                    intersections.push(Intersection { at: s, i, j });
                }

                o1 = o2.clone();
            }

            p1 = p2.clone();
        }

        intersections
    }
}

pub struct Intersection {
    at: Vec2D,
    i: usize,
    j: usize,
}
