//! # Polycentrics
//! Game logic (WASM Server) of Polycentrics, including the board and virtual players.

/* // use `wee_alloc` as the global allocator when compiling to wasm
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT; */

// the precision type
type Float = f32;

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
