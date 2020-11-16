mod common;
mod resource;
mod ui;

use bevy::prelude::*;
use common::*;
use resource::*;
use std::collections::HashMap;
use ui::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiPlugin)
        .add_plugin(ResourcePlugin)
        .run();
}
