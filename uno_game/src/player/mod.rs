mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use crate::player::systems::{
    place_card_from_hand, 
    setup_player, 
    temp_handle_player,
};

// May come in handy
// pub use components::{Player, PlayerCardVisual}; 

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
           .add_systems(Update, (temp_handle_player, place_card_from_hand));
    }
}