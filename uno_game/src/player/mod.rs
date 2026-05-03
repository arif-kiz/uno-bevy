mod components;
mod resources;
mod events;
mod systems;

use bevy::prelude::*;
use crate::player::systems::{
    place_card_from_hand, 
    setup_player, 
    temp_handle_player,
    player_refresh_card_visuals,
};

// May come in handy
// pub use components::{Player, PlayerCardVisual}; 
pub use resources::PlayerTurn;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerTurn>()
           .add_systems(Startup, setup_player)
           .add_systems(Update, (temp_handle_player, place_card_from_hand))
           .add_observer(player_refresh_card_visuals);
    }
}