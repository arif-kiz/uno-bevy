use crate::card::{UnoAction, UnoColor};
use bevy::prelude::*;
use card_shuffling::prelude::*;
use rand::rngs::StdRng;

// ratio of image size: 2:3
pub const CARD_WIDTH: f32 = 80.0; // 2x
pub const CARD_HEIGHT: f32 = 120.0; // 3x
pub const DECK_POSITION: Vec3 = Vec3::new(90.0, 0.0, 0.0);

// --- PLUGINS ---

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

// --- COMPONENTS ---

#[derive(Component)]
pub struct DeckVisual;

#[derive(Component)]
pub struct DiscardedVisual;

// --- RESOURCES ---

#[derive(Resource)]
pub struct Deck {
    deck_cards: Cards<UnoAction, UnoColor, StdRng>
}

impl Deck {
    pub fn draw(&mut self) -> Option<Card<UnoAction, UnoColor>> {
        self.deck_cards.cards.pop()
    }
}

impl Default for Deck {
    fn default() -> Self {
        let rng: StdRng = rand::make_rng();
        let mut deck_cards: Cards<UnoAction, UnoColor, _> = Cards::from_file("uno.txt", Some(rng));
        deck_cards.randomize();
        Self { 
            deck_cards,
        }
    }
}

#[derive(Resource, Default)]
pub struct Discarded {
    card: Card<UnoAction, UnoColor>
}

impl Discarded {
    pub fn get_color(&self) -> UnoColor {
        self.card.get_color()
    }

    pub fn get_action(&self) -> UnoAction {
        self.card.get_action()
    }

    pub fn can_put(&self, card: Card<UnoAction, UnoColor>) -> bool {
        let card_action = card.get_action();
        let card_color = card.get_color();

        if card_color == UnoColor::Wild || card_color == self.card.get_color() {
            return true
        }
        if card_action == self.get_action() {
            return true
        }

        false
    }

    pub fn place_card(&mut self, card: Card<UnoAction, UnoColor>) -> Result<(), String> {
        if self.can_put(card) {
            self.card = card;
            Ok(())
        } else {
            Err(format!("You can't place this card"))
        }
    }
}

#[derive(Event)]
pub struct RefreshDiscardedVisualsEvent;

// --- SYSTEMS ---

fn setup_deck(mut commands: Commands, asset_server: Res<AssetServer>, discarded: Res<Discarded>) {
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

fn temp_handle_deck(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut deck: ResMut<Deck>, 
    mut discarded: ResMut<Discarded>, 
) {
    if input.just_pressed(KeyCode::Space) {
        if let Some(card) = deck.draw() {
            discarded.card = card; // Temporary for now
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