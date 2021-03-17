use crate::Curve;
use core::f64;
use graph::EdgesGraph;
use graph::Graph;
use math::Vec2D;
use std::collections::BTreeSet;

pub struct Board {
    pub graph: EdgesGraph<Vec2D, Curve>,
    pub points: Vec<Vec<Point>>,
    pub tiles: BTreeSet<Tile>,
    pub arrow: (Vec2D, f64),
    pub score: (u8, u8),
    pub step: u8,
    pub game_state: GameState,
}

pub enum GameState {
    Undecided,
    Invalid,
    AWon,
    BWon,
    Draw,
}

#[derive(Debug, Clone)]
pub enum Point {
    Free,
    TakenA,
    TakenB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    pub dir: TileDir,
    pub radius: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileDir {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Tile {
    pub fn control(&self, start: Vec2D, angle: f64) -> (Vec2D, Vec2D) {
        let mid = start
            + Vec2D::from_polar(
                angle,
                match &self.dir {
                    TileDir::UpLeft | TileDir::UpRight => self.radius,
                    TileDir::DownLeft | TileDir::DownRight => -self.radius,
                } as f64,
            );

        let end = mid
            + Vec2D::from_polar(
                angle + std::f64::consts::FRAC_PI_2,
                match &self.dir {
                    TileDir::UpLeft | TileDir::DownLeft => -self.radius,
                    TileDir::UpRight | TileDir::DownRight => self.radius,
                } as f64,
            );

        (mid, end)
    }

    pub fn rotation(&self) -> f64 {
        match self.dir {
            TileDir::UpLeft | TileDir::DownLeft => -std::f64::consts::FRAC_PI_2,
            TileDir::UpRight | TileDir::DownRight => std::f64::consts::FRAC_PI_2,
        }
    }
}

impl Board {
    pub fn new(start: Vec2D) -> Self {
        Self {
            graph: {
                let mut graph = EdgesGraph::new();
                graph.push_node(start);
                graph
            },
            points: vec![vec![Point::Free; 11]; 11],
            tiles: BTreeSet::new(),
            arrow: (start, -std::f64::consts::FRAC_PI_2),
            score: (0, 0),
            step: 0,
            game_state: GameState::Undecided,
        }
    }

    pub fn tile(&mut self, tile: Tile) {
        self.step += 1;

        if !self.tiles.insert(tile) {
            self.game_state = GameState::Invalid;
        }

        let (mid, end) = tile.control(self.arrow.0, self.arrow.1);

        let detail = 12;

        self.graph.fit_edge(
            self.arrow.0,
            end,
            Curve::bezier(self.arrow.0, mid, end, detail),
        );

        self.arrow.1 += tile.rotation();
        self.arrow.0 = end;
    }
}
