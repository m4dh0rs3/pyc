use crate::{arrow::Arrow, curve::Curve, graph::Graph};
use math::Vec2D;

/// The subject of the game is the [`Board`].
/// It holds the current state and all data.
struct Board {
    active: Player,
    arrow: Arrow,
    graph: Graph<Vec2D<f64>, Curve>,
}

/// Enum of possible players.
/// [`Player::Gamma`] inspired by GAMMAGRAPHICS.
enum Player {
    Gamma,
    Delta,
}
