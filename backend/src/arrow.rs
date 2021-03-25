use crate::tile::Tile;
use math::Vec2D;

pub struct Arrow {
    pub(crate) position: Vec2D<i8>,
    pub(crate) rotation: Rotation,
}

impl Arrow {
    pub(crate) fn control_points(&self, tile: &Tile) -> (Vec2D<i8>, Vec2D<i8>, Vec2D<i8>) {
        let horizont_dir: Vec2D<i8> = Vec2D::from_polar(
            (self.rotation + (*tile.vertical()).into()).into(),
            *tile.radius() as f64,
        )
        .into();

        let mid = self.position + horizont_dir;

        let vertical_dir: Vec2D<i8> = Vec2D::from_polar(
            (self.rotation + (*tile.horizontal()).into()).into(),
            *tile.radius() as f64,
        )
        .into();

        let end = mid + vertical_dir;

        (self.position, mid, end)
    }

    pub fn position(&self) -> &Vec2D<i8> {
        &self.position
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }
}

#[derive(Clone, Copy)]
pub enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

impl From<Rotation> for f64 {
    fn from(rotation: Rotation) -> Self {
        match rotation {
            Rotation::Up => 0.0,
            Rotation::Right => std::f64::consts::FRAC_PI_2,
            Rotation::Down => std::f64::consts::PI,
            Rotation::Left => std::f64::consts::PI + std::f64::consts::FRAC_PI_2,
        }
    }
}

impl std::ops::Add for Rotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Rotation::Up, Rotation::Up) => Rotation::Up,
            (Rotation::Up, Rotation::Right) => Rotation::Right,
            (Rotation::Up, Rotation::Down) => Rotation::Down,
            (Rotation::Up, Rotation::Left) => Rotation::Left,
            (Rotation::Right, Rotation::Up) => Rotation::Right,
            (Rotation::Right, Rotation::Right) => Rotation::Down,
            (Rotation::Right, Rotation::Down) => Rotation::Left,
            (Rotation::Right, Rotation::Left) => Rotation::Up,
            (Rotation::Down, Rotation::Up) => Rotation::Down,
            (Rotation::Down, Rotation::Right) => Rotation::Left,
            (Rotation::Down, Rotation::Down) => Rotation::Up,
            (Rotation::Down, Rotation::Left) => Rotation::Right,
            (Rotation::Left, Rotation::Up) => Rotation::Left,
            (Rotation::Left, Rotation::Right) => Rotation::Up,
            (Rotation::Left, Rotation::Down) => Rotation::Right,
            (Rotation::Left, Rotation::Left) => Rotation::Down,
        }
    }
}

impl std::ops::AddAssign for Rotation {
    fn add_assign(&mut self, rhs: Rotation) {
        *self = *self + rhs;
    }
}
