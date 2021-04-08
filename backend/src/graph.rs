//! Graph structure
//! Based on: [Optimal Listing of Cycles and st-Vec<usize>s in Undirected Graphs](https://arxiv.org/pdf/1205.2766.pdf)

use math::prelude::Vec2D;

/// Graph of nodes, which may be connected by edges.
/// Both can hold values of type [`N`] and [`E`].
/// There can't be multiple nodes of the same value,
/// but there may be equal edges.
#[derive(Clone)]
pub(crate) struct Graph<N, E> {
    // This is not an BTreeSet, because f64 does not implement Eq.
    // It should be changed for performance improvements.
    pub(crate) nodes: Vec<N>,
    // This is not BTreeSet, because order should be preserved.
    pub(crate) edges: Vec<((usize, usize), E)>,
}

impl<N, E> Graph<N, E> {
    /// Constructs an empty [`Graph`].
    pub(crate) fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Constructs an empty [`Graph`] with given capacities.
    pub(crate) fn with_capacity(nodes: usize, edges: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(nodes),
            edges: Vec::with_capacity(edges),
        }
    }

    /// Insert edge at specific place in the stack of edges, e.g. to preserve order.
    pub(crate) fn insert_edge_at(&mut self, at: usize, i: usize, j: usize, edge: E) {
        self.edges.insert(at, ((i, j), edge))
    }

    /// May return the reference to a node, given a valid key.
    pub(crate) fn get_node(&self, key: usize) -> Option<&N> {
        self.nodes.get(key)
    }

    /// Removes and returns a node given an valid key.
    pub(crate) fn remove_node(&mut self, key: usize) -> Option<N> {
        if key >= self.nodes.len() {
            None
        } else {
            self.edges.retain(|((i, j), _)| &key != i || &key != j);

            for ((i, j), _) in self.edges.iter_mut() {
                if &*j >= &key {
                    *j -= 1;
                }

                if &*i >= &key {
                    *i -= 1;
                }
            }

            Some(self.nodes.remove(key))
        }
    }

    /// May return a reference to the value of an edge, given valid keys.
    pub(crate) fn get_edge(&self, i: usize, j: usize) -> Option<&E> {
        self.edges
            .iter()
            .position(|((k, l), _)| &i == k && &j == l)
            .map(|key| self.edges.get(key).map(|(_, edge)| edge))
            .flatten()
    }

    /// Returns all edges connecting i and j.
    pub(crate) fn get_edges(&self, i: usize, j: usize) -> Vec<&E> {
        self.edges
            .iter()
            .filter(|((k, l), _)| (i == *k && *l == j) || (i == *l && j == *k))
            .map(|(_, edge)| edge)
            .collect()
    }

    /// Insert an value of an edge, connecting two unchecked keys.
    pub(crate) fn insert_edge(&mut self, i: usize, j: usize, edge: E) {
        self.edges.push(((i, j), edge))
    }

    /// Remove all edges connecting two nodes.
    pub(crate) fn remove_edge(&mut self, i: usize, j: usize) {
        self.edges.retain(|((k, l), _)| !(&i == k && &j == l))
    }

    /* /// Returns references of nodes from connected by edge.
    pub(crate) fn get_nodes_from_edge(&self, i: usize) -> (&N, &N) {
        (
            &self.nodes[self.edges[i].0 .0],
            &self.nodes[self.edges[i].0 .1],
        )
    } */
}

impl<N, E> Graph<N, E>
where
    N: PartialEq,
{
    /// Insert node or return key to already existing equal.
    pub(crate) fn push_node(&mut self, node: N) -> usize {
        match self.nodes.iter().position(|other| &node == other) {
            Some(i) => i,
            None => {
                self.nodes.push(node);
                self.nodes.len() - 1
            }
        }
    }

    /// Reuse nodes of the same value and connect to the old one instead.
    pub(crate) fn fit_edge(&mut self, start: N, end: N, edge: E) {
        let i = self.push_node(start);
        let j = self.push_node(end);

        self.insert_edge(i, j, edge);
    }
}

const CLIP: f64 = 0.05;

impl<E> Graph<Vec2D<f64>, E> {
    /// Insert node or return key to already existing about equal.
    pub(crate) fn clip_node(&mut self, node: Vec2D<f64>) -> usize {
        match self.nodes.iter().position(|other| node.dist(*other) < CLIP) {
            Some(i) => i,
            None => {
                self.nodes.push(node);
                self.nodes.len() - 1
            }
        }
    }

    /// Reuse nodes of the about equal value and connect to the old one instead.
    pub(crate) fn clip_edge(&mut self, start: Vec2D<f64>, end: Vec2D<f64>, edge: E) {
        let i = self.clip_node(start);
        let j = self.clip_node(end);

        self.insert_edge(i, j, edge);
    }
}

impl<N, E> Graph<N, E> {
    /// Return all cycles in the graph.
    pub(crate) fn cycles(&self) -> Vec<Vec<usize>> {
        let mut cycles = Vec::new();

        for (edge, _) in &self.edges {
            dfs_cycle::<E>(&self.edges, &mut cycles, vec![edge.0]);
            dfs_cycle::<E>(&self.edges, &mut cycles, vec![edge.1]);
        }

        cycles
    }
}

/// Depth-first search trough edges for cycles.
fn dfs_cycle<E>(edges: &Vec<((usize, usize), E)>, cycles: &mut Vec<Vec<usize>>, path: Vec<usize>) {
    let start = match path.first() {
        Some(start) => *start,
        None => return,
    };

    for ((left, right), _) in edges {
        if &start == left || &start == right {
            let next = if &start == left { right } else { left };
            if !path.contains(next) {
                let mut sub = vec![*next];
                sub.append(&mut path.clone());
                dfs_cycle(edges, cycles, sub);
            } else if path.len() > 2 && next == path.last().unwrap() {
                let p = rotate_to_smallest(path.clone());
                let inv = invert(p.clone());
                if !cycles.contains(&p) && !cycles.contains(&inv) {
                    cycles.push(p)
                }
            }
        }
    }
}

/// Rotate [`Vec`] around its smallest value.
fn rotate_to_smallest(path: Vec<usize>) -> Vec<usize> {
    let (mut min, mut i) = (usize::MAX, 0);

    for (j, node) in path.iter().enumerate() {
        if node < &min {
            i = j;
            min = *node;
        }
    }

    return path[i..]
        .to_vec()
        .into_iter()
        .chain(path[..i].to_vec().into_iter())
        .collect();
}

/// Reverse and rotate [`Vec`] around its smallest value.
fn invert(path: Vec<usize>) -> Vec<usize> {
    let mut path = path;
    path.reverse();
    rotate_to_smallest(path)
}
