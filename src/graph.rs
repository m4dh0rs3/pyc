use crate::curve::Curve;
use crate::point::Point;

struct Graph {
    curves: Vec<Curve>,
    nodes: Vec<Point>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn add_curve(&mut self, curve: Curve) {}
}


/* use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct UnGraph {
    adj: HashMap<usize, Vec<usize>>,
}

impl UnGraph {
    pub fn push_edge(&mut self, u: usize, v: usize) {
        self.adj
            .entry(u)
            .and_modify(|k| {k.push(v)})
            .or_insert(vec![v]);

        self.adj
            .entry(v)
            .and_modify(|k| {k.push(u)})
            .or_insert(vec![u]);
    }
} */