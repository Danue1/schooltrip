use crate::*;

pub(crate) struct ItemResourcePlugin;

impl Plugin for ItemResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ItemResource::default())
            .add_startup_system(startup.system());
    }
}

#[derive(Default)]
pub(crate) struct ItemResource {
    items: HashMap<ItemId, Item>,
}

impl ItemResource {
    pub(crate) fn get(&self, id: &ItemId) -> Option<&Item> {
        self.items.get(id)
    }

    fn insert<Id: Into<ItemId>>(&mut self, id: Id, item: Item) {
        self.items.insert(id.into(), item);
    }
}

fn startup(mut resource: ResMut<ItemResource>) {
    let mut next_id = {
        let mut index = 0;
        move || -> ItemId {
            let id = index.into();
            index += 1;
            id
        }
    };

    resource.insert(
        next_id(),
        Item::new("Danuel".to_owned(), "Functional Programmer".to_owned()),
    );
}
