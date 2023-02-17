use crate::tile_state::TileState;


pub struct TurnInfo{
    pub human: TileState,
    pub npc: TileState,
    pub current_turn: TileState,
}

impl TurnInfo {
    pub fn change_turn(&mut self){
        use TileState::*;
        self.current_turn = match self.current_turn {
            Cross => Circle,
            Circle => Cross,
            Unchecked => {unreachable!("Unchecked turn??")},
        }
    }
}

