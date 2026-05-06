use bevy::prelude::*;

use crate::game::deck::{CARD_HEIGHT, CARD_WIDTH, DECK_POSITION, components::{DeckVisual, DiscardedVisual}, events::RefreshDiscardedVisualsEvent, resources::{Deck, Discarded}};

pub fn setup_deck(mut commands: Commands, asset_server: Res<AssetServer>, discarded: Res<Discarded>) {
    let texture = asset_server.load("back.png");
    let card_name = format!("{}_{}.png", discarded.get_action(), discarded.get_color());
    let discarded_texture = asset_server.load(card_name);
    
    commands.spawn((
        DeckVisual,
        Sprite {
            image: texture,
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..Default::default()
        },
        Transform::from_translation(DECK_POSITION),
    ));

    commands.spawn((
        DiscardedVisual,
        Sprite {
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            image: discarded_texture,
            ..Default::default()
        },
        Transform::from_translation(DECK_POSITION.with_x(-60.0))
    ));
}

pub fn temp_handle_deck(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut deck: ResMut<Deck>, 
    mut discarded: ResMut<Discarded>, 
) {
    if input.just_pressed(KeyCode::Space) {
        if let Some(card) = deck.draw() {
            discarded.place_nomatterwhat(card);
            commands.trigger(RefreshDiscardedVisualsEvent);
        }
    }
}

pub fn refresh_discarded_visuals(
    _trigger: On<RefreshDiscardedVisualsEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    discarded: Res<Discarded>,
    discarded_query: Query<Entity, With<DiscardedVisual>>,
) {
    // Despawn the old discarded visuals so they don't pile up endlessly
    for entity in discarded_query.iter() {
        commands.entity(entity).despawn();
    }
    let card_name = format!("{}_{}.png", discarded.get_action(), discarded.get_color());
    let discarded_texture = asset_server.load(card_name);

    commands.spawn((
        DiscardedVisual,
        Sprite {
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            image: discarded_texture,
            ..Default::default()
        },
        Transform::from_translation(DECK_POSITION.with_x(-60.0))
    ));
}