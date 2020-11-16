mod inventory;
mod preference;
mod state;

use crate::*;
pub(crate) use inventory::*;
pub(crate) use preference::*;
pub(crate) use state::*;

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_plugin(UiStatePlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(PreferencePlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());
}
