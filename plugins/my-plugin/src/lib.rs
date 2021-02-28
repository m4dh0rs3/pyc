use std::collections::{BTreeMap, BTreeSet};

struct UnGraph<V> {
    edges: BTreeMap<usize, BTreeSet<usize>>,
    nodes: BTreeMap<usize, V>,
}

impl<V> UnGraph<V> {
    fn new() -> Self {
        Self {
            edges: BTreeMap::new(),
            nodes: BTreeMap::new(),
        }
    }

    fn insert_edge(&mut self, i: usize, j: usize) {
        assert_ne!(i, j);

        self.edges.entry(i).or_insert(BTreeSet::new()).insert(j);
        self.edges.entry(j).or_insert(BTreeSet::new()).insert(i);
    }

    fn remove_edge(&mut self, i: usize, j: usize) {
        self.edges.get_mut(&i).map(|node| node.remove(&j));
        self.edges.get_mut(&j).map(|node| node.remove(&i));
    }

    fn push_node(&mut self, value: V) -> usize {
        let key = self.nodes.len();
        self.nodes.insert(key, value);
        key
    }

    fn remove_node(&mut self, key: usize) {
        self.nodes.remove(&key);
        self.edges.remove(&key);

        for (_, edge) in self.edges.iter_mut() {
            edge.remove(&key);
        }
    }

    fn edges(&self) -> usize {
        let mut edges = 0;

        for (_, edge) in &self.edges {
            edges += edge.len();
        }

        edges
    }

    fn cycles(&self) -> Vec<Vec<usize>> {
        let mut color = BTreeMap::new();
        let mut mark = BTreeMap::new();
        let mut par = BTreeMap::new();

        let cyclenumber: usize = 0;
        let edges = self.edges();

        self.dfs_cycle(1, 0, &mut color, &mut mark, &mut par, &mut cyclenumber);

        let cycles: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

        for i in 1..=edges {
            if let Some(&m) = mark.get(&i) {
                if m != 0 {
                    cycles.entry(key).or_insert(Vec::new()).push(i)
                }
            }
        }

        Vec::new()
    }

    fn dfs_cycle(
        &self,
        u: usize,
        p: usize,
        color: &mut BTreeMap<usize, usize>,
        mark: &mut BTreeMap<usize, usize>,
        par: &mut BTreeMap<usize, usize>,
        cyclenumber: &mut usize,
    ) {
    }
}

struct Graph(Vec<(usize, usize)>);

impl Graph {
    fn cycles(&self) -> Vec<Vec<usize>> {
        let mut cycles: Vec<Vec<usize>> = Vec::new();

        for edge in self.0 {
            self.dfs_cylce(&mut cycles, vec![edge.0]);
            self.dfs_cylce(&mut cycles, vec![edge.1]);
        }

        cycles
    }

    fn dfs_cycle(&self, cycles: &mut Vec<Vec<usize>>, path: Vec<usize>) {
        let start_node = *path.first().unwrap();
        let next_node = None;
        let sub = Vec::new();

        for (node1, node2) in self.0 {
            if start_node == node1 || start_node == node2 {
                next_node = Some(node2);
            } else {
                next_node = Some(node1)
            }

            if !visited(&next_node.unwrap(), &path) {
                sub = vec![next_node.unwrap()];
                sub.append(&mut path);

                self.dfs_cycle(cycles, sub);
            } else if path.len() > 2 && &next_node.unwrap() == path.last().unwrap() {
                let p = rotate_to_smallest(path);
                let inv = invert(p);x

                if is_new(p) && is_new(inv) {
                    cycles.append(&mut p)
                }
            }
        }
    }
}

fn visited(node: &usize, path: &Vec<usize>) -> bool {
    path.contains(&node)
}
