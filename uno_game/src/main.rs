mod deck;
mod card;
mod player;

use bevy::prelude::*;

use crate::deck::DeckPlugin;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DeckPlugin, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}