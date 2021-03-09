pub(crate) type Path = Vec<usize>;

pub(crate) fn rotate_to_smallest(path: Path) -> Path {
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

pub(crate) fn invert(path: Path) -> Path {
    let mut path = path;
    path.reverse();
    rotate_to_smallest(path)
}
