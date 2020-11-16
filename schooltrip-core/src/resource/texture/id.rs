use crate::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct TextureId(Id);

impl From<TextureId> for Id {
    fn from(id: TextureId) -> Self {
        id.0
    }
}

impl From<TextureId> for u32 {
    fn from(id: TextureId) -> Self {
        id.0.into()
    }
}

impl From<Id> for TextureId {
    fn from(id: Id) -> Self {
        TextureId(id)
    }
}

impl From<u32> for TextureId {
    fn from(id: u32) -> Self {
        TextureId(id.into())
    }
}
