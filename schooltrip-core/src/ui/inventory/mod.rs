mod event;
mod info;
mod keyboard_event_handler;

use crate::*;
use event::*;
pub(crate) use info::*;
use keyboard_event_handler::*;

pub(crate) struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Inventory::default())
            .add_plugin(EventPlugin)
            .add_plugin(KeyboardEventHandlerPlugin);
    }
}

#[derive(Debug, Default)]
pub(crate) struct Inventory {
    items: Vec<ItemInfo>,
}

impl Inventory {
    pub(crate) fn add_item(&mut self, index: usize, count: u8) {
        if let Some(info) = self.items.get_mut(index) {
            info.add_count(count);
        }
    }

    pub(crate) fn sub_item(&mut self, index: usize, count: u8) {
        if let Some(info) = self.items.get_mut(index) {
            info.sub_count(count);
        }
    }

    pub(crate) fn insert_item(&mut self, index: usize, id: ItemId) {
        self.items.insert(index, ItemInfo::new(id, 0))
    }

    pub(crate) fn remove_item(&mut self, index: usize) -> ItemInfo {
        self.items.remove(index)
    }
}
