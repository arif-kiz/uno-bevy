mod components;
mod resources;
mod events;
mod systems;

use bevy::prelude::*;
use crate::game::deck::systems::{
    refresh_discarded_visuals, 
    setup_deck, 
    temp_handle_deck
};

pub use resources::{Deck, Discarded};
pub use events::RefreshDiscardedVisualsEvent;

// Image size( width:height ) ratio = 2:3
pub const CARD_WIDTH: f32 = 80.0; // 2x
pub const CARD_HEIGHT: f32 = 120.0; // 3x
pub const DECK_POSITION: Vec3 = Vec3::new(90.0, 0.0, 0.0);

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Deck>()
           .init_resource::<Discarded>()
           .add_systems(Startup, setup_deck)
           .add_systems(Update, temp_handle_deck)
           .add_observer(refresh_discarded_visuals);
    }
}