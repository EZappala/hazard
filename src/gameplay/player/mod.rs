use bevy::prelude::*;

mod dice_roller;
mod interactables;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(dice_roller::plugin);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Name::new("Player"))]
pub struct Player;

/// The player character.
// 
pub(super) fn player() -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Camera3d::default(),
        Camera {
            order: 1,
            ..default()
        },
        Transform::from_xyz(0., 10., 0.).looking_at(Vec3::ZERO, Vec3::Y),
    )
}

pub(super) fn teardown_player(mut commands: Commands, player: Single<Entity, With<Player>>) {
    commands.entity(*player).despawn();
}
