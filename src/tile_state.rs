use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileState {
    Cross,
    Circle,
    Empty,
}

#[allow(dead_code)]
impl TileState {
    pub fn is_cross(&self) -> bool{
        matches!(*self, TileState::Cross)
    }

    pub fn is_circle(&self) -> bool{
        matches!(*self, TileState::Circle)
    }

    pub fn is_unchecked(&self) -> bool{
        matches!(*self, TileState::Empty)
    }
}

impl fmt::Display for TileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let choice = match *self {
            Self::Circle => 'O',
            Self::Cross => 'X',
            Self::Empty => '_',
        };
        write!(f, "{}", choice)
    }
}
