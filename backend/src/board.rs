use crate::math::{turn::Turn, vec2d::Vec2D};

/// The subject of the game is the [`Board`].
/// It holds the current state and all data.
#[derive(Clone)]
pub struct Board {
    active: Player,
    step: u8,
    arrow: Arrow,
    path: Vec<Curve>,
    tiles: Vec<Curve>,
    points: [[Option<Player>; 11]; 11],
    state: State,
    score: Score,
}

impl Board {
    /// Step by choosing a tile. Panics if index is invalid.
    fn step(&mut self, tile: usize) {
        self.set_tile(tile);
    }

    /// Set a tile on the [`Board`].
    fn set_tile(&mut self, tile: usize) {
        let mut tile = self.tiles.remove(tile);

        tile.start = (tile.start + self.arrow.turn).normal();
        tile.end = (tile.end + self.arrow.turn).normal();

        tile.mid = self.arrow.position
            + Into::<Vec2D<i8>>::into(Vec2D::from_polar(
                tile.start + Turn::straight(),
                tile.radius as f64,
            ));

        self.arrow.turn = tile.start;
        self.arrow.position =
            tile.mid + Into::<Vec2D<i8>>::into(Vec2D::from_polar(tile.end, tile.radius as f64));

        self.path.push(tile);
    }
}

/// Enum of possible players.
/// [`Player::Gamma`] inspired by GAMMAGRAPHICS.
#[derive(Clone, Copy)]
enum Player {
    Gamma,
    Delta,
}

/// The possible states the [`Board`] can be in.
/// They are exclusive and alter the behavior
/// of methods called on the [`Board`].
#[derive(Clone)]
enum State {
    Victory(Player),
    Pending,
    Draw,
}

/// Each player holds an score according
/// to how much points he "collects".
/// Nevertheless this does not have to decide
/// the end result if the opponent makes an
/// invalid move.
#[derive(Clone)]
struct Score {
    gamma: u16,
    delta: u16,
}

/// The pointer where the next tile will be appended.
#[derive(Clone)]
struct Arrow {
    position: Vec2D<i8>,
    turn: Turn,
}

/// A tile of an polycentric curve.
#[derive(Clone)]
struct Curve {
    mid: Vec2D<i8>,
    radius: u8,
    start: Turn,
    end: Turn,
    turn_dir: TurnDir,
}

/// Turn-direction.
#[derive(Clone)]
pub enum TurnDir {
    // Clockwise shorthand
    Positive,
    // Counterclockwise shorthand
    Negative,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            active: Player::Gamma,
            step: 0,
            arrow: Arrow {
                position: Vec2D::new(5, 5),
                turn: Turn::zero(),
            },
            path: Vec::with_capacity(12),
            tiles: Curve::convex_4x3(),
            points: [[None; 11]; 11],
            state: State::Pending,
            score: Score { gamma: 0, delta: 0 },
        }
    }
}

impl Curve {
    /// Checks if [`Curve`] contains an [`Turn`].
    /// This can't be made with simple comparisons,
    /// as [`Curve`] is an modulo-intervall.
    fn contains(&self, turn: Turn) -> bool {
        match self.turn_dir {
            TurnDir::Positive => {
                if self.start > self.end {
                    !(turn > self.end && turn < self.start)
                } else {
                    turn >= self.start && turn <= self.end
                }
            }
            TurnDir::Negative => {
                if self.start < self.end {
                    !(turn > self.start && turn < self.end)
                } else {
                    turn >= self.end && turn <= self.start
                }
            }
        }
    }

    /// Check intersection with another [`Curve`].
    /// Based on: [Intersection of two circles](http://paulbourke.net/geometry/circlesphere/)
    fn intersect(&self, other: &Curve) -> Vec<(Vec2D<f64>, Turn, Turn)> {
        // offsets between circle centers
        let off: Vec2D<f64> = (other.mid - self.mid).into();

        // straight line distance between the centers
        let dist = off.maq();

        // check for solvability, circles should neither be contained, nor distant
        if dist <= (self.radius + other.radius) as f64
            && dist >= (self.radius as f64 - other.radius as f64).abs()
        {
            // distance from point 0 to point 2
            let tan = ((self.radius as f64).powi(2) - (other.radius as f64).powi(2) + dist.powi(2))
                / (2.0 * dist);

            let mid: Vec2D<f64> = Into::<Vec2D<f64>>::into(self.mid) + (off * (tan / dist));

            // distance from point 2 to either of the intersection points
            let height = ((self.radius as f64).powi(2) - tan.powi(2)).sqrt();

            // offsets of the intersection points from point 2
            let hypot = Vec2D::new(-off.y * (height / dist), off.x * (height / dist));

            (if hypot == Vec2D::zero() {
                vec![mid]
            } else {
                vec![mid - hypot, mid + hypot]
            })
            .into_iter()
            .map(|point| {
                (
                    point,
                    (point - Into::<Vec2D<f64>>::into(self.mid)).turn(),
                    (point - Into::<Vec2D<f64>>::into(other.mid)).turn(),
                )
            })
            .filter(|(_, self_angle, other_angle)| {
                self.contains(*self_angle) && other.contains(*other_angle)
            })
            .collect()
        } else {
            Vec::new()
        }
    }

    #[rustfmt::skip]
    fn convex_4x3() -> Vec<Self> {
        vec![
            Self { mid: Vec2D::zero(), radius: 1, start: Turn::half(), end: Turn::quarter(), turn_dir: TurnDir::Negative },
            Self { mid: Vec2D::zero(), radius: 2, start: Turn::half(), end: Turn::quarter(), turn_dir: TurnDir::Negative },
            Self { mid: Vec2D::zero(), radius: 3, start: Turn::half(), end: Turn::quarter(), turn_dir: TurnDir::Negative },

            Self { mid: Vec2D::zero(), radius: 3, start: Turn::zero(), end: Turn::quarter(), turn_dir: TurnDir::Positive },
            Self { mid: Vec2D::zero(), radius: 2, start: Turn::zero(), end: Turn::quarter(), turn_dir: TurnDir::Positive },
            Self { mid: Vec2D::zero(), radius: 1, start: Turn::zero(), end: Turn::quarter(), turn_dir: TurnDir::Positive },

            Self { mid: Vec2D::zero(), radius: 1, start: Turn::half(), end: Turn::three_quarter(), turn_dir: TurnDir::Positive },
            Self { mid: Vec2D::zero(), radius: 2, start: Turn::half(), end: Turn::three_quarter(), turn_dir: TurnDir::Positive },
            Self { mid: Vec2D::zero(), radius: 3, start: Turn::half(), end: Turn::three_quarter(), turn_dir: TurnDir::Positive },

            Self { mid: Vec2D::zero(), radius: 3, start: Turn::zero(), end: Turn::three_quarter(), turn_dir: TurnDir::Negative },
            Self { mid: Vec2D::zero(), radius: 2, start: Turn::zero(), end: Turn::three_quarter(), turn_dir: TurnDir::Negative },
            Self { mid: Vec2D::zero(), radius: 1, start: Turn::zero(), end: Turn::three_quarter(), turn_dir: TurnDir::Negative },
        ]
    }
}
