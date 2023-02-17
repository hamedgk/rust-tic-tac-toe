mod game_board;
mod tile_state;
mod turn_info;
mod npc;
mod human;
mod win_check;
mod npc_deffense_logic;
mod possible_moves;

use game_board::GameBoard;
use tile_state::TileState;

fn main() {
    let mut game_board = GameBoard::new(Some(TileState::Cross));
    loop{
        game_board.play();
        game_board.reset_game(None);
    }

}
