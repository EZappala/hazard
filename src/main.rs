use bevy::prelude::*;

use proto2::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
