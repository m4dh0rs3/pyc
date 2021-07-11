use super::curve::*;
use crate::math::prelude::*;

/// The subject of the game is the [`Board`].
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
    pub angle: Angle,
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

// detail of curve interpolation to find winding number.
// TODO: could be determend analyticaly
const DETAIL: usize = 12;

// no getter functions, because there is a default function
// for `Board` creation, instead of a `Board::new()` function
// to reduce complexity. This needs the field to be public anyway.
// this could make it more hackable, but itss wasm, so it would be
// very complicated.
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
    pub fn step(&mut self, tile: u8) {
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
    // to understand what happens, have a look at this: [GeoGebra PYC](https://www.geogebra.org/calculator/qp8gjrsz)
    fn set_tile(&mut self, tile: u8) {
        // removes and retunrns the tile, panics if the index is out of bounds
        let mut tile = self.tiles.remove(tile as usize);

        // calculate midpoint
        // this is like local to global transformation, first translation
        tile.mid = self.arrow.position
            + Into::<Vec2D<i8>>::into(Vec2D::from_polar(
                self.arrow.angle - tile.start,
                tile.radius as f64,
            ));

        // then adjust the rotation of the curve to the arrow
        // normalized for checking of containment later
        tile.start = (tile.start + self.arrow.angle).normal();
        // tile.off = (tile.off + self.arrow.angle).normal();

        // set the arrow to the end of the curve
        self.arrow.angle = (tile.start + Angle::straight()).normal();
        self.arrow.position = tile.mid
            + Into::<Vec2D<i8>>::into(Vec2D::from_polar(
                (tile.start + tile.off).normal(),
                tile.radius as f64,
            ));

        // insert the curve into the path
        self.path.push(tile);
    }

    /// Check for polygons and collect points.
    fn update_score(&mut self) {
        let intersections = self.intersections();
        let polys = self.polys(&intersections);
        self.check_points(&polys);
    }

    fn check_points(&mut self, polys: &Vec<Vec<Vec2D<f64>>>) {
        for poly in polys {
            // iterate through all free points. could be optimized with `flatten` and `filter`
            // self.points.iter().enumerate().map(|(i, points)| points.iter().filter(|point| point.is_none()).map(||));
            for (i, points) in self.points.iter_mut().enumerate() {
                for (j, point) in points.iter_mut().enumerate() {
                    if let None = point {
                        // the crossing number algo does not work for non-simple polys
                        // thats why we have to use the winding number algo
                        if winding_number(Vec2D::new(j as f64, i as f64), &poly) != 0 {
                            *point = Some(self.active);
                        }
                    }
                }
            }
        }
    }

    /// Find all new intersections with the last tile and the path.
    pub fn intersections(&self) -> Vec<(Vec2D<f64>, usize, Angle, Angle)> {
        // only test with at least 3 tiles
        if self.path.len() > 2 {
            let mut intersections = Vec::new();

            // get latest tile
            let last = self.path.last().unwrap();

            // skip the last and connecting tile as they cant intersect
            for (i, tile) in self.path[..self.path.len() - 2].iter().enumerate() {
                // find every intersections of `tile` and `last`
                // (section, last_angle, tile_angle)
                for (section, last_angle, tile_angle) in last.intersects(tile) {
                    intersections.push((section, i, last_angle, tile_angle))
                }
            }

            intersections
        } else {
            Vec::new()
        }
    }

    /// Generate all polygons from the new intersections.
    pub fn polys(
        &self,
        intersections: &Vec<(Vec2D<f64>, usize, Angle, Angle)>,
    ) -> Vec<Vec<Vec2D<f64>>> {
        let mut polys = Vec::new();

        let last = self.path.last().unwrap();

        for (section, i, last_angle, tile_angle) in intersections {
            let mut poly = Vec::new();

            let mut cut_tile = self.path[*i].clone();
            cut_tile.off = cut_tile.off - cut_tile.start.min_dist(*tile_angle);
            cut_tile.start = *tile_angle;

            poly.append(&mut cut_tile.poly(DETAIL));

            for curve in self.path[i + 1..self.path.len() - 1].iter() {
                poly.append(&mut curve.poly(DETAIL));
            }

            let mut cut_last = last.clone();
            cut_last.off = cut_last.off - last_angle.min_dist(cut_last.start + cut_last.off);

            poly.append(&mut cut_last.poly(DETAIL));

            poly.push(poly.first().unwrap().clone());
            polys.push(poly);
        }

        polys
    }
}

/// Compute the winding number of a polygon $P_n = P_0$ around a point.
/// ![](http://web.archive.org/web/20210504233957/http://geomalgorithms.com/a03-_inclusion.html)
/// Copyright 2001, 2012, 2021 Dan Sunday
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// There is no warranty for this code, and the author of it cannot
// be held liable for any real or imagined damage from its use.
// Users of this code must verify correctness for their application.
fn winding_number(point: Vec2D<f64>, poly: &Vec<Vec2D<f64>>) -> i32 {
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
