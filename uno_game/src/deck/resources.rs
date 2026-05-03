use bevy::prelude::*;
use card_shuffling::{card::Card, cards::Cards};
use rand::rngs::StdRng;
use crate::card::{UnoAction, UnoColor};

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

        if self.get_color() == UnoColor::Wild {
            return true;
        }

        if card_color == UnoColor::Wild || card_color == self.card.get_color() {
            return true;
        }

        if card_action == self.get_action() {
            return true;
        }

        false
    }

    pub fn place_card(&mut self, card: Card<UnoAction, UnoColor>) -> Result<Card<UnoAction, UnoColor>, String> {
        if self.can_put(card) {
            self.card = card;
            Ok(card)
        } else {
            Err(format!("You can't place this card"))
        }
    }

    // Temporary
    pub fn place_nomatterwhat(&mut self, card: Card<UnoAction, UnoColor>) {
        self.card = card;
    }
}