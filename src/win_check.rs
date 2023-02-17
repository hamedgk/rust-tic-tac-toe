use crate::game_board::GameBoard;

#[allow(non_upper_case_globals)]
pub fn rows<const i: usize>(game_board: &GameBoard) -> bool{
    let buf = &game_board.board_buffer; 
    !buf[0][i].is_unchecked() && (buf[0][i] == buf[1][i]) && (buf[1][i]== buf[2][i])
}

#[allow(non_upper_case_globals)]
pub fn columns<const i: usize>(game_board: &GameBoard) -> bool{
    let buf = &game_board.board_buffer; 
    !buf[i][0].is_unchecked() && (buf[i][0] == buf[i][1]) && (buf[i][1]== buf[i][2])
}

pub fn primary_diagonal(game_board: &GameBoard) -> bool{
    let buf = &game_board.board_buffer; 
    !buf[0][0].is_unchecked() && (buf[0][0] == buf[1][1]) && (buf[1][1]== buf[2][2])
}

pub fn secondary_diagonal(game_board: &GameBoard) -> bool{
    let buf = &game_board.board_buffer; 
    !buf[2][0].is_unchecked() && (buf[2][0] == buf[1][1]) && (buf[1][1]== buf[0][2])
}
