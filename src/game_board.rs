use std::fmt;
use crate::turn::Turn;
use crate::tile_state::TileState;

pub(crate) struct GameBoard {
    pub(crate) board_buffer: [[TileState; 3]; 3],
    pub(crate) play_number: u8,
    pub(crate) turn: Turn,
}

impl GameBoard {
    pub fn new(starter: Option<Turn>) -> Self {
        Self {
            board_buffer: [[TileState::Unchecked; 3]; 3],
            play_number: 0,
            turn: starter.unwrap_or(Turn::Cross),
        }
    }

    fn draw(&mut self, coords: (usize, usize)) -> Result<(usize, usize), &str> {
        let board = &mut self.board_buffer[coords.0][coords.1];
        if !matches!(board, TileState::Unchecked){
            return Err("this tile is already filled");
        }
        *board = self.turn.draw_for_turn();
        self.turn.shift_turn();
        self.play_number += 1;

        Ok((coords.0, coords.1))
    }

    pub fn reset_game(&mut self, starter: Option<Turn>){
        self.board_buffer.iter_mut().flatten().for_each(|tile|{
            *tile = TileState::Unchecked;
        });

        self.play_number = 0;
        self.turn = starter.unwrap_or(Turn::Cross);
    }
}

impl fmt::Display for GameBoard{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.board_buffer;
        write!(f, 
r#"| {0} | {1} | {2} |    tried: {9}
| {3} | {4} | {5} |    turn:  {10}
| {6} | {7} | {8} |"#, 
            b[0][0],
            b[0][1],
            b[0][2],
            b[1][0],
            b[1][1],
            b[1][2],
            b[2][0],
            b[2][1],
            b[2][2],
            self.play_number,
            self.turn,
            )
    }
}
