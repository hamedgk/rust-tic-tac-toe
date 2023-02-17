use crate::human::human_move;
use crate::npc::npc_move;
use crate::tile_state::TileState;
use crate::turn_info::TurnInfo;
use crate::possible_moves::{POSSIBLE_MOVES, reset_possible_moves};
use std::fmt;


pub struct GameBoard {
    pub board_buffer: [[TileState; 3]; 3],
    pub play_number: u8,
    pub turn_info: TurnInfo,
}

impl GameBoard {
    pub fn new(starter: Option<TileState>) -> Self {
        Self {
            board_buffer: [[TileState::Empty; 3]; 3],
            play_number: 0,
            turn_info: TurnInfo::init_turn_info(starter),
        }
    }

    fn win_check(&self) -> Option<TileState> {
        unsafe{
            for possible_move in POSSIBLE_MOVES.iter(){
                match possible_move{
                Some(ops) => {
                    if (ops.1)(self){
                        return Some(self.turn_info.current_turn)
                    }
                },
                None => continue,
                }
            }
        }
        None
    }

    fn draw(&mut self, coords: (usize, usize)) -> Result<(usize, usize), &str> {
        let board = &mut self.board_buffer[coords.0][coords.1];
        if !matches!(board, TileState::Empty) {
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
                println!("enter choice e.g 11 : ");
                let human_choice = human_move();
                if self.draw(human_choice).is_ok() {
                    break;
                }
                eprintln!("please choose empty tile");
            }
            if let Some(turn) = self.win_check(){
                println!("winner: {}", turn);
                break;
            }
            self.game_step();
            println!("{}", self);

            let npc_choice = npc_move(self);
            match npc_choice {
                Some(coords) => {
                    self.draw(coords).expect("npc went wrong!");
                    if let Some(turn) = self.win_check(){
                        println!("winner: {}", turn);
                        break;
                    }
                    self.game_step();
                    println!("{}", self);
                }
                None => {
                    println!("tie");
                    break;
                },
            }
        }
    }

    #[allow(dead_code)]
    pub fn reset_game(&mut self, starter: Option<TileState>) {
        self.board_buffer.iter_mut().flatten().for_each(|tile| {
            *tile = TileState::Empty;
        });

        self.play_number = 0;
        self.turn_info = TurnInfo::init_turn_info(starter);
        reset_possible_moves();
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
