use bevy::prelude::*;

pub(crate) struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FontResource::default())
            .add_startup_system(startup.system());
    }
}

pub(crate) enum FontKind {
    FiraCode,
}

impl From<FontKind> for &str {
    fn from(kind: FontKind) -> Self {
        match kind {
            FontKind::FiraCode => "fonts/FiraCode-VF.ttf",
        }
    }
}

fn startup(asset_server: Res<AssetServer>, mut resource: ResMut<FontResource>) {
    macro_rules! load_font {
        ($($variant:ident,)+) => {
            $(
                let handle = asset_server.load::<Font, &str>(FontKind::$variant.into());
                resource.add(FontKind::$variant.into(), handle);
            )+
        };
    }

    load_font![FiraCode,];
}

pub(crate) trait FontGet<T> {
    fn get(&self, t: T) -> Option<Handle<Font>>;
}

impl FontGet<FontKind> for FontResource {
    fn get(&self, t: FontKind) -> Option<Handle<Font>> {
        self.map.get(t.into()).map(Clone::clone)
    }
}

#[derive(Default)]
pub(crate) struct FontResource {
    map: std::collections::HashMap<String, Handle<Font>>,
}

impl FontResource {
    fn add(&mut self, name: &str, handle: Handle<Font>) {
        self.map.insert(name.to_owned(), handle);
    }
}
