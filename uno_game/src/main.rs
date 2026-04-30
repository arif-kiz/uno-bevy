mod deck;
mod card;
mod player;

use bevy::prelude::*;

use crate::deck::DeckPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DeckPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}