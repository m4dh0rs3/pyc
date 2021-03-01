use crate::graph::*;
use crate::path::*;

#[test]
fn path_rotation() {
    assert_eq!(rotate_to_smallest(vec![3, 2, 8, 0]), vec![0, 3, 2, 8]);
    assert_eq!(rotate_to_smallest(vec![3, 0, 8, 2]), vec![0, 8, 2, 3]);
    assert_eq!(rotate_to_smallest(vec![0, 3, 8, 2]), vec![0, 3, 8, 2]);
}

#[test]
fn cycles() {
    /*
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
    */

    let mut graph = Graph::new();

    for i in 0..=9 {
        graph.push_node(i);
    }

    graph.insert_edge(1, 2);
    graph.insert_edge(1, 3);
    graph.insert_edge(1, 4);
    graph.insert_edge(2, 3);
    graph.insert_edge(3, 4);
    graph.insert_edge(2, 6);
    graph.insert_edge(4, 6);
    graph.insert_edge(8, 7);
    graph.insert_edge(8, 9);
    graph.insert_edge(9, 7);

    println!("{:?}", graph.cycles());
}

#[test]
fn graph() {
    let mut graph = Graph::new();

    for i in 0..=9 {
        graph.push_node(i);
    }

    graph.insert_edge(1, 2);
    graph.insert_edge(1, 3);
    graph.insert_edge(1, 4);
    graph.insert_edge(2, 3);
    graph.insert_edge(3, 4);
    graph.insert_edge(2, 6);
    graph.insert_edge(4, 6);
    graph.insert_edge(8, 7);
    graph.insert_edge(8, 9);
    graph.insert_edge(9, 7);

    println!("{:?}", &graph);

    graph.remove_node(&2);

    println!("{:?}", &graph);
}
