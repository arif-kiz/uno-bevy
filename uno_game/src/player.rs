use crate::{
    card::{UnoAction, UnoColor},
    deck::{CARD_HEIGHT, CARD_WIDTH, Deck, Discarded, RefreshDiscardedVisualsEvent},
};
use bevy::{ecs::relationship::Relationship, prelude::*, window::PrimaryWindow};
use card_shuffling::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
           .add_systems(Update, (temp_handle_player, place_card_from_hand));
    }
}

#[derive(Component, Clone)]
pub struct Player {
    cards: Vec<Card<UnoAction, UnoColor>>,
}

impl Player {
    pub fn hand_cards(&self) -> &Vec<Card<UnoAction, UnoColor>> {
        &self.cards
    }

    pub fn add_card(&mut self, card: Card<UnoAction, UnoColor>) {
        self.cards.push(card)
    }

    pub fn drop_card(&mut self, index: usize, mut discarded: ResMut<Discarded>) -> Result<(), String> {
        if !discarded.can_put(*self.cards.get(index).unwrap()) {
            return Err("You can't place this card".to_string())
        }

        let card = self.cards.remove(index);
        discarded.place_card(card)
    }
}

#[derive(Component)]
pub struct PlayerCardVisual(usize);

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut deck: ResMut<Deck>,
) {
    let mut player = Player { cards: Vec::new() };
    for _ in 0..7 {
        if let Some(card) = deck.draw() {
            player.add_card(card);
        }
    }
    let player_entity = commands.spawn((
        player.clone(),
        Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
        Visibility::default(),
    )).id();

    player_refresh_card_visuals(commands, asset_server, player_entity, &player);
}

pub fn temp_handle_player(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut deck: ResMut<Deck>,
    input: Res<ButtonInput<MouseButton>>, 
    mut player_query: Query<(Entity, &mut Player)>,
) {
    let (player_entity, mut player) = player_query.single_mut().unwrap();
    if input.just_pressed(MouseButton::Right) {
        if let Some(card) = deck.draw() {
            player.add_card(card);
            player_refresh_card_visuals(commands, asset_server, player_entity, &player);
        }
    }
}

fn player_refresh_card_visuals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_entity: Entity,
    player: &Player,
) {
    let mut player_entity = commands.entity(player_entity);
    player_entity.despawn_children();

    player_entity.with_children(|parent| {
        for (index, card) in player.hand_cards().iter().enumerate() {
            let card_name = format!("{}_{}.png", card.get_action(), card.get_color());
            let x = position_of_card_in_player(index as f32, player.hand_cards().len() as f32);

            parent.spawn((
                PlayerCardVisual(index),
                Sprite {
                    image: asset_server.load(card_name),
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(x, 0.0, 0.0))
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

fn place_card_from_hand(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    click_input: Res<ButtonInput<MouseButton>>,
    discarded: ResMut<Discarded>,
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
            clicked_card = Some((card_index.0, parent.get()));
        }
    }
    
    let Some((index, player_entity)) = clicked_card else { return; };
    let Ok(mut player) = player_query.get_mut(player_entity) else { return; };

    if player.drop_card(index, discarded).is_err() {
        println!("You can't place this card");
        return; 
        // TODO: better error handling.
    }

    commands.trigger(RefreshDiscardedVisualsEvent);
    player_refresh_card_visuals(commands, asset_server, player_entity, &player);
}