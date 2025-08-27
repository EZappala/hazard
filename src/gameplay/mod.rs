use avian3d::PhysicsPlugins;
use bevy::prelude::*;

use crate::{
    gameplay::{
        cursors::{init_cursor_icons, set_cursors},
        level::spawn_level,
        player::teardown_player,
    },
    screens::Screen,
};

mod cursors;
mod level;
mod models;
mod player;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        level::plugin,
        models::plugin,
        PhysicsPlugins::default(),
    ));
    app.add_systems(Startup, (init_cursor_icons, set_cursors).chain());
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    app.add_systems(OnExit(Screen::Gameplay), teardown_player);
}
