use crate::graph::Graph;

#[test]
fn graph_cycles() {
    let mut graph = Graph::new();

    for i in 0..=9 {
        graph.push_node(i);
    }

    graph.insert_edge(1, 2, ());
    graph.insert_edge(1, 3, ());
    graph.insert_edge(1, 4, ());
    graph.insert_edge(2, 3, ());
    graph.insert_edge(3, 4, ());
    graph.insert_edge(2, 6, ());
    graph.insert_edge(4, 6, ());
    graph.insert_edge(8, 7, ());
    graph.insert_edge(8, 9, ());
    graph.insert_edge(9, 7, ());

    // compare result of python script "print_cycles.py"
    assert_eq!(
        vec![
            vec![1, 3, 2],
            vec![1, 4, 3, 2],
            vec![1, 4, 6, 2],
            vec![1, 3, 4, 6, 2],
            vec![1, 4, 6, 2, 3],
            vec![1, 4, 3],
            vec![2, 6, 4, 3],
            vec![7, 8, 9]
        ],
        graph.cycles()
    );
}
