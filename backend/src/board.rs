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
    polys: HashSet<Vec<usize>>,
}

/// Enum of possible players.
/// [`Player::Gamma`] inspired by GAMMAGRAPHICS.
#[derive(Clone, Copy)]
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
            polys: HashSet::new(),
        }
    }
}

const PATH_RES: usize = 8;

impl Board {
    /// Step by choosing a tile. Panics if index is invalid.
    pub fn step(&mut self, tile: usize) {
        self.set_tile(tile);
        self.split_curves(self.graph.edges.len() - 1);
        self.check_points();
    }

    /// Set a tile on the [`Board`].
    fn set_tile(&mut self, tile: usize) {
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

    fn check_points(&mut self) {
        let polys = self.generate_polys();

        for i in 0..self.size as usize {
            for j in 0..self.size as usize {
                let point = Vec2D::new(i as f64, j as f64);

                for poly in &polys {
                    if self.point_in_poly(point, poly) {
                        self.points[j][i] = Some(self.active)
                    }
                }
            }
        }
    }

    fn generate_polys(&mut self) -> Vec<Vec<Vec2D<f64>>> {
        let cycles: Vec<Vec<usize>> = self
            .graph
            .cycles()
            .into_iter()
            .filter(|cycle| !self.polys.contains(cycle))
            .collect();

        cycles.iter().for_each(|cycle| {
            self.polys.insert(cycle.clone());
        });

        let mut polys = Vec::new();

        for cycle in cycles {
            polys.append(&mut self.continue_poly(Vec::new(), cycle));
        }

        polys
    }

    fn continue_poly(
        &self,
        mut poly: Vec<Vec2D<f64>>,
        mut cycle: Vec<usize>,
    ) -> Vec<Vec<Vec2D<f64>>> {
        let mut polys = Vec::new();

        if cycle.len() > 1 {
            let mut i = cycle.remove(0);
            for (n, j) in cycle.iter().enumerate() {
                let mut edges = self.graph.get_edges(i, *j);

                if edges.len() > 0 {
                    poly.append(&mut edges.remove(0).path(PATH_RES));

                    for edge in self.graph.get_edges(i, *j) {
                        polys.append(&mut self.continue_poly(poly.clone(), cycle[n..].to_owned()));
                    }
                }

                i = *j;
            }

            polys.push(poly);
        }

        polys
    }

    fn cycle_to_polys(&self, cycle: Vec<usize>, polys: &mut Vec<Vec<Vec2D<f64>>>) {
        for node in cycle {}
    }

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

    pub fn get_point(&self, i: usize, j: usize) -> Option<Player> {
        self.points.get(j)?.get(i)?.clone()
    }

    /// Returns current position of [`Arrow`].
    pub fn get_position(&self) -> Vec2D<i8> {
        self.arrow.position
    }

    /// Returns current angle of [`Arrow`].
    pub fn get_angle(&self) -> Angle {
        self.arrow.angle
    }

    pub fn get_polys(&self) -> &HashSet<Vec<usize>> {
        &self.polys
    }

    fn point_in_poly(&self, point: Vec2D<f64>, poly: &Vec<Vec2D<f64>>) -> bool {
        if poly.len() < 3 {
            return false;
        }

        let mut intersects = Vec::new();

        for n in 0..RAY_TESTS {
            let ray = point
                + Vec2D::from_polar(
                    remap(n as f64, 0.0, RAY_TESTS as f64, 0.0, TAU).into(),
                    self.size as f64,
                );

            let mut count = 0;

            let first = poly.first().expect("Poly is empty");
            for second in &poly[1..] {
                if segment_intersection(point, ray, *first, *second).is_some() {
                    count += 1
                }

                let first = second;
            }

            intersects.push(count);
        }

        if let Some(mode) = mode(intersects) {
            mode % 2 == 1
        } else {
            false
        }
    }
}

const RAY_TESTS: usize = 6;

use std::{collections::HashMap, f64::consts::TAU};
use std::{collections::HashSet, hash::Hash};

fn mode<T: Eq + Hash>(vec: Vec<T>) -> Option<T> {
    let mut values: HashMap<T, usize> = HashMap::new();

    for value in vec {
        values
            .entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(0);
    }

    let max = 0;
    let mut mode: Option<T> = None;

    for value in values {
        if value.1 > max {
            mode = Some(value.0);
        }
    }

    mode
}

fn segment_intersection(
    p1: Vec2D<f64>,
    p2: Vec2D<f64>,
    r1: Vec2D<f64>,
    r2: Vec2D<f64>,
) -> Option<Vec2D<f64>> {
    let s1 = p2 - p1;
    let s2 = r2 - r1;

    let k = s1.cross_zero(s2);

    if k == 0.into() {
        return None;
    }

    let d = p1 - r1;

    let s = s1.cross_zero(d) / k;
    let t = s2.cross_zero(d) / k;

    /* let s = (-s1.y * (p1.x - r1.x) + s1.x * (p1.y - r1.y)) / k;
    let t = (s2.x * (p1.y - r1.y) - s2.y * (p1.x - r1.x)) / k; */

    if s > 0.into() && s < 1.into() && t > 0.into() && t < 1.into() {
        Some(Vec2D::new(p1.x + t * s1.x, p1.y + t * s1.y))
    } else {
        None
    }
}

fn aabb_intersection(p1: Vec2D<f64>, p2: Vec2D<f64>, r1: Vec2D<f64>, r2: Vec2D<f64>) -> bool {
    p1.x <= r2.x && p2.x >= r1.x && p1.y <= r2.y && p2.y >= r1.y
}
