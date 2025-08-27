//! # Dice Roller
//! Adapted from Erik Horton's implementation.
//! https://blog.erikhorton.com/2024/08/25/building-a-bevy-plugin-for-rolling-dice.html

use avian3d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_die, spawn_ground_plane, spawn_containment_box),
    );

    app.add_systems(
        Update,
        roll_die.run_if(in_state(Screen::Gameplay).and(input_just_pressed(MouseButton::Left))),
    );

    app.add_systems(Update, update_die);
}

/// Enum to represent the current state of the die
#[derive(Debug, PartialEq, Eq)]
enum DieState {
    //  TODO: Review how die state is managed, because it seems to happen very fast?
    Rolling,
    Stationary,
    Cocked,
}

/// Component representing a die with its state and spin timer
#[derive(Component)]
struct Die {
    state: DieState,
    spin_timer: Timer,
}

const CUBE_SIDES: [Vec3; 6] = [
    Vec3::new(0.0, 1.0, 0.0),  // Top face (1)
    Vec3::new(1.0, 0.0, 0.0),  // Right face (2)
    Vec3::new(0.0, 0.0, -1.0), // Back face (3)
    Vec3::new(0.0, 0.0, 1.0),  // Front face (4)
    Vec3::new(-1.0, 0.0, 0.0), // Left face (5)
    Vec3::new(0.0, -1.0, 0.0), // Bottom face (6)
];

// Component for entities that display the roll result text
// #[derive(Component)]
// struct RollResultText;

/// Spawns the die and its associated components in the game world
fn spawn_die(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: Kill dice when returning to main menu.
    let dice_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/dice.glb"));
    let dice_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });

    commands.spawn((
        //  TODO: Make these physics properies more easilly editable. (low priority)
        SceneRoot(dice_handle),
        Transform {
            translation: Vec3::new(0.0, 1.5, 0.0),
            scale: Vec3::splat(0.8),
            ..Default::default()
        },
        MeshMaterial3d(dice_material),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        ExternalForce::new(Vec3::ZERO).with_persistence(false),
        ExternalTorque::new(Vec3::ZERO).with_persistence(false),
        ExternalImpulse::new(Vec3::ZERO).with_persistence(false),
        ExternalAngularImpulse::new(Vec3::ZERO).with_persistence(false),
        GravityScale(1.0),
        Mass(10.),
        LinearVelocity::default(),
        Die {
            state: DieState::Stationary,
            spin_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Name::new("Die"),
    ));
}

/// Spawns the ground plane to prevent the die from falling indefinitely
fn spawn_ground_plane(mut commands: Commands) {
    // TODO: Make this texture more interseting, like a dice mat. Should have some friction?
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 0.1, 10.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Ground Plane"),
    ));
}

/// Spawns the containment box with transparent walls to keep the die within bounds
fn spawn_containment_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //  TODO: Use the physics inspector to gauge the boundaries, becuase they should be
    //  big enough that the dice doesn't fall off the board. which right now it seems to do.
    let box_size = 1.5;
    let wall_thickness = 0.01;
    let wall_height = box_size * 2.0;
    let new_wall_height = wall_height * 2.0;

    let transparent_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.5, 0.5, 0.0),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let material = transparent_material.clone();
    let position = Vec3::new(-box_size - wall_thickness / 2.0, new_wall_height / 2.0, 0.0);
    let size = Vec3::new(wall_thickness, new_wall_height, wall_height);
    let collider_size = Vec3::new(
        wall_thickness / 2.0,
        new_wall_height / 2.0,
        wall_height / 2.0,
    );
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Static,
        Collider::cuboid(collider_size.x, collider_size.y, collider_size.z),
        Name::new("Left Wall"),
    ));

    let material = transparent_material.clone();
    let position = Vec3::new(box_size + wall_thickness / 2.0, new_wall_height / 2.0, 0.0);
    let size = Vec3::new(wall_thickness, new_wall_height, wall_height);
    let collider_size = Vec3::new(
        wall_thickness / 2.0,
        new_wall_height / 2.0,
        wall_height / 2.0,
    );
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Static,
        Collider::cuboid(collider_size.x, collider_size.y, collider_size.z),
        Name::new("Right Wall"),
    ));

    let material = transparent_material.clone();
    let position = Vec3::new(0.0, new_wall_height / 2.0, box_size + wall_thickness / 2.0);
    let size = Vec3::new(wall_height, new_wall_height, wall_thickness);
    let collider_size = Vec3::new(
        wall_height / 2.0,
        new_wall_height / 2.0,
        wall_thickness / 2.0,
    );
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Static,
        Collider::cuboid(collider_size.x, collider_size.y, collider_size.z),
        Name::new("Front Wall"),
    ));

    let material = transparent_material.clone();
    let position = Vec3::new(0.0, new_wall_height / 2.0, -box_size - wall_thickness / 2.0);
    let size = Vec3::new(wall_height, new_wall_height, wall_thickness);
    let collider_size = Vec3::new(
        wall_height / 2.0,
        new_wall_height / 2.0,
        wall_thickness / 2.0,
    );
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Static,
        Collider::cuboid(collider_size.x, collider_size.y, collider_size.z),
        Name::new("Back Wall"),
    ));

    let material = transparent_material.clone();
    let position = Vec3::new(0.0, new_wall_height + wall_thickness / 2.0, 0.0);
    let size = Vec3::new(wall_height, wall_thickness, wall_height);
    let collider_size = Vec3::new(wall_height / 2.0, wall_thickness / 2.0, wall_height / 2.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Static,
        Collider::cuboid(collider_size.x, collider_size.y, collider_size.z),
        Name::new("Top Wall"),
    ));

    let material = transparent_material.clone();
    let position = Vec3::new(0.0, -wall_thickness / 2.0, 0.0);
    let size = Vec3::new(wall_height, wall_thickness, wall_height);
    let collider_size = Vec3::new(wall_height / 2.0, wall_thickness / 2.0, wall_height / 2.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Static,
        Collider::cuboid(collider_size.x, collider_size.y, collider_size.z),
        Name::new("Bottom Wall"),
    ));
}

/// Rolls the die when the space key is pressed
fn roll_die(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut Die,
        &mut ExternalForce,
        &mut ExternalTorque,
        &mut ExternalImpulse,
        &mut ExternalAngularImpulse,
    )>,
) {
    //  TODO: rolling a die just pushes it in the same direction each time.
    for (
        mut die,
        mut external_force,
        mut external_torque,
        mut external_impulse,
        mut external_angular_impulse,
    ) in query.iter_mut()
    {
        if matches!(die.state, DieState::Stationary | DieState::Cocked) {
            println!("Rolling the die!");
            die.state = DieState::Rolling;
            die.spin_timer.reset();
            apply_initial_forces(
                &mut external_force,
                &mut external_torque,
                &mut external_impulse,
                &mut external_angular_impulse,
            );
        }
    }
}

/// Applies initial forces to the die when rolling
fn apply_initial_forces(
    external_force: &mut ExternalForce,
    external_torque: &mut ExternalTorque,
    external_impulse: &mut ExternalImpulse,
    external_angular_impulse: &mut ExternalAngularImpulse,
) {
    external_force.apply_force(Vec3::new(0.0, -19.62, 0.0));
    external_torque.apply_torque(Vec3::new(7.0, 1.0, 6.0));
    external_impulse.apply_impulse(Vec3::new(5.0, 1.0, 5.0));
    external_angular_impulse.apply_impulse(Vec3::new(7.0, 1.0, 6.0));
}

/// Updates the state of the die based on its velocity and position
fn update_die(
    // mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &mut Die,
        &Transform,
        &mut ExternalForce,
        &mut ExternalTorque,
        &mut ExternalImpulse,
        &mut ExternalAngularImpulse,
        &mut RigidBody,
        &LinearVelocity,
        &AngularVelocity,
    )>,
    // text_query: Query<Entity, With<RollResultText>>,
    // camera_query: Single<&Transform, With<Camera>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (
        mut die,
        transform,
        mut external_force,
        mut external_torque,
        mut external_impulse,
        mut external_angular_impulse,
        mut rigid_body,
        velocity,
        ang_velocity,
    ) in query.iter_mut()
    {
        die.spin_timer.tick(time.delta());

        if die.spin_timer.finished() {
            handle_die_state(
                &mut die,
                transform,
                &mut external_force,
                &mut external_torque,
                &mut external_impulse,
                &mut external_angular_impulse,
                &mut rigid_body,
                velocity,
                ang_velocity,
            );
        }
    }
}

/// Handles the state of the die after it has finished rolling
fn handle_die_state(
    // commands: &mut Commands,
    die: &mut Die,
    transform: &Transform,
    external_force: &mut ExternalForce,
    external_torque: &mut ExternalTorque,
    external_impulse: &mut ExternalImpulse,
    external_angular_impulse: &mut ExternalAngularImpulse,
    rigid_body: &mut RigidBody,
    velocity: &LinearVelocity,
    ang_velocity: &AngularVelocity,
    // text_query: &Query<Entity, With<RollResultText>>,
    // camera_transform: &Transform,
    // meshes: &mut ResMut<Assets<Mesh>>,
    // materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    match die.state {
        DieState::Rolling => check_roll_completion(die, transform, velocity, ang_velocity),
        DieState::Cocked => handle_cocked_die(
            // commands,
            // text_query,
            // camera_transform,
            // meshes,
            // materials,
            external_impulse,
            external_angular_impulse,
            die,
        ),
        DieState::Stationary => display_roll_result(
            // commands,
            // transform,
            // text_query,
            // camera_transform,
            // meshes,
            // materials,
            external_force,
            external_torque,
            external_impulse,
            external_angular_impulse,
            rigid_body,
        ),
    }
}

/// Checks if the die has finished rolling and updates its state accordingly
fn check_roll_completion(
    die: &mut Die,
    transform: &Transform,
    velocity: &LinearVelocity,
    ang_velocity: &AngularVelocity,
) {
    let threshold = 0.4;
    let cocked_tolerance = 0.4;

    if velocity.length() < threshold && ang_velocity.length() < threshold {
        let is_cocked = is_die_cocked(transform, cocked_tolerance);
        die.state = if is_cocked {
            println!("The die is cocked!");
            DieState::Cocked
        } else {
            println!("The die is stationary.");
            DieState::Stationary
        };
    }
}

/// Determines if the die is cocked based on its current position
fn is_die_cocked(transform: &Transform, tolerance: f32) -> bool {
    CUBE_SIDES.iter().any(|&side| {
        let world_normal = transform.rotation.mul_vec3(side);
        let abs_x = world_normal.x.abs();
        let abs_y = world_normal.y.abs();
        let abs_z = world_normal.z.abs();

        (abs_x > tolerance && abs_x < 1.0 - tolerance)
            || (abs_y > tolerance && abs_y < 1.0 - tolerance)
            || (abs_z > tolerance && abs_z < 1.0 - tolerance)
    })
}

/// Handles the situation when the die is cocked and displays a message
fn handle_cocked_die(
    // commands: &mut Commands,
    // text_query: &Query<Entity, With<RollResultText>>,
    // camera_transform: &Transform,
    // meshes: &mut ResMut<Assets<Mesh>>,
    // materials: &mut ResMut<Assets<StandardMaterial>>,
    external_impulse: &mut ExternalImpulse,
    external_angular_impulse: &mut ExternalAngularImpulse,
    die: &mut Die,
) {
    external_impulse.set_impulse(Vec3::new(0.0, 0.0, 0.0));
    external_angular_impulse.set_impulse(Vec3::new(-0.15, -0.05, -0.15));
    die.state = DieState::Rolling;
}

/// Displays the result of the die roll and updates the game state
fn display_roll_result(
    external_force: &mut ExternalForce,
    external_torque: &mut ExternalTorque,
    external_impulse: &mut ExternalImpulse,
    external_angular_impulse: &mut ExternalAngularImpulse,
    rigid_body: &mut RigidBody,
) {
    // let mut max_dot = -1.0;
    // let mut face_up = 0;
    //
    // for (i, side) in CUBE_SIDES.iter().enumerate() {
    //     let dot_product = transform.rotation.mul_vec3(*side).dot(Vec3::Y);
    //     if dot_product > max_dot {
    //         max_dot = dot_product;
    //         face_up = i + 1;
    //     }
    // }

    external_force.set_force(Vec3::ZERO);
    external_torque.set_torque(Vec3::ZERO);
    external_impulse.set_impulse(Vec3::ZERO);
    external_angular_impulse.set_impulse(Vec3::ZERO);

    *rigid_body = RigidBody::Dynamic;
}

// Creates a text mesh to display in the game world
// fn create_text_mesh(
//     // text: &str,
//     // font_data: &'static [u8],
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
//     color: Color,
//     camera_transform: &Transform,
// ) -> impl Bundle {
//     // let mut generator = MeshGenerator::new(font_data);
//     // let transform = Mat4::from_scale(Vec3::new(0.1, 0.1, 0.1)).to_cols_array();
//     // let text_mesh: MeshText = generator
//     //     .generate_section(text, false, Some(&transform))
//     //     .expect("Failed to generate text mesh");
//
//     // let positions: Vec<[f32; 3]> = text_mesh
//     //     .vertices
//     //     .chunks(3)
//     //     .map(|c| [c[0], c[1], 0.5])
//     //     .collect();
//     // let uvs = vec![[0.0, 0.0]; positions.len()];
//
//     let mut mesh = Mesh::new(
//         PrimitiveTopology::TriangleList,
//         RenderAssetUsages::default(),
//     );
//     // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//     // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
//     mesh.compute_flat_normals();
//
//     let text_position = Vec3::new(-0.2, 9.0, 0.1);
//     let text_rotation =
//         Quat::from_rotation_x(std::f32::consts::PI) * Quat::from_rotation_x(std::f32::consts::PI);
//
//     (
//         Mesh3d(meshes.add(mesh)),
//         MeshMaterial3d(materials.add(color)),
//         Transform {
//             translation: text_position,
//             rotation: camera_transform.rotation * text_rotation,
//             scale: Vec3::splat(0.5),
//             ..Default::default()
//         }
//     )
// }
