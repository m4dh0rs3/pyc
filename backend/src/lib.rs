//! # Polycentrics
//! Game logic of Polycentrics, including the board and virtual players.

mod game;
mod math;

pub mod prelude {
    pub use crate::{
        game::{
            board::{Board, Player},
            curve::Curve,
        },
        math::prelude::*,
    };
}
