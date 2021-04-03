//! # Polycentrics
//! Game logic of Polycentrics, including the board and virtual players

mod arrow;
mod board;
mod curve;
mod graph;

pub mod prelude {
    pub use crate::{
        arrow::Arrow,
        board::{Board, Player},
        curve::{Curve, Turn},
    };
}

#[cfg(test)]
mod tests;
