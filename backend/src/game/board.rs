use super::curve::{Curve, Intersection, Path};
use crate::{game::DELTA, math::prelude::*, Float};

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
        self.update_score();

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
        // removes and returns the tile, panics if the index is out of bounds
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

    /// Check for polygons and collect points.
    fn update_score(&mut self) {
        let ints = self.latest_intersections();
        let polys = self.polys(ints);
        self.check_points(polys); // , points);
    }

    /// Find all new intersections with the last tile and the path.
    pub fn latest_intersections(&self) -> Vec<(usize, Intersection)> {
        // only test with at least 3 tiles
        if self.path.len() > 2 {
            let mut ints = Vec::new();

            // get latest tile
            let last = self.path.last().unwrap();

            // skip the last and connecting tile as they cant intersect
            for (i, tile) in self.path[..self.path.len() - 2].iter().enumerate() {
                // find every intersections of `tile` and `last`
                // (section, last_angle, tile_angle)
                // TODO: make map
                for params in last.intersects(tile) {
                    ints.push((i, params))
                }
            }

            ints
        } else {
            Vec::new()
        }
    }

    /// Generate all polygons from the new intersections.
    pub fn polys(&self, ints: Vec<(usize, Intersection)>) -> Vec<Path> {
        // , Vec<Vec2D<i8>>) {
        /* // -> Box<dyn Iterator<Item = Path<'a>>>
        Box::new(ints.iter().map(|(i, (last_t, tile_t))| {
            self.path[*i]
                .path(*tile_t, 1.0)
                .chain(
                    self.path[i + 1..self.path.len() - 1]
                        .iter()
                        .map(|tile| tile.path(0.0, 1.0))
                        .flatten(),
                )
                .chain(self.path.last().unwrap().path(0.0, *last_t))
                .chain(std::iter::once(self.path[*i].point(0.0)))
        })) */

        // points that are on polys can be directly included,
        // and are not always captured by the wn algorithm
        // let mut points = Vec::new();

        // TODO: replace with map
        let mut polys = Vec::new();

        for (i, (last_t, tile_t)) in ints {
            let mut poly = Vec::new();

            // TODO: merge collect into chain?
            let mut tile: Vec<Vec2D<Float>> = self.path[i].path(tile_t, 1.0);
            // points.push(self.path[i].end);
            poly.append(&mut tile);

            for tile in self.path[i + 1..self.path.len() - 1].iter() {
                poly.append(&mut tile.path(0.0, 1.0));
                // points.push(tile.end);
            }

            let mut last: Vec<Vec2D<Float>> = self.path.last().unwrap().path(0.0, last_t);
            poly.append(&mut last);

            poly.push(poly.first().unwrap().clone());
            polys.push(poly);
        }

        polys // , Vec::new())
    }

    fn check_points(&mut self, polys: Vec<Path>) {
        // , points: Vec<Vec2D<i8>>) {
        for poly in polys {
            // iterate through all free points. could be optimized with `flatten` and `filter`
            // self.points.iter().enumerate().map(|(i, points)| points.iter().filter(|point| point.is_none()).map(||));
            for (j, points) in self.points.iter_mut().enumerate() {
                for (i, point) in points.iter_mut().enumerate() {
                    if let None = point {
                        // the crossing number algorithm does not work for non-simple polys
                        // thats why we have to use the winding number algorithm
                        /* if winding_number(
                            Vec2D {
                                x: i as Float,
                                y: j as Float,
                            },
                            &poly,
                        ) != 0 */

                        if vec![
                            Vec2D {
                                x: i as Float,
                                y: j as Float,
                            },
                            Vec2D {
                                x: i as Float,
                                y: j as Float - DELTA,
                            },
                            Vec2D {
                                x: i as Float + DELTA,
                                y: j as Float,
                            },
                            Vec2D {
                                x: i as Float,
                                y: j as Float + DELTA,
                            },
                            Vec2D {
                                x: i as Float - DELTA,
                                y: j as Float,
                            },
                        ]
                        .into_iter()
                        .map(|variant| winding_number(variant, &poly))
                        .sum::<i32>()
                            != 0
                        {
                            *point = Some(self.active);
                        }
                    }
                }
            }
        }

        /* for point in points {
            if point.x >= 0
                && (point.x as usize) < self.points.len()
                && point.y >= 0
                && (point.y as usize) < self.points.len()
            {
                let board_point = &mut self.points[point.y as usize][point.x as usize];
                if let None = board_point {
                    *board_point = Some(self.active);
                }
            }
        } */
    }
}

/// Compute the winding number of a polygon $P_n = P_0$ around a point.
/// ![Copyright 2001, 2012, 2021 Dan Sunday](http://web.archive.org/web/20210504233957/http://geomalgorithms.com/a03-_inclusion.html)
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// There is no warranty for this code, and the author of it cannot
// be held liable for any real or imagined damage from its use.
// Users of this code must verify correctness for their application.
fn winding_number(point: Vec2D<Float>, poly: &Vec<Vec2D<Float>>) -> i32 {
    if poly.len() < 3 {
        0
    } else {
        let mut wn = 0;

        for i in 0..poly.len() - 1 {
            if poly[i].y <= point.y {
                if poly[i + 1].y > point.y {
                    if point.is_left(poly[i], poly[i + 1]) > 0.0 {
                        wn += 1;
                    }
                }
            } else if poly[i + 1].y <= point.y {
                if point.is_left(poly[i], poly[i + 1]) < 0.0 {
                    wn -= 1;
                }
            }
        }

        wn
    }
}
