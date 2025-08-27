use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::music, gameplay::player, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            player::player(),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            ),
            (
                DirectionalLight {
                    color: Color::srgb(1.0, 1.0, 0.9),
                    illuminance: 100_000.0,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(0.0, 100.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
                Name::new("Sunlight"),
            ),
            (
                PointLight {
                    intensity: 2000.,
                    range: 20.,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(4.0, 8.0, 4.0),
                Name::new("Point Light"),
            )
        ],
    ));
}
