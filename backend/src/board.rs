use std::collections::BTreeSet;

use crate::curve::Curve;
use graph::{EdgesGraph, Graph};
use math::Vec2D;

use crate::arrow::{Arrow, Rotation};
use crate::tile::Tile;

const CURVE_RESOULUTION: usize = 11;

pub struct Board {
    step: u8,
    active: Player,
    pub arrow: Arrow,
    pub graph: EdgesGraph<Vec2D<f64>, Curve>,
    pub tiles: BTreeSet<Tile>,
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

        let (start, mid, end) = self.arrow.bezier_control_points(&tile);

        if !self.is_double(start, mid, end) {
            let curve = Curve::bezier(start.into(), mid.into(), end.into(), CURVE_RESOULUTION);
            self.graph.fit_edge(start.into(), end.into(), curve);
        }

        self.split_curves();

        self.arrow.position = end;
        self.arrow.rotation = self.arrow.rotation + tile.horizontal.into();

        self.active = match self.active {
            Player::Alpha => Player::Beta,
            Player::Beta => Player::Alpha,
        };

        if self.arrow.position.x < 0
            || self.arrow.position.x > 10
            || self.arrow.position.y < 0
            || self.arrow.position.y > 10
        {
            self.state = State::Won(self.active);
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
    Won(Player),
}
