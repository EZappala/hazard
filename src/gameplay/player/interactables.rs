// use crate::{
//     gameplay::models::suburban::{BuildingType, CurrentBuilding, SuburbanBuildings},
//     screens::Screen,
//     theme::{
//         palette::{
//             BUTTON_BACKGROUND, BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, BUTTON_TEXT,
//         },
//         prelude::InteractionPalette,
//     },
// };
// use bevy::{ecs::schedule::NotSystem, input::common_conditions::input_just_pressed, prelude::*};
// use bevy_rts_camera::RtsCamera;
// 
//
// pub(crate) fn plugin(app: &mut App) {
//     app.add_systems(OnEnter(Screen::Gameplay), render_ui);
//     app.add_systems(
//         Update,
//         (select_building_on_click)
//             .run_if(in_state(Screen::Gameplay).and(input_just_pressed(MouseButton::Left))),
//     );
//
//     app.add_systems(
//         Update,
//         (preview_building)
//             .run_if(in_state(Screen::Gameplay).and(resource_exists::<CurrentBuilding>)),
//     );
//
//     app.add_systems(
//         Update,
//         (place_building_on_click).run_if(
//             in_state(Screen::Gameplay)
//                 .and(resource_exists::<CurrentBuilding>)
//                 .and(
//                     input_just_pressed(MouseButton::Left)
//                         .and(any_with_component::<PreviewBuilding>),
//                 ),
//         ),
//     );
//
//     app.add_systems(
//         Update,
//         cleanup_current_building.run_if(resource_removed::<CurrentBuilding>),
//     );
//
//     app.add_systems(
//         Update,
//         (clear_building_selection).run_if(
//             in_state(Screen::Gameplay)
//                 .and(input_just_pressed(MouseButton::Right))
//                 .and(any_with_component::<PreviewBuilding>),
//         ),
//     );
//
//     app.add_systems(OnExit(Screen::Gameplay), cleanup_ui);
// }
//
// #[derive(Component)]
// struct HotbarUi;
//
// 
// fn render_ui(mut commands: Commands, buildings: Res<SuburbanBuildings>) {
//     commands
//         .spawn((
//             Node {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::End,
//                 flex_direction: FlexDirection::Row,
//                 ..default()
//             },
//             HotbarUi,
//         )
//         .with_children(|parent| {
//             for (btype, handle) in buildings.buildings.iter() {
//                 parent.spawn((
//                     Button,
//                     Node {
//                         width: Val::Px(50.0),
//                         height: Val::Px(50.0),
//                         justify_content: JustifyContent::Center,
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     Name::new(handle.name().to_string()),
//                     btype.clone(),
//                     BackgroundColor(BUTTON_BACKGROUND),
//                     InteractionPalette {
//                         none: BUTTON_BACKGROUND,
//                         hovered: BUTTON_HOVERED_BACKGROUND,
//                         pressed: BUTTON_PRESSED_BACKGROUND,
//                     },
//                     children![(
//                         Name::new("Button Text"),
//                         Text(handle.name().to_string()),
//                         TextFont::from_font_size(40.0),
//                         TextColor(BUTTON_TEXT),
//                         // Don't bubble picking events from the text up to the button.
//                         Pickable::IGNORE,
//                     )],
//                 ));
//             }
//         });
// }
//
// 
// fn cleanup_ui(mut commands: Commands, ui: Single<Entity, With<HotbarUi>>) {
//     commands.entity(*ui).despawn()
// }
//
// 
// fn select_building_on_click(
//     mut commands: Commands,
//     mut interaction_q: Query<(&Interaction, &BuildingType), (Changed<Interaction>, With<Button>)>,
//     buildings: Res<SuburbanBuildings>,
//     gltf: Res<Assets<Gltf>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     for (interaction, building_type) in interaction_q.iter_mut() {
//         if *interaction == Interaction::Pressed {
//             match CurrentBuilding::new(building_type, &buildings, &gltf, &mut materials) {
//                 Ok(current_building) => {
//                     commands.insert_resource(current_building);
//                     info!("Selected {:?}", building_type);
//                 }
//                 Err(e) => {
//                     error!("Couldn't construct current_building resource.{e:?}");
//                 }
//             }
//         }
//     }
// }
//
// 
// fn cleanup_current_building(
//     mut commands: Commands,
//     mut preview_q: Query<Entity, With<PreviewBuilding>>,
// ) {
//     for entity in preview_q.iter_mut() {
//         commands.entity(entity).despawn();
//     }
// }
//
// 
// fn preview_building(
//     mut commands: Commands,
//     mut preview_q: Query<Entity, With<PreviewBuilding>>,
//     window: Single<&Window>,
//     camera: Single<(&Camera, &GlobalTransform), With<RtsCamera>>,
// ) {
//     // Compute raycast from mouse to world
//     if let Some(cursor_pos) = window.cursor_position() {
//         let (camera, cam_transform) = camera.into_inner();
//         if let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_pos) {
//             // Find ground intersection (assume y=0)
//             let t = -ray.origin.y / ray.direction.y;
//             let world_pos = ray.origin + ray.direction * t;
//
//             // Spawn preview if it doesn't exist
//             if preview_q.is_empty() {
//                 commands.spawn((Transform::from_translation(world_pos), PreviewBuilding));
//             } else {
//                 // Move existing preview
//                 for entity in preview_q.iter_mut() {
//                     commands
//                         .entity(entity)
//                         .insert(Transform::from_translation(world_pos));
//                 }
//             }
//         }
//     }
// }
//
// #[derive(Component)]
// struct PreviewBuilding;
//
// 
// fn place_building_on_click(
//     preview_q: Single<&Transform, With<PreviewBuilding>>,
//     current_building: Res<CurrentBuilding>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut commands: Commands,
// ) {
//     let tf = *preview_q;
//     let id = current_building.mat_id();
//     if let Some(mat_asset) = materials.get_mut(id) {
//         mat_asset.diffuse_transmission = 1.0;
//     } else {
//         warn!("Invalid material asset with id {id:?}");
//     }
//
//     commands.spawn((SceneRoot(current_building.scene().clone()), *tf));
// }
//
// 
// fn clear_building_selection(mut commands: Commands) {
//     commands.remove_resource::<CurrentBuilding>();
// }
