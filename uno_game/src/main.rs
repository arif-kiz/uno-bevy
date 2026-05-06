mod game;

use bevy::app::App;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .run();
}