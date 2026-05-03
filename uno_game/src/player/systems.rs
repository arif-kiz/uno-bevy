use crate::{
    deck::{
        CARD_HEIGHT, CARD_WIDTH, 
        Deck, 
        Discarded,
        RefreshDiscardedVisualsEvent,
    }, player::{
        PlayerTurn, components::*, events::*
    }
};
use bevy::{
    prelude::*,
    ecs::relationship::Relationship,  
    window::PrimaryWindow,
};
use card_shuffling::card::Action;

pub const NUMBER_OF_PLAYERS: usize = 2;

pub fn setup_player(
    mut commands: Commands,
    mut deck: ResMut<Deck>,
) {
    for id in 0..NUMBER_OF_PLAYERS {
        let mut player = Player::new(id);
        for _ in 0..7 {
            if let Some(card) = deck.draw() {
                player.add_card(card);
            }
        }

        let y = (id as f32 - 0.5) * 300.0;
        commands.spawn((
            player,
            Transform::from_translation(Vec3::new(0.0, y, 0.0)),
            Visibility::default(),
        ));
    }
    commands.trigger(RefreshPlayerVisualsEvent::all());
}

pub fn temp_handle_player(
    mut commands: Commands,
    mut deck: ResMut<Deck>,
    mut player_turn: ResMut<PlayerTurn>,
    input: Res<ButtonInput<MouseButton>>, 
    mut player_query: Query<&mut Player>,
) {
    if input.just_pressed(MouseButton::Right) {
        for mut player in player_query.iter_mut() {
            if player.get_id() == player_turn.get_current_player() {
                if let Some(card) = deck.draw() {
                    player.add_card(card);
                    commands.trigger(RefreshPlayerVisualsEvent::new(player.get_id()));
                    player_turn.change_player((player.get_id() + 1) % NUMBER_OF_PLAYERS);
                    break;
                }
            }
        }
    }
}

pub fn player_refresh_card_visuals(
    trigger: On<RefreshPlayerVisualsEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(Entity, &Player)>,
) {
    match trigger.event().get_event() {
        PlayerVisualsEvent::RefreshAll => {
            for (player_entity, player) in player_query.iter() {
                refresh_player(&mut commands, player_entity, player, &asset_server);
            }
        }

        PlayerVisualsEvent::Refresh(id) => {
            for (player_entity, player) in player_query.iter() {
                if &player.get_id() == id {
                    refresh_player(&mut commands, player_entity, player, &asset_server);
                    break;
                }
            }
        }
    }
}

fn refresh_player(
    commands: &mut Commands,
    player_entity: Entity, 
    player: &Player, 
    asset_server: &Res<AssetServer>,
) {
    let mut player_entity = commands.entity(player_entity);
    player_entity.despawn_children();

    player_entity.with_children(|parent| {
        for (index, card) in player.hand_cards().iter().enumerate() {
            let card_name = format!("{}_{}.png", card.get_action(), card.get_color());
            let x = position_of_card_in_player(index as f32, player.hand_cards().len() as f32);

            parent.spawn((
                PlayerCardVisual::new(index),
                Sprite {
                    image: asset_server.load(card_name),
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(x, 0.0, index as f32))
            ));
        }
    });
}

fn position_of_card_in_player(index: f32, cards_len: f32) -> f32 {
    if cards_len <= 1.0 {
        return 0.0;
    }

    let ideal_spacing = CARD_WIDTH * 0.5;
    let min_spacing = CARD_WIDTH * 0.2;
    let max_hand_width = 800.0;

    let required_spacing = (max_hand_width - CARD_WIDTH) / (cards_len - 1.0);

    let actual_spacing = ideal_spacing.min(required_spacing).max(min_spacing);

    let start_x = -((cards_len - 1.0) * actual_spacing) / 2.0;

    start_x + (index * actual_spacing)
}

pub fn place_card_from_hand(
    mut commands: Commands,
    click_input: Res<ButtonInput<MouseButton>>,
    mut discarded: ResMut<Discarded>,
    mut player_turn: ResMut<PlayerTurn>,
    camera_q: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<PlayerCardVisual>)>,
    card_query: Query<(&GlobalTransform, &PlayerCardVisual, &ChildOf)>,
    mut player_query: Query<&mut Player>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    if !click_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window_q.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };

    let world_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();

    let mut clicked_card = None;

    for (global_transform, card_index, parent) in card_query.iter() {
        let center = global_transform.translation().xy();
        let half_size = Vec2::new(CARD_WIDTH / 2.0, CARD_HEIGHT / 2.0);
        let card_rect = Rect::from_center_half_size(center, half_size);
        
        if card_rect.contains(world_pos) {
            if let Some((index, _)) = clicked_card {
                if index > card_index.get_index() {
                    break;
                }
            }
            clicked_card = Some((card_index.get_index(), parent.get()));
        }
    }
    
    let Some((index, player_entity)) = clicked_card else { return; };
    let Ok(mut player) = player_query.get_mut(player_entity) else { return; };

    if player.get_id() != player_turn.get_current_player() {
        println!("Nor current player.");
        return;
    }

    let Ok(placed_card) = player.drop_card(index, &mut discarded) else {
                        println!("You can't place this card"); return; };
    if placed_card.get_action().power() == 1 {
        player_turn.change_player((player.get_id() + 1) % NUMBER_OF_PLAYERS);
    }

    commands.trigger(RefreshDiscardedVisualsEvent);
    commands.trigger(RefreshPlayerVisualsEvent::new(player.get_id()));
}