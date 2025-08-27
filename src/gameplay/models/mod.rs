use bevy::prelude::*;
pub mod dice;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(dice::plugin);
}
