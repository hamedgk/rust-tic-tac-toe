mod game_board;
mod tile_state;
mod turn_info;
mod npc;
mod human;

use game_board::GameBoard;
use tile_state::TileState;

fn main() {
    let mut game_board = GameBoard::new(Some(TileState::Cross));
    game_board.play();
    println!("{}", game_board);

}
