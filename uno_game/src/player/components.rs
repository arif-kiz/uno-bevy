use bevy::prelude::*;
use card_shuffling::card::Card;
use crate::card::{UnoAction, UnoColor};
use crate::deck::Discarded;

#[derive(Component, Default)]
pub struct Player {
    id: usize,
    cards: Vec<Card<UnoAction, UnoColor>>,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self { id, cards: Vec::new() }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn hand_cards(&self) -> &Vec<Card<UnoAction, UnoColor>> {
        &self.cards
    }

    pub fn add_card(&mut self, card: Card<UnoAction, UnoColor>) {
        self.cards.push(card)
    }

    pub fn drop_card(&mut self, index: usize, discarded: &mut ResMut<Discarded>) -> Result<Card<UnoAction, UnoColor>, String> {
        if !discarded.can_put(*self.cards.get(index).unwrap()) {
            return Err("You can't place this card".to_string())
        }

        let card = self.cards.remove(index);
        discarded.place_card(card)
    }
}

#[derive(Component, Default)]
pub struct PlayerCardVisual(usize);

impl PlayerCardVisual {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn get_index(&self) -> usize{
        self.0
    }
}