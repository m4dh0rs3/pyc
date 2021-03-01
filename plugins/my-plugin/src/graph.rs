use crate::path::PathExt;

pub struct Graph(Vec<(usize, usize)>);

impl Graph {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn from_raw(graph: Vec<(usize, usize)>) -> Self {
        Self(graph)
    }

    pub fn cycles(&self) -> Vec<Vec<usize>> {
        let mut cycles: Vec<Vec<usize>> = Vec::new();

        for edge in &self.0 {
            self.dfs_cycle(&mut cycles, vec![edge.0]);
            self.dfs_cycle(&mut cycles, vec![edge.1]);
        }

        cycles
    }

    fn dfs_cycle(&self, cycles: &mut Vec<Vec<usize>>, path: Vec<usize>) {
        let start = match path.first() {
            Some(start) => *start,
            None => return,
        };

        for (left, right) in &self.0 {
            if &start == left || &start == right {
                let next = if &start == left { right } else { left };

                if !path.contains(next) {
                    let mut sub = vec![*next];
                    sub.append(&mut path.clone());

                    self.dfs_cycle(cycles, sub);
                } else if path.len() > 2 && next == path.last().unwrap() {
                    let p = path.clone().rotate_to_smallest();
                    let inv = p.clone().invert();

                    if !cycles.contains(&p) && !cycles.contains(&inv) {
                        cycles.push(p)
                    }
                }
            }
        }
    }
}
