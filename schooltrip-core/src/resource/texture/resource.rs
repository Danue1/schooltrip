use crate::*;

pub(crate) struct TextureResourcePlugin;

impl Plugin for TextureResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TextureResource::default())
            .add_startup_system(startup.system());
    }
}

#[derive(Default)]
pub(crate) struct TextureResource {
    textures: HashMap<TextureId, Handle<Texture>>,
}

impl TextureResource {
    pub(crate) fn get(&self, id: &TextureId) -> Option<&Handle<Texture>> {
        self.textures.get(id)
    }

    fn insert<Id: Into<TextureId>>(&mut self, id: Id, texture: Handle<Texture>) {
        self.textures.insert(id.into(), texture);
    }
}

fn startup(mut resource: ResMut<TextureResource>, asset_server: Res<AssetServer>) {
    let mut next_id = {
        let mut index = 0;
        move || -> TextureId {
            let id = index.into();
            index += 1;
            id
        }
    };

    resource.insert(
        next_id(),
        asset_server.load("ui/inventory/slot/texture.png"),
    );
}
