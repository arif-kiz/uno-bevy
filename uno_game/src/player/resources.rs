use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerTurn {
    player_id: usize,
}

impl PlayerTurn {
    pub fn get_current_player(&self) -> usize {
        self.player_id
    }

    pub fn change_player(&mut self, player_id: usize) {
        self.player_id = player_id
    }
}