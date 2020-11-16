mod event;
mod keyboard_event_handler;

use crate::*;
use event::*;
use keyboard_event_handler::*;

pub(crate) struct PreferencePlugin;

impl Plugin for PreferencePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Preference::default())
            .add_plugin(EventPlugin)
            .add_plugin(KeyboardEventHandlerPlugin);
    }
}

#[derive(Debug, Default)]
pub(crate) struct Preference {
    //
}
