use crate::human::human_move;
use crate::npc::npc_move;
use crate::tile_state::TileState;
use crate::turn_info::TurnInfo;
use std::fmt;

pub(crate) struct GameBoard {
    pub(crate) board_buffer: [[TileState; 3]; 3],
    pub(crate) play_number: u8,
    pub(crate) turn_info: TurnInfo,
}

impl GameBoard {
    pub fn new(starter: Option<TileState>) -> Self {

        Self {
            board_buffer: [[TileState::Unchecked; 3]; 3],
            play_number: 0,
            turn_info: Self::init_turn_info(starter),
        }
    }

    fn init_turn_info(starter: Option<TileState>) -> TurnInfo{
        use TileState::*;
        match starter {
            Some(tile_state) => match tile_state {
                Cross => TurnInfo {
                    human: Cross,
                    npc: Circle,
                    current_turn: Cross,
                },
                Circle => TurnInfo {
                    human: Circle,
                    npc: Cross,
                    current_turn: Circle,
                },
                Unchecked => TurnInfo {
                    human: Cross,
                    npc: Circle,
                    current_turn: Cross,
                },
            },
            None => TurnInfo {
                human: Cross,
                npc: Circle,
                current_turn: Cross,
            },
        }
    }

    fn draw(&mut self, coords: (usize, usize)) -> Result<(usize, usize), &str> {
        let board = &mut self.board_buffer[coords.0][coords.1];
        if !matches!(board, TileState::Unchecked) {
            return Err("this tile is already filled");
        }
        *board = self.turn_info.current_turn;
        Ok((coords.0, coords.1))
    }

    fn game_step(&mut self) {
        self.play_number += 1;
        self.turn_info.change_turn();
    }

    pub fn play(&mut self) {
        loop {
            loop {
                let human_choice = human_move();
                if self.draw(human_choice).is_ok() {
                    break;
                }
                eprintln!("please choose empty tile");
            }
            self.game_step();
            println!("{}", self);

            let npc_choice = npc_move(self);
            match npc_choice {
                Some(coords) => {
                    self.draw(coords).expect("npc went wrong!");
                    self.game_step();
                    println!("{}", self);
                }
                None => break,
            }
        }
    }

    pub fn reset_game(&mut self, starter: Option<TileState>) {
        self.board_buffer.iter_mut().flatten().for_each(|tile| {
            *tile = TileState::Unchecked;
        });

        self.play_number = 0;
        self.turn_info = Self::init_turn_info(starter);
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.board_buffer;
        write!(
            f,
            r#"
    1   2   3
1 | {0} | {1} | {2} |    tried: {9}
2 | {3} | {4} | {5} |    turn:  {10}
3 | {6} | {7} | {8} |"#,
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
            self.turn_info.current_turn,
        )
    }
}
