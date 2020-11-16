use super::*;
use crate::*;

pub(crate) struct KeyboardEventHandlerPlugin;

impl Plugin for KeyboardEventHandlerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(toggle_preference.system());
    }
}

fn toggle_preference(
    keyboard_input: Res<Input<KeyCode>>,
    state: Res<UiState>,
    mut open_event: ResMut<Events<OpenPreferenceEvent>>,
    mut close_event: ResMut<Events<ClosePreferenceEvent>>,
) {
    if keyboard_input.just_pressed(KeyCode::Comma) {
        if state.is_closed() {
            open_event.send(OpenPreferenceEvent);
        }

        if state.is_open_preference() {
            close_event.send(ClosePreferenceEvent);
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        if state.is_open_preference() {
            close_event.send(ClosePreferenceEvent);
        }
    }
}
