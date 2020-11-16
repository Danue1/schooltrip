use crate::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup_setup.system())
            .add_system(setup.system());
    }
}

fn startup_setup() {
    //
}

fn setup() {
    //
}
