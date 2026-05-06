use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub enum PlayerVisualsEvent {
    #[default]
    RefreshAll,
    Refresh(Entity),
}

#[derive(Event, Clone, Debug)]
pub struct RefreshPlayerVisualsEvent(PlayerVisualsEvent);

impl RefreshPlayerVisualsEvent {
    pub fn all() -> Self {
        Self(PlayerVisualsEvent::RefreshAll)
    }

    pub fn new(player_entity: Entity) -> Self {
        Self(PlayerVisualsEvent::Refresh(player_entity))
    }

    pub fn get_event(&self) -> &PlayerVisualsEvent {
        &self.0
    }
}