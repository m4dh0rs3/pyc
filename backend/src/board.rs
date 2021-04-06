use crate::{arrow::Arrow, curve::Curve, graph::Graph};
use math::prelude::*;

/// The subject of the game is the [`Board`].
/// It holds the current state and all data.
#[derive(Clone)]
pub struct Board {
    active: Player,
    step: u8,
    arrow: Arrow,
    // reduced precision for node fitting
    graph: Graph<Vec2D<f64>, Curve>,
    // remainig possible tiles
    // this is not a BTreeSet because wave collapse
    // is easier to iterate over remainders than to
    // generate f64
    tiles: Vec<Curve>,
    size: u8,
    points: Vec<Vec<Option<Player>>>,
    state: State,
    score: Score,
}

/// Enum of possible players.
/// [`Player::Gamma`] inspired by GAMMAGRAPHICS.
#[derive(Clone)]
pub enum Player {
    Gamma,
    Delta,
}

/// The possible states the [`Board`] can be in.
/// They are exclusive and alter the behavior
/// of methods called on the [`Board`].
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
#[derive(Clone)]
struct Score {
    gamma: u16,
    delta: u16,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            active: Player::Gamma,
            step: 0,
            arrow: Arrow::new(Vec2D::new(5, 5), Angle::new(0.0)),
            graph: Graph::with_capacity(22, 22),
            tiles: Curve::convex_4x3(),
            size: 11,
            points: vec![vec![None; 11]; 11],
            state: State::Pending,
            score: Score { gamma: 0, delta: 0 },
        }
    }
}

impl Board {
    /// Set a tile.
    pub fn step(&mut self, tile: usize) {
        use std::f64::consts::PI;

        let mut tile = self.tiles.remove(tile);

        tile.start = tile.start + self.arrow.angle + PI.into();
        tile.end = tile.end + self.arrow.angle + PI.into();

        tile.mid = self.arrow.position
            - Into::<Vec2D<i8>>::into(Vec2D::from_polar(tile.start, tile.radius as f64));
        let end = Into::<Vec2D<f64>>::into(tile.mid)
            + Vec2D::from_polar(tile.end, tile.radius as f64).round();

        self.arrow.angle = tile.start - PI.into();

        self.graph.clip_edge(self.arrow.position.into(), end, tile);

        self.arrow.position = end.into();

        self.split_curves(self.graph.edges.len() - 1);
    }

    /// Split curves at intersections. Takes the index of a new [`Curve`].
    #[rustfmt::skip]
    fn split_curves(&mut self, i: usize) {
        let mut j = 0;

        while j < self.graph.edges.len() {
            if i != j {
                // mid points must be different
                if self.graph.edges[i].1.mid != self.graph.edges[j].1.mid {
                    // nodes must be different
                    if self.graph.nodes[self.graph.edges[i].0.0] != self.graph.nodes[self.graph.edges[j].0.0] &&
                        self.graph.nodes[self.graph.edges[i].0.0] != self.graph.nodes[self.graph.edges[j].0.1] &&
                        self.graph.nodes[self.graph.edges[i].0.1] != self.graph.nodes[self.graph.edges[j].0.0] &&
                        self.graph.nodes[self.graph.edges[i].0.1] != self.graph.nodes[self.graph.edges[j].0.1]
                    {
                        match self.graph.edges[i].1.intersect(&self.graph.edges[j].1)[..] {
                            [(point, curve_angle, other_angle)] => {
                                let j = if j > i { j - 1 } else { j };

                                let ((first_start_node, first_end_node), curve) =
                                    self.graph.edges.remove(i);
                                let ((second_start_node, second_end_node), other) =
                                    self.graph.edges.remove(j);
                                let (first_curve, second_curve) = curve.split_at(curve_angle);
                                let (first_other, second_other) = other.split_at(other_angle);

                                let point = self.graph.clip_node(point.into());

                                self.graph.insert_edge(point, first_end_node, second_curve);
                                self.graph.insert_edge(first_start_node, point, first_curve);

                                self.graph.insert_edge(point, second_end_node, second_other);
                                self.graph
                                    .insert_edge(second_start_node, point, first_other);
                            }

                            _ => {}
                        }
                    }
                }
            }

            j += 1;
        }
    }

    fn split_two_curves(&mut self, i: usize, j: usize) {}

    /// Create a specific board.
    pub fn new(first_player: Player, arrow: Arrow, tiles: Vec<Curve>, size: u8) -> Self {
        Self {
            active: first_player,
            arrow,
            graph: Graph::with_capacity(tiles.len() * 2, tiles.len() * 2),
            tiles,
            size,
            points: vec![vec![None; size as usize]; size as usize],
            ..Board::default()
        }
    }

    /// Show remainding tiles
    pub fn options(&self) -> &Vec<Curve> {
        &self.tiles
    }

    /// Returns the number of points in length.
    pub fn get_size(&self) -> u8 {
        self.size
    }

    /// Returns all nodes, on points or intersections.
    pub fn get_nodes(&self) -> &Vec<Vec2D<f64>> {
        &self.graph.nodes
    }

    /// Returns all curves, which may be split up.
    pub fn get_edges(&self) -> &Vec<((usize, usize), Curve)> {
        &self.graph.edges
    }

    /// Returns current position of [`Arrow`].
    pub fn get_position(&self) -> Vec2D<i8> {
        self.arrow.position
    }

    /// Returns current angle of [`Arrow`].
    pub fn get_angle(&self) -> Angle {
        self.arrow.angle
    }
}
