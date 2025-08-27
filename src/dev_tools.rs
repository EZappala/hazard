//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions,
    input::{common_conditions::input_just_pressed, keyboard::KeyboardInput},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{EguiContext, EguiContexts, egui};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DevelopmentConsole>();
    app.init_resource::<DevelopmentOverlay>();
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_development_console.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    app.add_systems(
        Update,
        draw_development_ui.run_if(resource_exists_and_equals(DevelopmentConsole(true))),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

#[derive(Resource, Reflect, Default, PartialEq)]
#[reflect(Resource)]
struct DevelopmentConsole(bool);

impl DevelopmentConsole {
    fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

#[derive(Resource, Reflect, Default, PartialEq)]
#[reflect(Resource)]
enum DevelopmentOverlay {
    #[default]
    None,
    Ui,
    Physics,
}

// UiDebugOptions
fn toggle_development_console(mut options: ResMut<DevelopmentConsole>) {
    options.toggle();
}

//  TODO: Make these buttons actually do something. There needs to be a system that updates on the
//  state change of Development Overlay and toggles the relevent one.
fn draw_development_ui(
    mut ctxs: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    // mut camera: Single<&mut Camera, Without<EguiContext>>,
    // window: Single<&mut Window, With<PrimaryWindow>>,
    mut overlay: ResMut<DevelopmentOverlay>,
) -> Result {
    //  TODO: Kill UI on exiting this screen to main menu
    let ctx = ctxs.ctx_mut()?;
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Debug Overlays");
            if ui.button("UI").clicked() {
                *overlay = DevelopmentOverlay::Ui;
            }

            if ui.button("Physics").clicked() {
                *overlay = DevelopmentOverlay::Physics;
            }

            // TODO: Alter behaviour of pause to check if debug ui is open, this
            // escape should take prescedent over the pause menu's escape trigger.
            if keys.just_pressed(KeyCode::Escape) {
                *overlay = DevelopmentOverlay::None
            }
        });
    Ok(())
}
