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
            Empty => {unreachable!("Empty turn??")},
        }
    }

    pub fn init_turn_info(starter: Option<TileState>) -> Self{
        use TileState::*;
        match starter {
            Some(tile_state) => match tile_state {
                Cross => TurnInfo {
                    human: Cross,
                    npc: Circle,
                    current_turn: Cross,
                },
                Circle => TurnInfo {
                    human: Circle,
                    npc: Cross,
                    current_turn: Circle,
                },
                Empty => TurnInfo {
                    human: Cross,
                    npc: Circle,
                    current_turn: Cross,
                },
            },
            None => TurnInfo {
                human: Cross,
                npc: Circle,
                current_turn: Cross,
            },
        }
    }
}

