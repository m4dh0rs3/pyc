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

// TODO: Thursday:
//  - Add start and end control points from curves in poly to points on board
//  - Intersect previeous control points does not yet work?
