use std::fmt;
use crate::tile_state::TileState;

#[derive(Debug)]
pub enum Turn {
    Cross,
    Circle,
}

impl Turn {
    pub fn shift_turn(&mut self) {
        if let Self::Cross = *self {
            *self = Self::Circle;
        } else {
            *self = Self::Cross;
        }
    }

    pub const fn draw_for_turn(&self) -> TileState {
        if let Self::Cross = *self {
            TileState::Cross
        } else {
            TileState::Circle
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let choice = match *self {
            Self::Circle => 'O',
            Self::Cross => 'X',
        };
        write!(f, "{}", choice)
    }
}
