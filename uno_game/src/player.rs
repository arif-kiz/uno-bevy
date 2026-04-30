use crate::card::{UnoAction, UnoColor};
use bevy::prelude::Component;
use card_shuffling::prelude::*;

#[derive(Component)]
pub struct Player {
    cards: Vec<Card<UnoAction, UnoColor>>,
}

impl Player {
    pub fn hand_cards(&self) -> &Vec<Card<UnoAction, UnoColor>> {
        &self.cards
    }
}
