#[cfg(test)]
mod tests;

mod edges;
mod graph;
mod path;

pub use crate::graph::EdgeGraph;

pub trait Graph<K, N, E>
where
    K: Copy,
{
    fn get_node<'a>(&'a self, key: K) -> Option<&'a N>;

    fn push_node(&mut self, node: N) -> K;
    fn remove_node(&mut self, key: K) -> Option<N>;

    fn get_edge<'a>(&'a self, i: K, j: K) -> Option<&'a E>;

    fn insert_edge(&mut self, i: K, j: K, edge: E) -> Option<E>;
    fn remove_edge(&mut self, i: K, j: K) -> Option<E>;

    fn cycles(&self) -> Vec<Vec<K>>;
}
