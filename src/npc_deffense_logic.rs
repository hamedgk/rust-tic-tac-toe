
use crate::GameBoard;
#[allow(non_upper_case_globals)]
pub fn rows<const i: usize>(game_board: &GameBoard) -> Option<(usize, usize)> {
    let opponent_choice = game_board.turn_info.human;
    let buf = &game_board.board_buffer;
    if buf[0][i] == opponent_choice && buf[2][i] == opponent_choice && buf[1][i].is_unchecked() {
        return Some((1, i));
    }
    if buf[1][i] == opponent_choice && buf[2][i] == opponent_choice && buf[0][i].is_unchecked() {
        return Some((0, i));
    }
    if buf[0][i] == opponent_choice && buf[1][i] == opponent_choice && buf[2][i].is_unchecked() {
        return Some((2, i));
    }
    None
}

#[allow(non_upper_case_globals)]
pub fn columns<const i: usize>(game_board: &GameBoard) -> Option<(usize, usize)> {
    let opponent_choice = game_board.turn_info.human;
    let board_buffer = &game_board.board_buffer;
    if board_buffer[i][0] == opponent_choice
        && board_buffer[i][2] == opponent_choice
        && board_buffer[i][1].is_unchecked()
    {
        return Some((i, 1));
    }
    if board_buffer[i][1] == opponent_choice
        && board_buffer[i][2] == opponent_choice
        && board_buffer[i][0].is_unchecked()
    {
        return Some((i, 0));
    }
    if board_buffer[i][0] == opponent_choice
        && board_buffer[i][1] == opponent_choice
        && board_buffer[i][2].is_unchecked()
    {
        return Some((i, 2));
    }
    None
}

pub fn primary_diagonal(game_board: &GameBoard) -> Option<(usize, usize)> {
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

pub fn secondary_diagonal(game_board: &GameBoard) -> Option<(usize, usize)> {
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
