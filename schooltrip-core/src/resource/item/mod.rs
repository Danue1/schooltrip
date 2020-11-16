mod id;
mod resource;

use crate::*;
pub(crate) use id::*;
pub(crate) use resource::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ItemResourcePlugin);
    }
}

#[derive(Debug)]
pub(crate) struct Item {
    name: String,
    description: String,
}

impl Item {
    pub(crate) fn new(name: String, description: String) -> Self {
        Item { name, description }
    }
}
