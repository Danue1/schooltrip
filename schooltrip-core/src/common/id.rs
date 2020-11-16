#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct Id(u32);

impl From<Id> for u32 {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<u32> for Id {
    fn from(id: u32) -> Self {
        Id(id)
    }
}
