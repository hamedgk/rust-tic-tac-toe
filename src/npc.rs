use crate::game_board::GameBoard;
use crate::possible_moves::POSSIBLE_MOVES;

fn npc_deffense(game_board: &GameBoard) -> Option<(usize, usize)> {
    unsafe {
        for operation in POSSIBLE_MOVES.iter_mut() {
            match operation {
                Some(ops) => {
                    let result = (ops.0)(game_board);
                    match result {
                        Some(_) => {
                            //skip this operation next time
                            *operation = None;
                            return result;
                        }
                        None => {
                            continue;
                        }
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }
    None
}

fn npc_offense(game_board: &GameBoard) -> Option<(usize, usize)> {
    for (i, tiles) in game_board.board_buffer.iter().enumerate() {
        for (j, tile) in tiles.iter().enumerate() {
            if tile.is_unchecked() {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn npc_move(game_board: &GameBoard) -> Option<(usize, usize)> {
    let deffense_result = npc_deffense(game_board);
    match deffense_result {
        Some(_) => deffense_result,
        None => npc_offense(game_board),
    }
}
