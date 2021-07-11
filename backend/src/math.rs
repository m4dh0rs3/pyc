mod angle;
mod utils;
mod vec_2d;

pub(crate) mod prelude {
    pub use super::{
        angle::Angle,
        utils::{bezier, lerp, remap},
        vec_2d::Vec2D,
    };
}
