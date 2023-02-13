mod game_board;
mod tile_state;
mod turn;
mod decision_ai;

use game_board::GameBoard;
use turn::Turn;

fn main() {
    let mut game_board = GameBoard::new(Some(Turn::Cross));
    println!("{}", game_board);

}
