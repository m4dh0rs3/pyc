//! # Polycentrics
//! Game logic of Polycentrics, including the board and virtual players.

mod board;
mod math;

pub mod prelude {
    pub use crate::board::Board;
}

// model is visually validated by an frontend, so no tests
