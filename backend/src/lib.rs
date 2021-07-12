//! # Polycentrics
//! Game logic (WASM Server) of Polycentrics, including the board and virtual players.

mod game;
mod math;
// re-export for frontend use
pub mod prelude {
    pub use crate::{
        game::{
            board::{Arrow, Board, Player},
            curve::Curve,
        },
        math::prelude::*,
    };
}
