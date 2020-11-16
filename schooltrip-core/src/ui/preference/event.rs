use crate::*;

pub(crate) struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<OpenPreferenceEvent>()
            .add_event::<ClosePreferenceEvent>()
            .add_resource(State::default())
            .add_system(open_preference.system())
            .add_system(close_preference.system());
    }
}

#[derive(Default)]
struct State {
    id: Option<Entity>,
}

#[derive(Default)]
pub(crate) struct OpenPreferenceEvent;

#[derive(Default)]
pub(crate) struct ClosePreferenceEvent;

fn open_preference(
    mut commands: Commands,
    event: Res<Events<OpenPreferenceEvent>>,
    mut event_reader: Local<EventReader<OpenPreferenceEvent>>,
    mut ui_state: ResMut<UiState>,
    mut state: ResMut<State>,
    font_resource: Res<FontResource>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in event_reader.iter(&event) {
        ui_state.open_preference();
        state.id = commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn(ButtonComponents {
                        style: Style {
                            size: Size::new(Val::Px(256.0), Val::Px(256.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: materials.add(ColorMaterial {
                            color: Color::BLACK,
                            texture: Some(asset_server.load("ui/inventory/slot/texture.png")),
                        }),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextComponents {
                            text: Text {
                                value: "Preference".to_owned(),
                                font: font_resource.get(FontKind::FiraCode).unwrap(),
                                style: TextStyle {
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            })
            .current_entity();
    }
}

fn close_preference(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    event: Res<Events<ClosePreferenceEvent>>,
    mut event_reader: Local<EventReader<ClosePreferenceEvent>>,
    mut state: ResMut<State>,
) {
    for _ in event_reader.iter(&event) {
        ui_state.close();
        if let Some(entity) = state.id {
            commands.despawn_recursive(entity);
            state.id = None;
        }
    }
}
