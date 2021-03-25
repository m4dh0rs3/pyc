use crate::arrow::Rotation;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    horizontal: Horizontal,
    vertical: Vertical,
    radius: u8,
}

impl Tile {
    pub(crate) fn vertical(&self) -> &Vertical {
        &self.vertical
    }

    pub(crate) fn horizontal(&self) -> &Horizontal {
        &self.horizontal
    }

    pub(crate) fn radius(&self) -> &u8 {
        &self.radius
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Horizontal {
    Left,
    Right,
}

impl From<Horizontal> for Rotation {
    fn from(horizontal: Horizontal) -> Self {
        match horizontal {
            Horizontal::Left => Rotation::Left,
            Horizontal::Right => Rotation::Right,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Vertical {
    Up,
    Down,
}

impl From<Vertical> for Rotation {
    fn from(vertical: Vertical) -> Self {
        match vertical {
            Vertical::Up => Rotation::Up,
            Vertical::Down => Rotation::Down,
        }
    }
}

impl Tile {
    pub fn up_left_1() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical: Vertical::Up,
            radius: 1,
        }
    }

    pub fn up_left_2() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical: Vertical::Up,
            radius: 2,
        }
    }

    pub fn up_left_3() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical: Vertical::Up,
            radius: 3,
        }
    }

    pub fn up_right_3() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical: Vertical::Up,
            radius: 3,
        }
    }

    pub fn up_right_2() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical: Vertical::Up,
            radius: 2,
        }
    }

    pub fn up_right_1() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical: Vertical::Up,
            radius: 1,
        }
    }

    pub fn down_left_1() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical: Vertical::Down,
            radius: 1,
        }
    }

    pub fn down_left_2() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical: Vertical::Down,
            radius: 2,
        }
    }

    pub fn down_left_3() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical: Vertical::Down,
            radius: 3,
        }
    }

    pub fn down_right_3() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical: Vertical::Down,
            radius: 3,
        }
    }

    pub fn down_right_2() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical: Vertical::Down,
            radius: 2,
        }
    }

    pub fn down_right_1() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical: Vertical::Down,
            radius: 1,
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            match self.horizontal {
                Horizontal::Left => "Left",
                Horizontal::Right => "Right",
            },
            match self.vertical {
                Vertical::Up => "Up",
                Vertical::Down => "Down",
            },
            self.radius,
        )
    }
}
