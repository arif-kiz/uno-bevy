use crate::game::{
    card::UnoAction, deck::{
        CARD_HEIGHT, CARD_WIDTH, 
        Deck, 
        Discarded,
        RefreshDiscardedVisualsEvent,
    }, player::{
        NUMBER_OF_PLAYERS, PlayerTurn, components::*, events::*
    }
};
use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

pub fn setup_player(
    mut commands: Commands,
    mut deck: ResMut<Deck>,
    mut player_turn: ResMut<PlayerTurn>
) {
    for id in 0..NUMBER_OF_PLAYERS {
        let mut player = Player::new();
        for _ in 0..7 {
            if let Some(card) = deck.draw() {
                player.add_card(card);
            }
        }

        let y = (id as f32 - 0.5) * 300.0;
        let player_entity = commands.spawn((
            player,
            Transform::from_translation(Vec3::new(0.0, y, 0.0)),
            Visibility::default(),
        )).id();
        player_turn.set_player(player_entity, id);
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
        let current_player_id = player_turn.get_current_player();
        if let Ok(mut player) = player_query.get_mut(current_player_id) {
            if let Some(card) = deck.draw() {
                player.add_card(card);
                commands.trigger(RefreshPlayerVisualsEvent::new(current_player_id));
                player_turn.change_player();
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

        PlayerVisualsEvent::Refresh(player_entity) => {
            if let Ok((entity, player)) = player_query.get(*player_entity) {
                refresh_player(&mut commands, entity, player, &asset_server);
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
    card_query: Query<(&GlobalTransform, &PlayerCardVisual)>,
    mut player_query: Query<(&mut Player, Option<&Children>)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    if !click_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window_q.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };

    let world_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();

    let current_player_entity = player_turn.get_current_player();
    let Ok((mut player, children_opt)) = player_query.get_mut(current_player_entity) else { return; };

    let mut clicked_card = None;

    if let Some(children) = children_opt {
        for child in children.iter() {
            if let Ok((global_transform, card_index)) = card_query.get(child) {
                let center = global_transform.translation().xy();
                let half_size = Vec2::new(CARD_WIDTH / 2.0, CARD_HEIGHT / 2.0);
                let card_rect = Rect::from_center_half_size(center, half_size);
                
                if card_rect.contains(world_pos) {
                    if let Some((index, _)) = clicked_card {
                        if index > card_index.get_index() {
                            continue;
                        }
                    }
                    clicked_card = Some((card_index.get_index(), child));
                }
            }
        }
    }
    
    let Some((index, _)) = clicked_card else { return; };

    let Ok(placed_card) = player.drop_card(index, &mut discarded) else {
        println!("You can't place this card");
        return; 
    };

    player_turn.change_player();
    match placed_card.get_action() {
        UnoAction::Skip => player_turn.change_player(),
        UnoAction::Reverse => {player_turn.change_direction(); if NUMBER_OF_PLAYERS == 2 {player_turn.change_player();}},
        UnoAction::DrawTwo => add_draw_cards(&mut commands, player_turn.get_current_player(), 2),
        UnoAction::DrawFour => add_draw_cards(&mut commands, player_turn.get_current_player(), 4),
        _ => {},
    }

    commands.trigger(RefreshDiscardedVisualsEvent);
    commands.trigger(RefreshPlayerVisualsEvent::new(current_player_entity));
}

fn add_draw_cards(
    commands: &mut Commands,
    current_player: Entity,
    no_cards: usize,
) {
    let mut player_commands = commands.entity(current_player);
    player_commands.insert(DrawCards::new(no_cards));
}

pub fn draw_cards(
    mut commands: Commands,
    mut deck: ResMut<Deck>,
    mut player_query: Query<(Entity, &mut Player, &mut DrawCards)>,
) {
    for (player_entity, mut player, draw) in player_query.iter_mut() {
        for _ in 0..draw.no_cards() {
            if let Some(card) = deck.draw() {
                player.add_card(card);
            } else {
                break;
            }
        }
        
        commands.trigger(
            RefreshPlayerVisualsEvent::new(player_entity)
        );
        commands.entity(player_entity).remove::<DrawCards>();
    }
}