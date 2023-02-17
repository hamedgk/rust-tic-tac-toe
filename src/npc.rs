use crate::game_board::GameBoard;

type MatrixOperations = [Option<fn(&GameBoard) -> Option<(usize, usize)>>; 8];
static mut NPC_MEMORY: MatrixOperations = [
    Some(first_row),
    Some(second_row),
    Some(third_row),
    Some(first_column),
    Some(second_column),
    Some(third_column),
    Some(primary_diagonal),
    Some(secondary_diagonal),
];


macro_rules! rows {
    ($i : literal, $fn_name: ident) => {
        #[allow(dead_code)]
        fn $fn_name(game_board: &GameBoard) -> Option<(usize, usize)> {
            let opponent_choice = game_board.turn_info.human;
            let buf = &game_board.board_buffer;
            if buf[0][$i] == opponent_choice
                && buf[2][$i] == opponent_choice
                && buf[1][$i].is_unchecked()
            {
                return Some((1, $i));
            }
            if buf[1][$i] == opponent_choice
                && buf[2][$i] == opponent_choice
                && buf[0][$i].is_unchecked()
            {
                return Some((0, $i));
            }
            if buf[0][$i] == opponent_choice
                && buf[1][$i] == opponent_choice
                && buf[2][$i].is_unchecked()
            {
                return Some((2, $i));
            }
            None
        }
    };
}

macro_rules! columns {
    ($i : literal, $fn_name: ident) => {
        #[allow(dead_code)]
        fn $fn_name(game_board: &GameBoard) -> Option<(usize, usize)> {
            let opponent_choice = game_board.turn_info.human;
            let board_buffer = &game_board.board_buffer;
            if board_buffer[$i][0] == opponent_choice
                && board_buffer[$i][2] == opponent_choice
                && board_buffer[$i][1].is_unchecked()
            {
                return Some(($i, 1));
            }
            if board_buffer[$i][1] == opponent_choice
                && board_buffer[$i][2] == opponent_choice
                && board_buffer[$i][0].is_unchecked()
            {
                return Some(($i, 0));
            }
            if board_buffer[$i][0] == opponent_choice
                && board_buffer[$i][1] == opponent_choice
                && board_buffer[$i][2].is_unchecked()
            {
                return Some(($i, 2));
            }
            None
        }
    };
}

rows!(0, first_row);
rows!(1, second_row);
rows!(2, third_row);

columns!(0, first_column);
columns!(1, second_column);
columns!(2, third_column);

#[allow(dead_code)]
fn primary_diagonal(game_board: &GameBoard) -> Option<(usize, usize)> {
    let opponent_choice = game_board.turn_info.human;
    let board_buffer = &game_board.board_buffer;
    if board_buffer[0][0] == opponent_choice
        && board_buffer[2][2] == opponent_choice
        && board_buffer[1][1].is_unchecked()
    {
        return Some((1, 1));
    }
    if board_buffer[0][0] == opponent_choice
        && board_buffer[1][1] == opponent_choice
        && board_buffer[2][2].is_unchecked()
    {
        return Some((2, 2));
    }
    if board_buffer[1][1] == opponent_choice
        && board_buffer[2][2] == opponent_choice
        && board_buffer[0][0].is_unchecked()
    {
        return Some((0, 0));
    }
    None
}

#[allow(dead_code)]
fn secondary_diagonal(game_board: &GameBoard) -> Option<(usize, usize)> {
    let opponent_choice = game_board.turn_info.human;
    let board_buffer = &game_board.board_buffer;
    if board_buffer[0][2] == opponent_choice
        && board_buffer[2][0] == opponent_choice
        && board_buffer[1][1].is_unchecked()
    {
        return Some((1, 1));
    }
    if board_buffer[0][2] == opponent_choice
        && board_buffer[1][1] == opponent_choice
        && board_buffer[2][0].is_unchecked()
    {
        return Some((2, 0));
    }
    if board_buffer[1][1] == opponent_choice
        && board_buffer[2][0] == opponent_choice
        && board_buffer[0][2].is_unchecked()
    {
        return Some((0, 2));
    }
    None
}

fn npc_deffense(game_board: &GameBoard) -> Option<(usize, usize)> {
    unsafe {
        for operation in NPC_MEMORY.iter_mut() {
            match operation {
                Some(op) => {
                    let result = op(game_board);
                    match result {
                        Some(_) => {
                            *operation = None;
                            return result
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

pub(crate) fn npc_move(game_board: &GameBoard) -> Option<(usize, usize)> {
    let deffense_result = npc_deffense(game_board);
    match deffense_result {
        Some(_) => deffense_result,
        None => npc_offense(game_board),
    }
}
