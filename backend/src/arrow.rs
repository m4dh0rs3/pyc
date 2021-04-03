use math::{angle::Angle, vec2d::Vec2D};

#[derive(Clone)]
pub struct Arrow {
    pub(crate) position: Vec2D<i8>,
    pub(crate) angle: Angle,
}

impl Arrow {
    pub fn new(position: Vec2D<i8>, angle: Angle) -> Self {
        Self { position, angle }
    }
}
