use std::collections::BTreeSet;

use crate::curve::{Curve, Intersection};
use graph::{EdgesGraph, Graph};
use math::Vec2D;

use crate::arrow::{Arrow, Rotation};
use crate::tile::Tile;

const CURVE_RESOULUTION: usize = 24;

pub struct Board {
    step: u8,
    active: Player,
    arrow: Arrow,
    graph: EdgesGraph<Vec2D<f64>, Curve>,
    tiles: BTreeSet<Tile>,
    points: [[Option<Player>; 11]; 11],
    score: Score,
    state: State,
}

impl Board {
    pub fn empty_start(origin: Vec2D<i8>) -> Self {
        Self {
            step: 0,
            active: Player::Alpha,
            arrow: Arrow {
                position: origin.into(),
                rotation: Rotation::Left,
            },
            graph: EdgesGraph::with_capacity(13, 15),
            tiles: BTreeSet::new(),
            points: [[None; 11]; 11],
            score: Score { alpha: 0, beta: 0 },
            state: State::Undecided,
        }
    }

    pub fn step(&mut self, tile: Tile) {
        self.step += 1;

        if self.tiles.contains(&tile) {
            self.state = State::Invalid;
        } else {
            self.tiles.insert(tile);
        }

        let (start, mid, end) = self.arrow.control_points(&tile);

        if !self.is_double(start, mid, end) {
            let curve = Curve::bezier(start.into(), mid.into(), end.into(), CURVE_RESOULUTION);
            self.graph.fit_edge(start.into(), end.into(), curve);
        }

        self.split_curves();

        self.arrow.position = end;
        self.arrow.rotation = self.arrow.rotation + (*tile.horizontal()).into();

        self.active = match self.active {
            Player::Alpha => Player::Beta,
            Player::Beta => Player::Alpha,
        }
    }

    fn is_double(&self, start: Vec2D<i8>, mid: Vec2D<i8>, end: Vec2D<i8>) -> bool {
        for (_, edge) in &self.graph.edges {
            if mid == edge.mid
                && ((start == edge.start && end == edge.end)
                    || (start == edge.end && end == edge.start))
            {
                return true;
            }
        }

        false
    }

    fn last_intersections(&self) -> Vec<PathIntersection> {
        let mut path_intersections = Vec::new();
        let curve = &self.graph.edges().last().unwrap().1;
        let i = self.graph.edges().len() - 1;

        for (j, (_, other)) in self.graph.edges().iter().enumerate() {
            path_intersections.append(
                &mut curve
                    .intersections(&other)
                    .into_iter()
                    .map(|s| PathIntersection { at: s, i, j })
                    .collect(),
            );
        }

        path_intersections
    }

    fn split_curves(&mut self) {
        let mut i = 0;

        while i < self.graph.edges.len() {
            let curve_len = self.graph.edges[i].1.path.len();
            let mut j = 0;

            'inner: while j < self.graph.edges.len() {
                // comparing the differnce may not work on split up curves, but did till now...
                if i.max(j) - i.min(j) > 1 {
                    if &self.graph.edges[i].1 != &self.graph.edges[j].1 {
                        let mut k = 0;
                        let other_len = self.graph.edges[j].1.path.len();

                        while k + 1 < curve_len {
                            let mut l = 0;

                            while l + 1 < other_len {
                                if let Some(intersection) = Vec2D::intersect(
                                    self.graph.edges[i].1.path[k],
                                    self.graph.edges[i].1.path[k + 1],
                                    self.graph.edges[j].1.path[l],
                                    self.graph.edges[j].1.path[l + 1],
                                ) {
                                    let ((curve_start, curve_end), curve) =
                                        self.graph.edges.remove(i);
                                    let ((other_start, other_end), other) =
                                        self.graph.edges.remove(if j < i { j } else { j - 1 });

                                    let (curve_one, curve_two) = curve.path.split_at(k);
                                    let (curve_one, curve_two) = (
                                        Curve {
                                            start: curve.start,
                                            mid: curve.mid,
                                            end: curve.end,
                                            path: {
                                                let mut curve_one = curve_one.to_vec();
                                                curve_one.push(intersection);
                                                curve_one
                                            },
                                        },
                                        Curve {
                                            start: curve.start,
                                            mid: curve.mid,
                                            end: curve.end,
                                            path: {
                                                let mut curve_tow = curve_two.to_vec();
                                                curve_tow[0] = intersection;
                                                curve_tow
                                            },
                                        },
                                    );

                                    let (other_one, other_two) = other.path.split_at(l);
                                    let (other_one, other_two) = (
                                        Curve {
                                            start: other.start,
                                            mid: other.mid,
                                            end: other.end,
                                            path: {
                                                let mut other_one = other_one.to_vec();
                                                other_one.push(intersection);
                                                other_one
                                            },
                                        },
                                        Curve {
                                            start: other.start,
                                            mid: other.mid,
                                            end: other.end,
                                            path: {
                                                let mut other_two = other_two.to_vec();
                                                other_two[0] = intersection;
                                                other_two
                                            },
                                        },
                                    );

                                    let node = self.graph.push_node(intersection);
                                    self.graph.insert_edge(curve_start, node, curve_one);
                                    self.graph.insert_edge(node, curve_end, curve_two);
                                    self.graph.insert_edge(other_start, node, other_one);
                                    self.graph.insert_edge(node, other_end, other_two);

                                    i = 0;
                                    //j -= 2;

                                    break 'inner;
                                }

                                l += 1;
                            }

                            k += 1;
                        }
                    }
                }

                j += 1;
            }

            i += 1;
        }
    }

    pub fn intersection_points(&self) -> Vec<Vec2D<f64>> {
        self.last_intersections()
            .into_iter()
            .map(|s| s.at.at)
            .collect()
    }

    pub fn tiles(&self) -> &BTreeSet<Tile> {
        &self.tiles
    }

    pub fn arrow(&self) -> &Arrow {
        &self.arrow
    }

    pub fn graph(&self) -> &EdgesGraph<Vec2D<f64>, Curve> {
        &self.graph
    }
}

#[derive(Clone, Copy)]
struct PathIntersection {
    at: Intersection,
    i: usize,
    j: usize,
}

#[derive(Clone, Copy)]
enum Player {
    Alpha,
    Beta,
}

struct Score {
    alpha: u8,
    beta: u8,
}

enum State {
    Undecided,
    Invalid,
    Draw,
    AlphaWon,
    BetaWon,
}
