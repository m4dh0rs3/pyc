use std::collections::BTreeSet;

use crate::curve::{Curve, Intersection};
use graph::EdgesGraph;
use math::Vec2D;

use crate::arrow::{Arrow, Rotation};
use crate::tile::Tile;

const CURVE_RESOULUTION: usize = 12;

pub struct Board {
    step: u8,
    active: Player,
    arrow: Arrow,
    graph: EdgesGraph<Vec2D<i8>, Curve>,
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
                position: origin,
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

        let mut backwards: bool = false;

        if let Some((_, curve)) = self.graph.edges.last() {
            if curve.mid == mid
                && ((curve.start == start && curve.end == end)
                    || (curve.start == end && curve.end == start))
            {
                backwards = true;
            }
        }

        if !backwards {
            let curve = Curve::bezier(start.into(), mid.into(), end.into(), CURVE_RESOULUTION);
            self.graph.fit_edge(start, end, curve);
        }

        self.arrow.position = end;
        self.arrow.rotation = self.arrow.rotation + (*tile.horizontal()).into();

        self.active = match self.active {
            Player::Alpha => Player::Beta,
            Player::Beta => Player::Alpha,
        }
    }

    fn intersections(&self) -> Vec<PathIntersection> {
        let mut path_intersections = Vec::new();

        for (i, (_, curve)) in self.graph.edges().iter().enumerate() {
            for (j, (_, other)) in self.graph.edges().iter().enumerate() {
                path_intersections.append(
                    &mut curve
                        .intersections(&other)
                        .into_iter()
                        .map(|s| PathIntersection { at: s, i, j })
                        .collect(),
                );
            }
        }

        path_intersections
    }

    pub fn intersection_points(&self) -> Vec<Vec2D<f64>> {
        self.intersections().into_iter().map(|s| s.at.at).collect()
    }

    pub fn tiles(&self) -> &BTreeSet<Tile> {
        &self.tiles
    }

    pub fn arrow(&self) -> &Arrow {
        &self.arrow
    }

    pub fn graph(&self) -> &EdgesGraph<Vec2D<i8>, Curve> {
        &self.graph
    }
}

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
