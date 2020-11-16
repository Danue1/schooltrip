use super::*;
use crate::*;

pub(crate) struct KeyboardEventHandlerPlugin;

impl Plugin for KeyboardEventHandlerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(toggle_inventory.system());
    }
}

fn toggle_inventory(
    keyboard_input: Res<Input<KeyCode>>,
    state: Res<UiState>,
    mut open_event: ResMut<Events<OpenInventoryEvent>>,
    mut close_event: ResMut<Events<CloseInventoryEvent>>,
) {
    if keyboard_input.just_pressed(KeyCode::I) {
        if state.is_closed() {
            open_event.send(OpenInventoryEvent);
        }

        if state.is_open_inventory() {
            close_event.send(CloseInventoryEvent);
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        if state.is_open_inventory() {
            close_event.send(CloseInventoryEvent);
        }
    }
}
