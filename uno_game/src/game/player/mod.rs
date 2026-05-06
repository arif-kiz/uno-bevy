mod components;
mod resources;
mod events;
mod systems;

use bevy::prelude::*;
use crate::game::player::systems::*;

// May come in handy
// pub use components::{Player, PlayerCardVisual}; 
pub use resources::PlayerTurn;

pub const NUMBER_OF_PLAYERS: usize = 2;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerTurn>()
           .add_systems(Startup, setup_player)
           .add_systems(Update, (temp_handle_player, place_card_from_hand, draw_cards))
           .add_observer(player_refresh_card_visuals);
    }
}