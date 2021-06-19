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
const DETAIL: u8 = 12;

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
        // only test with at least 3 tiles
        if self.path.len() > 2 {
            // get latest tile
            let last = self.path.last().unwrap();

            // skip the last and connecting tile as they cant intersect
            for (i, tile) in self.path[..self.path.len() - 2].iter().enumerate() {
                // find every intersections of `tile` and `last`
                for (section, last_angle, tile_angle) in last.intersects(tile) {
                    // iterate through all free points. could be optimized with `flatten` and `filter`
                    for (i, points) in self.points.iter().enumerate() {
                        for (j, point) in points.iter().enumerate() {
                            if let None = point {
                                let mut winding = 0.0;

                                for curve in self.path[i..].iter() {
                                    for n in 0..DETAIL {
                                        //winding += curve.mid + Vec2D::from_polar(curve.start + curve.off * n as f64 / DETAIL as f64, curve.radius)
                                    }
                                }

                                /* ![](http://web.archive.org/web/20210504233957/http://geomalgorithms.com/a03-_inclusion.html)
                                // Copyright 2001, 2012, 2021 Dan Sunday
                                // This code may be freely used and modified for any purpose
                                // providing that this copyright notice is included with it.
                                // There is no warranty for this code, and the author of it cannot
                                // be held liable for any real or imagined damage from its use.
                                // Users of this code must verify correctness for their application.


                                // a Point is defined by its coordinates {int x, y;}
                                //===================================================================


                                // isLeft(): tests if a point is Left|On|Right of an infinite line.
                                //    Input:  three points P0, P1, and P2
                                //    Return: >0 for P2 left of the line through P0 and P1
                                //            =0 for P2  on the line
                                //            <0 for P2  right of the line
                                inline int
                                isLeft( Point P0, Point P1, Point P2 )
                                {
                                    return ( (P1.x - P0.x) * (P2.y - P0.y)
                                            - (P2.x -  P0.x) * (P1.y - P0.y) );
                                }
                                //===================================================================


                                // cn_PnPoly(): crossing number test for a point in a polygon
                                //      Input:   P = a point,
                                //               V[] = vertex points of a polygon V[n+1] with V[n]=V[0]
                                //      Return:  0 = outside, 1 = inside
                                // This code is patterned after [Franklin, 2000]
                                int
                                cn_PnPoly( Point P, Point* V, int n )
                                {
                                    int    cn = 0;    // the  crossing number counter

                                    // loop through all edges of the polygon
                                    for (int i=0; i<n; i++) {    // edge from V[i]  to V[i+1]
                                    if (((V[i].y <= P.y) && (V[i+1].y > P.y))     // an upward crossing
                                        || ((V[i].y > P.y) && (V[i+1].y <=  P.y))) { // a downward crossing
                                            // compute  the actual edge-ray intersect x-coordinate
                                            float vt = (float)(P.y  - V[i].y) / (V[i+1].y - V[i].y);
                                            if (P.x <  V[i].x + vt * (V[i+1].x - V[i].x)) // P.x < intersect
                                                ++cn;   // a valid crossing of y=P.y right of P.x
                                        }
                                    }
                                    return (cn&1);    // 0 if even (out), and 1 if  odd (in)

                                }
                                //===================================================================


                                // wn_PnPoly(): winding number test for a point in a polygon
                                //      Input:   P = a point,
                                //               V[] = vertex points of a polygon V[n+1] with V[n]=V[0]
                                //      Return:  wn = the winding number (=0 only when P is outside)
                                int
                                wn_PnPoly( Point P, Point* V, int n )
                                {
                                    int    wn = 0;    // the  winding number counter

                                    // loop through all edges of the polygon
                                    for (int i=0; i<n; i++) {   // edge from V[i] to  V[i+1]
                                        if (V[i].y <= P.y) {          // start y <= P.y
                                            if (V[i+1].y  > P.y)      // an upward crossing
                                                if (isLeft( V[i], V[i+1], P) > 0)  // P left of  edge
                                                    ++wn;            // have  a valid up intersect
                                        }
                                        else {                        // start y > P.y (no test needed)
                                            if (V[i+1].y  <= P.y)     // a downward crossing
                                                if (isLeft( V[i], V[i+1], P) < 0)  // P right of  edge
                                                    --wn;            // have  a valid down intersect
                                        }
                                    }
                                    return wn;
                                } */
                            }
                        }
                    }
                }
            }
        }
    }
}
