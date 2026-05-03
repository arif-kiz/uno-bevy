use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub enum PlayerVisualsEvent {
    #[default]
    RefreshAll,
    Refresh(usize),
}

#[derive(Event, Clone, Debug, Default)]
pub struct RefreshPlayerVisualsEvent(PlayerVisualsEvent);

impl RefreshPlayerVisualsEvent {
    pub fn all() -> Self {
        Self(PlayerVisualsEvent::RefreshAll)
    }

    pub fn new(id: usize) -> Self {
        Self(PlayerVisualsEvent::Refresh(id))
    }

    pub fn get_event(&self) -> &PlayerVisualsEvent {
        &self.0
    }
}