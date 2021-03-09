use crate::path::*;

pub(crate) type Edges = Vec<(usize, usize)>;

pub(crate) fn cycles(edges: &Edges) -> Vec<Path> {
    let mut cycles: Vec<Path> = Vec::new();

    for edge in edges {
        dfs_cycle(edges, &mut cycles, vec![edge.0]);
        dfs_cycle(edges, &mut cycles, vec![edge.1]);
    }

    cycles
}

fn dfs_cycle(edges: &Edges, cycles: &mut Vec<Path>, path: Path) {
    let start = match path.first() {
        Some(start) => *start,
        None => return,
    };

    for (left, right) in edges {
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
