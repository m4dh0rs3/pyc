use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

use crate::{edges, path::Path};
use crate::{edges::*, Graph};

#[derive(Debug, Clone)]
pub struct UnsafeNodeGraph<V>(Vec<(V, Vec<usize>)>);

impl<V> UnsafeNodeGraph<V> {
    pub fn new() -> Self {
        UnsafeNodeGraph(Vec::new())
    }

    fn edges(&self) -> Edges {
        let mut raw_edges = Vec::new();

        for (i, (_, edges)) in self.0.iter().enumerate() {
            for j in edges {
                raw_edges.push((i, *j));
            }
        }

        raw_edges
    }
}

impl<V> Graph<usize, V, ()> for UnsafeNodeGraph<V> {
    fn get_node<'a>(&'a self, key: usize) -> Option<&'a V> {
        self.0.get(key).map(|(node, _)| node)
    }

    fn push_node(&mut self, value: V) -> usize {
        self.0.push((value, Vec::new()));
        self.0.len() - 1
    }

    fn get_edge<'a>(&'a self, _i: usize, _j: usize) -> Option<&'a ()> {
        None
    }

    fn insert_edge(&mut self, i: usize, j: usize, _edge: ()) -> Option<()> {
        if !self.0[i].1.contains(&j) {
            self.0[i].1.push(j);
        } else {
            return Some(());
        }

        if !self.0[j].1.contains(&i) {
            self.0[j].1.push(i);
        } else {
            return Some(());
        }

        None
    }

    fn remove_node(&mut self, key: usize) -> Option<V> {
        if key > self.0.len() {
            return None;
        }

        let node = self.0.remove(key).0;

        for (_, edges) in self.0.iter_mut() {
            edges.retain(|j| &key != j);

            for j in edges {
                if &*j >= &key {
                    *j -= 1;
                }
            }
        }

        return Some(node);
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Option<()> {
        self.0[i].1.retain(|n| n == &j);
        self.0[j].1.retain(|n| n == &i);

        None
    }

    fn cycles(&self) -> Vec<Path> {
        cycles(&self.edges())
    }
}

#[derive(Debug, Clone)]
pub struct SafeNodeGraph<V> {
    graph: BTreeMap<usize, (V, BTreeSet<usize>)>,
    count: usize,
}

impl<V> SafeNodeGraph<V> {
    pub fn new() -> Self {
        Self {
            graph: BTreeMap::new(),
            count: 0,
        }
    }

    fn edges(&self) -> Edges {
        let mut raw_edges = Vec::new();

        for (i, (_, edge)) in self.graph.iter() {
            for j in edge {
                raw_edges.push((*i, *j));
            }
        }

        raw_edges
    }
}

impl<V> Graph<usize, V, ()> for SafeNodeGraph<V>
where
    V: Debug + Copy,
{
    fn get_node<'a>(&'a self, key: usize) -> Option<&'a V> {
        self.graph.get(&key).map(|(node, _)| node)
    }

    fn push_node(&mut self, value: V) -> usize {
        self.count += 1;

        match self.graph.insert(self.count, (value, BTreeSet::new())) {
            Some(_) => panic!("Expected no entry in `Graph`"),
            None => {}
        }

        self.count
    }

    fn get_edge<'a>(&'a self, _i: usize, _j: usize) -> Option<&'a ()> {
        None
    }

    fn insert_edge(&mut self, i: usize, j: usize, _edge: ()) -> Option<()> {
        self.graph.entry(i).and_modify(|(_, edge)| {
            edge.insert(j);
        });
        self.graph.entry(j).and_modify(|(_, edge)| {
            edge.insert(i);
        });

        None
    }

    fn remove_node(&mut self, key: usize) -> Option<V> {
        let node = self.graph.remove_entry(&key).map(|(_, (node, _))| node);

        for (_, (_, edge)) in &mut self.graph {
            edge.remove(&key);
        }

        node
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Option<()> {
        self.graph.entry(i).and_modify(|(_, edge)| {
            edge.remove(&j);
        });
        self.graph.entry(j).and_modify(|(_, edge)| {
            edge.remove(&i);
        });

        None
    }

    fn cycles(&self) -> Vec<Path> {
        cycles(&self.edges())
    }
}

#[derive(Debug, Clone)]
pub struct EdgeGraph<N, E> {
    nodes: BTreeMap<usize, N>,
    edges: BTreeMap<(usize, usize), E>,
    count: usize,
}

impl<N, E> EdgeGraph<N, E>
where
    E: Clone,
{
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
            count: 0,
        }
    }

    fn edge_list(&self) -> Edges {
        self.edges.clone().into_iter().map(|(ij, _)| ij).collect()
    }

    pub fn edges(&mut self) -> &BTreeMap<(usize, usize), E> {
        &self.edges
    }

    pub fn nodes(&mut self) -> &BTreeMap<usize, N> {
        &self.nodes
    }

    pub fn last_node(&self) -> Option<&N> {
        self.nodes.get(&self.count)
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<N, E> Graph<usize, N, E> for EdgeGraph<N, E>
where
    E: Clone,
{
    fn get_node<'a>(&'a self, key: usize) -> Option<&'a N> {
        self.nodes.get(&key)
    }

    fn push_node(&mut self, node: N) -> usize {
        self.count += 1;
        self.nodes.insert(self.count, node);
        self.count
    }

    fn remove_node(&mut self, key: usize) -> Option<N> {
        self.nodes.remove(&key)
    }

    fn get_edge<'a>(&'a self, i: usize, j: usize) -> Option<&'a E> {
        self.edges.get(&(i, j))
    }

    fn insert_edge(&mut self, i: usize, j: usize, edge: E) -> Option<E> {
        self.edges.insert((i, j), edge)
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Option<E> {
        self.edges.remove(&(i, j))
    }

    fn cycles(&self) -> Vec<Path> {
        cycles(&self.edge_list())
    }
}

#[derive(Debug, Clone)]
pub struct EdgesGraph<N, E> {
    nodes: Vec<N>,
    edges: Vec<((usize, usize), E)>,
}

impl<N, E> EdgesGraph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl<N, E> Graph<usize, N, E> for EdgesGraph<N, E>
where
    E: Clone,
{
    fn get_node<'a>(&'a self, key: usize) -> Option<&'a N> {
        self.nodes.get(key)
    }

    fn push_node(&mut self, node: N) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn remove_node(&mut self, key: usize) -> Option<N> {
        if key >= self.nodes.len() {
            return None;
        }

        let node = self.nodes.remove(key);

        self.edges.retain(|((i, j), _)| i != &key || j != &key);

        for ((i, j), _) in self.edges.iter_mut() {
            if &*j >= &key {
                *j -= 1;
            }

            if &*i >= &key {
                *i -= 1;
            }
        }

        return Some(node);
    }

    fn get_edge<'a>(&'a self, i: usize, j: usize) -> Option<&'a E> {
        for (n, ((k, l), _)) in self.edges.iter().enumerate() {
            if &i == k && &j == l {
                return self.edges.get(n).map(|(_, edge)| edge);
            }
        }

        None
    }

    fn insert_edge(&mut self, i: usize, j: usize, edge: E) -> Option<E> {
        self.edges.push(((i, j), edge));

        None
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Option<E> {
        self.edges.retain(|((k, l), _)| !(&i == k && &j == l));

        None
    }

    fn cycles(&self) -> Vec<Vec<usize>> {
        cycles(&self.edges.iter().map(|(ij, _)| ij).cloned().collect())
    }
}
