use math::{remap, Vec2D};

#[derive(Debug, Clone)]
pub struct Curve {
    pub path: Vec<Vec2D<f64>>,
    pub start: Vec2D<i8>,
    pub mid: Vec2D<i8>,
    pub end: Vec2D<i8>,
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

        Self {
            path,
            start,
            mid,
            end,
        }
    }

    pub fn circle(start: Vec2D<i8>, mid: Vec2D<i8>, end: Vec2D<i8>, res: usize) -> Self {
        let start: Vec2D<f64> = start.into();
        let mid: Vec2D<f64> = mid.into();
        let end: Vec2D<f64> = end.into();

        let start_angle = (start.y - mid.y).atan2(start.x - mid.x);
        let end_angle = (end.y - mid.y).atan2(end.x - mid.x);
        let radius = ((start.x - mid.x).powi(2) + (start.y - mid.y).powi(2)).sqrt();
        let mut path = Vec::with_capacity(res + 1);

        for n in 0..=res {
            let angle = remap(n as f64, 0.0, res as f64, start_angle, end_angle);
            path.push(mid + Vec2D::from_polar(angle, radius));
        }

        Self {
            start: start.into(),
            mid: mid.into(),
            end: end.into(),
            path,
        }
    }

    pub fn first(&self) -> Vec2D<f64> {
        self.path.first().unwrap().clone()
    }

    pub fn last(&self) -> Vec2D<f64> {
        self.path.last().unwrap().clone()
    }
}

impl std::cmp::PartialEq for Curve {
    fn eq(&self, other: &Self) -> bool {
        self.mid == other.mid
            && ((self.start == other.start && self.end == other.end)
                || (self.start == other.end && self.end == other.start))
    }
}
