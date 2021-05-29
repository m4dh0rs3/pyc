use std::vec;

use crate::math::prelude::*;

/// The subject of the game is the [`Board`].
/// It holds the current state and all data.
#[derive(Clone)]
pub struct Board {
    // not computed because modolus is high complexity
    // must be chosen at beginning
    active: Player,
    // not `u/isize` because cross plattform communication (`x86_64` vs `wasm32`)
    // bigger than `u8` is rare and high serverload (max `2^4096`)
    // not computed by `path.len()` because function is bigger than `u8`
    // not reference to `path.len` because pointer bigger or equal than `u8`
    step: u8,
    // not computed from `path.last()` because empty at first
    // also complexity
    arrow: Arrow,
    // not union because size is const
    // not one vec sliced at step size to reduce complexity
    path: Vec<Curve>,
    tiles: Vec<Curve>,
    // not const generic because size decided at runtime
    // not fixed for more variety at same complexity
    // field is not just zero (palyer id) for lower complexity
    points: Vec<Vec<Option<Player>>>,
    // not computed because might be to heavy at higher board sizes (max `2^16`)
    state: State,
    // same as state
    // not in state because data is always the same, no matter the state
    score: Score,
}

/// Enum of possible players.
/// [`Player::Gamma`] inspired by GAMMAGRAPHICS.
// is not player id as u8 because handling of draws and out of border moves
#[derive(Clone, Copy)]
enum Player {
    Gamma,
    Delta,
}

/// The pointer where the next tile will be appended.
#[derive(Clone)]
struct Arrow {
    // i8, because there are out of border moves
    position: Vec2D<i8>,
    angle: Angle,
}

/// A tile of an polycentric curve.
#[derive(Clone)]
pub struct Curve {
    // as `Arrow` can move out of border
    mid: Vec2D<i8>,
    radius: u8,
    // "historical" start, not the smaller angle, nor a side of the curve
    start: Angle,
    end: Angle,
    // not a boolean (is_positive) because of asthetic reasons
    // otherwise player would only be an u8 and rust has no overhead
    dir: Direction,
}

/// Turn-direction of [`Curve`].
#[derive(Clone)]
pub enum Direction {
    // Clockwise shorthand (its just too long)
    Positive,
    // Counterclockwise shorthand
    Negative,
}

/// The possible states the [`Board`] can be in.
/// They are exclusive and alter the behavior
/// of methods called on the [`Board`].
// `Board` itself is not an enum because data is the same no matter the state
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
// not an union because of high complexity
#[derive(Clone)]
struct Score {
    // field of max `u8 x u8 = u64 <=> 2^8*2^8 = 2^8^2 = 2^64`
    gamma: u64,
    delta: u64,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            active: Player::Gamma,
            step: 0,
            arrow: Arrow {
                // start in the middle
                position: Vec2D::new(5, 5),
                angle: Angle::zero(),
            },
            // by default there are 12 tiles
            path: Vec::with_capacity(12),
            tiles: Curve::convex_4x3(),
            // field of 11 x 11
            points: vec![vec![None; 11]; 11],
            state: State::Pending,
            score: Score { gamma: 0, delta: 0 },
        }
    }
}

impl Board {
    /// Show remaining tiles.
    pub fn options(&self) -> &[Curve] {
        if let State::Pending = self.state {
            &self.tiles[..]
        } else {
            // you can't set any tiles if the game is over
            // cant return reference to empty vec, as its droped at runtime
            // but empty slice is optimized at compiletime
            &[]
        }
    }

    /// Step by choosing a tile. Panics if index on remaining tiles (`[Board::options()]`) is invalid.
    pub fn step(&mut self, tile: usize) {
        self.set_tile(tile);
    }

    /// Set a tile on the [`Board`].
    fn set_tile(&mut self, tile: usize) {
        // removes and retunrns the tile, panics if the index is out of bounds
        let mut tile = self.tiles.remove(tile);

        // adjust the rotation of the curve to the arrow
        // this is like local to global transformation, first rotation
        tile.start = (tile.start + self.arrow.angle).normal();
        tile.end = (tile.end + self.arrow.angle).normal();

        // and than translation
        tile.mid = self.arrow.position
            + Into::<Vec2D<i8>>::into(Vec2D::from_polar(
                tile.start + Angle::straight(),
                tile.radius as f64,
            ));

        // set the arrow to the end of the curve
        self.arrow.angle = tile.start;
        self.arrow.position =
            tile.mid + Into::<Vec2D<i8>>::into(Vec2D::from_polar(tile.end, tile.radius as f64));

        // insert the curve into the path
        self.path.push(tile);
    }

    /// Check for polygons and collect points.
    fn update_score(&mut self) {
        todo!()
    }
}

impl Curve {
    /// Checks if [`Curve`] contains an [`Angle`].
    /// This can't be done with simple comparisons,
    /// as [`Curve`] is an modulo-intervall.
    fn contains(&self, angle: Angle) -> bool {
        match self.dir {
            Direction::Positive => {
                if self.start > self.end {
                    !(angle > self.end && angle < self.start)
                } else {
                    angle >= self.start && angle <= self.end
                }
            }
            Direction::Negative => {
                if self.start < self.end {
                    !(angle > self.start && angle < self.end)
                } else {
                    angle >= self.end && angle <= self.start
                }
            }
        }
    }

    /// Check intersection with another [`Curve`].
    /// Based on: [Intersection of two circles](http://paulbourke.net/geometry/circlesphere/)
    fn intersect(&self, other: &Curve) -> Vec<(Vec2D<f64>, Angle, Angle)> {
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
                    (point - Into::<Vec2D<f64>>::into(self.mid)).angle(),
                    (point - Into::<Vec2D<f64>>::into(other.mid)).angle(),
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
            Self { mid: Vec2D::zero(), radius: 1, start: Angle::half(), end: Angle::quarter(), dir: Direction::Negative },
            Self { mid: Vec2D::zero(), radius: 2, start: Angle::half(), end: Angle::quarter(), dir: Direction::Negative },
            Self { mid: Vec2D::zero(), radius: 3, start: Angle::half(), end: Angle::quarter(), dir: Direction::Negative },

            Self { mid: Vec2D::zero(), radius: 3, start: Angle::zero(), end: Angle::quarter(), dir: Direction::Positive },
            Self { mid: Vec2D::zero(), radius: 2, start: Angle::zero(), end: Angle::quarter(), dir: Direction::Positive },
            Self { mid: Vec2D::zero(), radius: 1, start: Angle::zero(), end: Angle::quarter(), dir: Direction::Positive },

            Self { mid: Vec2D::zero(), radius: 1, start: Angle::half(), end: Angle::three_quarter(), dir: Direction::Positive },
            Self { mid: Vec2D::zero(), radius: 2, start: Angle::half(), end: Angle::three_quarter(), dir: Direction::Positive },
            Self { mid: Vec2D::zero(), radius: 3, start: Angle::half(), end: Angle::three_quarter(), dir: Direction::Positive },

            Self { mid: Vec2D::zero(), radius: 3, start: Angle::zero(), end: Angle::three_quarter(), dir: Direction::Negative },
            Self { mid: Vec2D::zero(), radius: 2, start: Angle::zero(), end: Angle::three_quarter(), dir: Direction::Negative },
            Self { mid: Vec2D::zero(), radius: 1, start: Angle::zero(), end: Angle::three_quarter(), dir: Direction::Negative },
        ]
    }
}

use std::fmt;

impl fmt::Debug for Curve {
    // name curves, shall removed later
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}x",
            if self.start.0 < 0.5 { "left" } else { "right" },
            if self.end.0 < 0.5 { "up" } else { "down" },
            self.radius,
        )
    }
}
