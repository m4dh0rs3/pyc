use super::curve::Curve;
use crate::{math::prelude::*, Float};

/// Subject of the game is the [`Board`].
/// It holds the current state and all data.
#[derive(Clone)]
pub struct Board {
    // not computed because modulus is high complexity
    // must be chosen at beginning
    pub active: Player,
    // not `u/isize` because cross platform communication (`x86_64` vs `wasm32`)
    // bigger than `u8` is rare and high server load (max `2^4096`)
    // not computed by `path.len()` because function is bigger than `u8`
    // not reference to `path.len` because pointer bigger or equal than `u8`
    pub step: u8,
    // not computed from `path.last()` because empty at first
    // also complexity
    pub arrow: Arrow,
    // not union because size is const
    // not one vec sliced at step size to reduce complexity
    // not just a set of points, it would solve the doubles
    // but then the type `Curve` would be kind of useless
    pub path: Vec<Curve>,
    pub tiles: Vec<Curve>,
    // not const generic because size decided at runtime
    // not fixed for more variety at same complexity
    // field is not just zero (player id) for lower complexity
    pub points: Vec<Vec<Option<Player>>>,
    // not computed because might be to heavy at higher board sizes (max `2^16`)
    pub state: State,
    // same as state
    // not in state because data is always the same, no matter the state
    pub score: Score,
}

/// The pointer where the next tile will be appended.
#[derive(Clone)]
pub struct Arrow {
    // i8, because there are out of border moves
    pub pos: Vec2D<i8>,
    pub dir: Direction,
}

/// Enum of possible players.
/// [`Player::Gamma`] inspired by GAMMAGRAPHICS.
// is not player id as u8 because handling of draws and out of border moves
#[derive(Clone, Copy)]
pub enum Player {
    Gamma,
    Delta,
}

/// The possible states the [`Board`] can be in.
/// They are exclusive and alter the behavior
/// of methods called on the [`Board`].
// `Board` itself is not an enum because data is the same no matter the state
#[derive(Clone)]
pub enum State {
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
pub struct Score {
    // field of max `u8 x u8 = u64 <=> 2^8*2^8 = 2^8^2 = 2^64`
    gamma: u64,
    delta: u64,
}

impl Default for Board {
    // default config of board, same as [here](https://polycentrics.com/)
    fn default() -> Self {
        Self {
            active: Player::Gamma,
            // the first step is not indexed at zero for user convenience
            step: 1,
            arrow: Arrow {
                // start in the middle
                pos: Vec2D { x: 5, y: 5 },
                dir: Direction::North,
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

use super::DETAIL;

// no getter functions, because there is a default function
// for `Board` creation, instead of a `Board::new()` function
// to reduce complexity. This needs the field to be public anyway.
// this could make it more hackable, but its WASM, so it would be
// very inaccessible.
impl Board {
    /// Show remaining tiles.
    pub fn options(&self) -> &[Curve] {
        if let State::Pending = self.state {
            // this is not iter, because its a reference to the vec
            &self.tiles[..]
        } else {
            // you can't set any tiles if the game is over
            // cant return reference to empty vec, as its dropped at runtime
            // but empty slices are optimized at compile time
            &[]
        }
    }

    /// Step by choosing a tile.
    /// Panics if index on remaining tiles ([`Board::options()`]) is out of bounds.
    pub fn step(&mut self, tile: usize) {
        self.set_tile(tile);
        // self.update_score();

        // increase step
        self.step += 1;
        // switch players
        self.active = match self.active {
            Player::Gamma => Player::Delta,
            Player::Delta => Player::Gamma,
        }
    }

    /// Set a tile on the [`Board`].
    // to understand what happens, have a look at this (deprecated since v0.5.0): [GeoGebra PYC](https://www.geogebra.org/calculator/qp8gjrsz)
    fn set_tile(&mut self, tile: usize) {
        // removes and retunrns the tile, panics if the index is out of bounds
        let mut tile = self.tiles.remove(tile as usize);

        let new_dir = self.arrow.dir
            + if tile.end.x > 0 {
                Direction::East
            } else {
                Direction::West
            };

        // translate the tile to arrow
        tile.start = self.arrow.pos;

        // rotate the control/end point in the arrow direction and then translate it to the arrow
        tile.mid = tile.start + tile.mid.rotate(self.arrow.dir);
        tile.end = tile.start + tile.end.rotate(self.arrow.dir);

        // translate arrow to the end of the curve and apply rotation
        self.arrow.pos = tile.end;
        self.arrow.dir = new_dir;

        // insert the curve into the path
        self.path.push(tile);
    }
}
