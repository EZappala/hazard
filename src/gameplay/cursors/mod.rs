use bevy::{
    prelude::*,
    winit::cursor::{CursorIcon, CustomCursor, CustomCursorImage},
};

#[derive(Resource, Default)]
pub struct CursorIcons(pub [CursorIcon; 1]);

pub fn init_cursor_icons(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(CursorIcons(
        [assets.load("images/cursor/pointer_c_shaded.png")].map(|c| -> CursorIcon {
            CustomCursor::Image(CustomCursorImage {
                handle: c.clone(),
                ..default()
            })
            .into()
        }),
    ));
}

pub fn set_cursors(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    cursors: Res<CursorIcons>,
) {
    commands
        .entity(*window)
        .insert(cursors.0.first().unwrap().clone());
}
