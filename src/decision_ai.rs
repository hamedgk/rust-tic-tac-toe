use crate::{game_board::GameBoard, tile_state::TileState, turn::Turn};

fn npc_deffense(game_board: &GameBoard) -> Option<(usize, usize)> {
    let opponent_choice = match game_board.turn {
        Turn::Cross => TileState::Circle,
        Turn::Circle => TileState::Cross,
    };

    let tiles = &game_board.board_buffer;

    for (i, tiles) in tiles.iter().enumerate() {
        if tiles[0] == opponent_choice && tiles[2] == opponent_choice {
            return Some((i, 1));
        }
        if tiles[1] == opponent_choice && tiles[2] == opponent_choice {
            return Some((i, 0));
        }
        if tiles[0] == opponent_choice && tiles[1] == opponent_choice {
            return Some((i, 2));
        }
    }

    for i in 0..3 {
        if tiles[0][i] == opponent_choice && tiles[2][i] == opponent_choice {
            return Some((1, i));
        }
        if tiles[1][i] == opponent_choice && tiles[2][i] == opponent_choice {
            return Some((0, i));
        }
        if tiles[0][i] == opponent_choice && tiles[1][i] == opponent_choice {
            return Some((2, i));
        }
    }
    if tiles[0][0] == opponent_choice && tiles[2][2] == opponent_choice {
        return Some((1, 1));
    }
    if tiles[0][2] == opponent_choice && tiles[2][0] == opponent_choice {
        return Some((1, 1));
    }
    None
}

fn npc_offense(game_board: &GameBoard) -> Option<(usize, usize)> {
    None
}

pub(crate) fn npc_move(game_board: &GameBoard){
    let result = npc_deffense(game_board).unwrap_or_else(||{
        npc_offense(game_board).expect("tie maybe")
    });
}
