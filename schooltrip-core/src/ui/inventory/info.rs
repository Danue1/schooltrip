use crate::*;

#[derive(Debug)]
pub(crate) struct ItemInfo {
    id: ItemId,
    count: u8,
}

impl ItemInfo {
    pub(crate) fn new(id: ItemId, count: u8) -> Self {
        ItemInfo { id, count }
    }

    pub(crate) fn add_count(&mut self, count: u8) {
        self.count += count;
    }

    pub(crate) fn sub_count(&mut self, count: u8) {
        self.count -= count;
    }
}
