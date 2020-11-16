use crate::*;

pub(crate) struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(UiState::default());
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum UiState {
    None,
    Inventory,
    Preference,
}

impl Default for UiState {
    fn default() -> Self {
        UiState::None
    }
}

impl UiState {
    pub(crate) fn close(&mut self) {
        *self = UiState::None;
    }

    pub(crate) fn open_inventory(&mut self) {
        *self = UiState::Inventory;
    }

    pub(crate) fn open_preference(&mut self) {
        *self = UiState::Preference;
    }

    pub(crate) fn is_closed(&self) -> bool {
        *self == UiState::None
    }

    pub(crate) fn is_open_inventory(&self) -> bool {
        *self == UiState::Inventory
    }

    pub(crate) fn is_open_preference(&self) -> bool {
        *self == UiState::Preference
    }
}
