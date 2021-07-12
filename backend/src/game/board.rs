use super::curve::Curve;
use crate::math::prelude::*;

/// Subject of the game is the [`Board`].
/// It holds the current state and all data.
#[derive(Clone)]
pub struct Board {
    // not computed because modolus is high complexity
    // must be chosen at beginning
    pub active: Player,
    // not `u/isize` because cross plattform communication (`x86_64` vs `wasm32`)
    // bigger than `u8` is rare and high serverload (max `2^4096`)
    // not computed by `path.len()` because function is bigger than `u8`
    // not reference to `path.len` because pointer bigger or equal than `u8`
    pub step: u8,
    // not computed from `path.last()` because empty at first
    // also complexity
    pub arrow: Arrow,
    // not union because size is const
    // not one vec sliced at step size to reduce complexity
    pub path: Vec<Curve>,
    pub tiles: Vec<Curve>,
    // not const generic because size decided at runtime
    // not fixed for more variety at same complexity
    // field is not just zero (palyer id) for lower complexity
    pub points: Vec<Vec<Option<Player>>>,
    // not computed because might be to heavy at higher board sizes (max `2^16`)
    pub state: State,
    // same as state
    // not in state because data is always the same, no matter the state
    pub score: Score,
}

/// Enum of possible players.
/// [`Player::Gamma`] inspired by GAMMAGRAPHICS.
// is not player id as u8 because handling of draws and out of border moves
#[derive(Clone, Copy)]
pub enum Player {
    Gamma,
    Delta,
}

/// The pointer where the next tile will be appended.
#[derive(Clone)]
pub struct Arrow {
    // i8, because there are out of border moves
    pub position: Vec2D<i8>,
    // this is the point where generics stop to make sense, so
    // TODO: reduce generics after everything works. Maybe set resolution
    // with type alias
    pub angle: Angle<f32>,
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

// detail of curve interpolation to find winding number and intersection point.
// TODO: could be determined analytically
const DETAIL: usize = 12;

use std::f32::consts::FRAC_PI_2;

// no getter functions, because there is a default function
// for `Board` creation, instead of a `Board::new()` function
// to reduce complexity. This needs the field to be public anyway.
// this could make it more hackable, but its WASM, so it would be
// very inaccessable.
impl Board {
    /// Show remaining tiles.
    pub fn options(&self) -> &[Curve] {
        if let State::Pending = self.state {
            &self.tiles[..]
        } else {
            // you can't set any tiles if the game is over
            // cant return reference to empty vec, as its droped at runtime
            // but empty slices are optimized at compiletime
            &[]
        }
    }

    /// Step by choosing a tile. Panics if index on remaining tiles (`[Board::options()]`) is invalid.
    pub fn step(&mut self, tile: usize) {
        self.set_tile(tile);

        // increase step
        self.step += 1;

        self.update_score();

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

        // rotate arrow angle either 90deg left or right, depending on tile
        // TODO: impl Add
        let new_arrow_angle = self.arrow.angle.0
            + if tile.end.x > 0 {
                FRAC_PI_2
            } else {
                -FRAC_PI_2
            };

        // translate and rotate tile to arrow
        tile.start = self.arrow.position.clone();
        tile.mid = Into::<Vec2D<i8>>::into(
            Into::<Vec2D<f32>>::into(tile.mid).rotate(self.arrow.angle + Angle::quarter()),
        ) + self.arrow.position;
        tile.end = Into::<Vec2D<i8>>::into(
            Into::<Vec2D<f32>>::into(tile.end).rotate(self.arrow.angle + Angle::quarter()),
        ) + self.arrow.position;

        // translate arrow to end of curve and apply roation
        self.arrow.position = tile.end;
        self.arrow.angle.0 = new_arrow_angle;

        // insert the curve into the path
        self.path.push(tile);
    }

    /// Check for polygons and collect points.
    fn update_score(&mut self) {
        let intersections = self.intersections();
        let polys = self.polys(&intersections);
        self.check_points(&polys);
    }

    /// Find all new intersections with the last tile and the path.
    pub fn intersections(&self) -> Vec<(usize, (f32, f32))> {
        // only test with at least 3 tiles
        if self.path.len() > 2 {
            let mut intersections = Vec::new();

            // get latest tile
            let last = self.path.last().unwrap();

            // skip the last and connecting tile as they cant intersect
            for (i, tile) in self.path[..self.path.len() - 2].iter().enumerate() {
                // find every intersections of `tile` and `last`
                // (section, last_angle, tile_angle)
                for params in last.intersects(tile, DETAIL) {
                    intersections.push((i, params))
                }
            }

            intersections
        } else {
            Vec::new()
        }
    }

    /// Generate all polygons from the new intersections.
    pub fn polys(&self, intersections: &Vec<(usize, (f32, f32))>) -> Vec<Vec<Vec2D<f32>>> {
        let mut polys = Vec::new();

        let last = self.path.last().unwrap();

        for (i, (last_t, tile_t)) in intersections {
            let mut poly = Vec::new();

            // TODO: check if the multiplaction does realy work
            let mut tile: Vec<Vec2D<f32>> = ((tile_t * DETAIL as f32) as usize..DETAIL)
                .map(|n| self.path[*i].point(n as f32 / DETAIL as f32))
                .collect();

            poly.append(&mut tile);

            for tile in self.path[i + 1..self.path.len() - 1].iter() {
                poly.append(&mut tile.path(DETAIL));
            }

            let mut last: Vec<Vec2D<f32>> = (0..(last_t * DETAIL as f32) as usize)
                .map(|n| last.point(n as f32 / DETAIL as f32))
                .collect();

            poly.append(&mut last);

            poly.push(poly.first().unwrap().clone());
            polys.push(poly);
        }

        polys
    }

    fn check_points(&mut self, polys: &Vec<Vec<Vec2D<f32>>>) {
        for poly in polys {
            // iterate through all free points. could be optimized with `flatten` and `filter`
            // self.points.iter().enumerate().map(|(i, points)| points.iter().filter(|point| point.is_none()).map(||));
            for (j, points) in self.points.iter_mut().enumerate() {
                for (i, point) in points.iter_mut().enumerate() {
                    if let None = point {
                        // the crossing number algo does not work for non-simple polys
                        // thats why we have to use the winding number algo
                        if winding_number(Vec2D::new(i as f32, j as f32), &poly) != 0 {
                            *point = Some(self.active);
                        }
                    }
                }
            }
        }
    }
}

/// Compute the winding number of a polygon $P_n = P_0$ around a point.
/// ![Copyright 2001, 2012, 2021 Dan Sunday](http://web.archive.org/web/20210504233957/http://geomalgorithms.com/a03-_inclusion.html)
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// There is no warranty for this code, and the author of it cannot
// be held liable for any real or imagined damage from its use.
// Users of this code must verify correctness for their application.
fn winding_number(point: Vec2D<f32>, poly: &Vec<Vec2D<f32>>) -> i32 {
    if poly.len() < 3 {
        0
    } else {
        let mut wn = 0;

        for i in 0..poly.len() - 1 {
            if poly[i].y <= point.y {
                if poly[i + 1].y > point.y {
                    if point.is_left(&poly[i], &poly[i + 1]) > 0.0 {
                        wn += 1;
                    }
                }
            } else if poly[i + 1].y <= point.y {
                if point.is_left(&poly[i], &poly[i + 1]) < 0.0 {
                    wn -= 1;
                }
            }
        }

        wn
    }
}
