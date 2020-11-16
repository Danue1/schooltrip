mod character;
mod font;
mod item;
mod texture;

use crate::*;
pub(crate) use character::*;
pub(crate) use font::*;
pub(crate) use item::*;
pub(crate) use texture::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FontPlugin)
            .add_plugin(TexturePlugin)
            .add_plugin(ItemPlugin)
            .add_plugin(CharacterPlugin);
    }
}
