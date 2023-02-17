use crate::GameBoard;
use crate::{npc_deffense_logic, win_check};

type NPCDeffenseAction = fn(&GameBoard) -> Option<(usize, usize)>;
type WinCheckAction = fn(&GameBoard) -> bool;

pub static mut POSSIBLE_MOVES: [Option<(NPCDeffenseAction, WinCheckAction)>; 8] = [
    Some((npc_deffense_logic::rows::<0>, win_check::rows::<0>)),
    Some((npc_deffense_logic::rows::<1>, win_check::rows::<1>)),
    Some((npc_deffense_logic::rows::<2>, win_check::rows::<2>)),
    Some((npc_deffense_logic::columns::<0>, win_check::columns::<0>)),
    Some((npc_deffense_logic::columns::<1>, win_check::columns::<1>)),
    Some((npc_deffense_logic::columns::<2>, win_check::columns::<2>)),
    Some((
        npc_deffense_logic::primary_diagonal,
        win_check::primary_diagonal,
    )),
    Some((
        npc_deffense_logic::secondary_diagonal,
        win_check::secondary_diagonal,
    )),
];

pub fn reset_possible_moves() {
    unsafe {
        POSSIBLE_MOVES = [
            Some((npc_deffense_logic::rows::<0>, win_check::rows::<0>)),
            Some((npc_deffense_logic::rows::<1>, win_check::rows::<1>)),
            Some((npc_deffense_logic::rows::<2>, win_check::rows::<2>)),
            Some((npc_deffense_logic::columns::<0>, win_check::columns::<0>)),
            Some((npc_deffense_logic::columns::<1>, win_check::columns::<1>)),
            Some((npc_deffense_logic::columns::<2>, win_check::columns::<2>)),
            Some((
                npc_deffense_logic::primary_diagonal,
                win_check::primary_diagonal,
            )),
            Some((
                npc_deffense_logic::secondary_diagonal,
                win_check::secondary_diagonal,
            )),
        ];
    }
}
