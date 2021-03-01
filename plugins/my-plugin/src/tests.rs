use crate::graph::Graph;
use crate::path::PathExt;

#[test]
fn path_rotation() {
    assert_eq!(vec![3, 2, 8, 0].rotate_to_smallest(), vec![0, 3, 2, 8]);

    assert_eq!(vec![3, 0, 8, 2].rotate_to_smallest(), vec![0, 8, 2, 3]);

    assert_eq!(vec![0, 3, 8, 2].rotate_to_smallest(), vec![0, 3, 8, 2]);
}

#[test]
fn cycles() {
    let g = Graph::from_raw(vec![
        (1, 2),
        (1, 3),
        (1, 4),
        (2, 3),
        (3, 4),
        (2, 6),
        (4, 6),
        (8, 7),
        (8, 9),
        (9, 7),
    ]);

    println!("{:?}", g.cycles());
}
