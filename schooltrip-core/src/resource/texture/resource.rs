use crate::*;

pub(crate) struct TextureResourcePlugin;

impl Plugin for TextureResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TextureResource::default())
            .add_startup_system(startup.system());
    }
}

#[derive(Debug, Default)]
pub(crate) struct TextureResource {
    textures: HashMap<String, Handle<Texture>>,
}

impl TextureResource {
    pub(crate) fn get(&self, id: &str) -> Option<&Handle<Texture>> {
        self.textures.get(id)
    }

    fn insert(&mut self, id: &str, texture: Handle<Texture>) {
        self.textures.insert(id.to_owned(), texture);
    }
}

pub(crate) const SLOT_TEXTURE: &'static str = "ui/inventory/slot/texture.png";

fn startup(mut resource: ResMut<TextureResource>, asset_server: Res<AssetServer>) {
    macro_rules! load_texture {
        ($($id:expr,)+) => {
            $(
                resource.insert($id, asset_server.load($id));
            )+
        };
    }

    load_texture![SLOT_TEXTURE,];

    dbg!(resource);
}
