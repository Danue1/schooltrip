use crate::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct ItemId(Id);

impl From<ItemId> for Id {
    fn from(id: ItemId) -> Self {
        id.0
    }
}

impl From<ItemId> for u32 {
    fn from(id: ItemId) -> Self {
        id.0.into()
    }
}

impl From<Id> for ItemId {
    fn from(id: Id) -> Self {
        ItemId(id)
    }
}

impl From<u32> for ItemId {
    fn from(id: u32) -> Self {
        ItemId(id.into())
    }
}
