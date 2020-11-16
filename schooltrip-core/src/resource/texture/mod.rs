mod id;
mod resource;

use crate::*;
pub(crate) use id::*;
pub(crate) use resource::*;

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TextureResourcePlugin);
    }
}
