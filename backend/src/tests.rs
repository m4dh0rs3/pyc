use crate::{
    angle::Angle,
    curve::{Curve, Turn},
    graph::Graph,
};
use math::Vec2D;

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

#[test]
fn contains_angle() {
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), 0.0.into(), 3.0.into(), Turn::Positive).contains(2.0.into()),
        true
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), (-2.0).into(), 3.0.into(), Turn::Positive)
            .contains(2.0.into()),
        true
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), (-2.0).into(), 1.0.into(), Turn::Positive)
            .contains(2.0.into()),
        false
    );
    assert_eq!(
        Curve::new(
            Vec2D::new(0, 0),
            (-3.0).into(),
            (-2.0).into(),
            Turn::Positive
        )
        .contains(1.0.into()),
        false
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), 2.0.into(), 3.0.into(), Turn::Positive).contains(1.0.into()),
        false
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), (-2.0).into(), 1.0.into(), Turn::Positive)
            .contains(1.0.into()),
        true
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), (-2.0).into(), 1.0.into(), Turn::Positive)
            .contains((-2.0).into()),
        true
    );

    assert_eq!(
        Curve::new(Vec2D::new(0, 0), 0.0.into(), 3.0.into(), Turn::Negative).contains(2.0.into()),
        false
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), (-2.0).into(), 3.0.into(), Turn::Negative)
            .contains(2.0.into()),
        false
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), (-2.0).into(), 1.0.into(), Turn::Negative)
            .contains(2.0.into()),
        true
    );
    assert_eq!(
        Curve::new(
            Vec2D::new(0, 0),
            (-3.0).into(),
            (-2.0).into(),
            Turn::Negative
        )
        .contains(1.0.into()),
        true
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), 2.0.into(), 3.0.into(), Turn::Negative).contains(1.0.into()),
        true
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), (-2.0).into(), 1.0.into(), Turn::Negative)
            .contains(1.0.into()),
        true
    );
    assert_eq!(
        Curve::new(Vec2D::new(0, 0), (-2.0).into(), 1.0.into(), Turn::Negative)
            .contains((-2.0).into()),
        true
    );
}
