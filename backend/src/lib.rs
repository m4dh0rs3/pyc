//! # Polycentrics
//! Game logic of Polycentrics, including the board and virtual players.

mod board;
mod math;

pub mod prelude {
    pub use crate::{board::Board, math::prelude::*};
}
