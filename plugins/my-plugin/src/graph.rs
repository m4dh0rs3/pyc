use crate::edges::*;
use crate::path::Path;

#[derive(Debug, Clone)]
pub struct Graph<V>(Vec<(V, Vec<usize>)>);

impl<V> Graph<V> {
    pub fn new() -> Self {
        Graph(Vec::new())
    }

    pub fn push_node(&mut self, value: V) -> usize {
        self.0.push((value, Vec::new()));
        self.0.len() - 1
    }

    pub fn insert_edge(&mut self, i: usize, j: usize) {
        if !self.0[i].1.contains(&j) {
            self.0[i].1.push(j);
        }
        if !self.0[j].1.contains(&i) {
            self.0[j].1.push(i);
        }
    }

    pub fn remove_node(&mut self, key: &usize) {
        self.0.remove(*key);

        for (_, edges) in self.0.iter_mut() {
            edges.retain(|j| key != j);

            for j in edges {
                if &*j >= key {
                    *j -= 1;
                }
            }
        }
    }

    pub fn remove_edge(&mut self, i: &usize, j: &usize) {
        self.0[*i].1.retain(|n| n == j);
        self.0[*j].1.retain(|n| n == i);
    }

    pub fn edges(&self) -> Edges {
        let mut raw_edges = Vec::new();

        for (i, (_, edges)) in self.0.iter().enumerate() {
            for j in edges {
                raw_edges.push((i, *j));
            }
        }

        raw_edges
    }

    pub fn cycles(&self) -> Vec<Path> {
        cycles(&self.edges())
    }
}
