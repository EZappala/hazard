use bevy::prelude::*;

use hazard::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
