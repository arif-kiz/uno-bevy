use std::ops::Neg;

use bevy::prelude::*;
use crate::game::player::NUMBER_OF_PLAYERS;

#[derive(Default, Clone, Copy)]
enum Direction {
    #[default]
    Clockwise,
    AntiClockwise,
}

impl Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Clockwise => Self::AntiClockwise,
            Self::AntiClockwise => Self::Clockwise,
        }
    }
}

#[derive(Resource)]
pub struct PlayerTurn {
    current_player: usize,
    direction: Direction,
    players: [Option<Entity>; NUMBER_OF_PLAYERS],
}

impl Default for PlayerTurn {
    fn default() -> Self {
        Self {
            current_player: 0,
            direction: Direction::Clockwise,
            players: [None; NUMBER_OF_PLAYERS],
        }
    }
}

impl PlayerTurn {
    pub fn get_current_player(&self) -> Entity {
        self.players[self.current_player].expect("Current player not set")
    }

    pub fn set_player(&mut self, player: Entity, id: usize) {
        self.players[id] = Some(player);
    }

    pub fn change_player(&mut self) {
        self.current_player = match self.direction {
            Direction::Clockwise => (self.current_player + 1) % NUMBER_OF_PLAYERS,
            Direction::AntiClockwise => (self.current_player + NUMBER_OF_PLAYERS - 1) % NUMBER_OF_PLAYERS,
        };
    }

    pub fn change_direction(&mut self) {
        self.direction = -self.direction
    }
}