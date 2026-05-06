mod deck;
mod card;
mod player;

use bevy::prelude::*;

use deck::DeckPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, DeckPlugin, PlayerPlugin))
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}