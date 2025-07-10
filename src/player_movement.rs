use bevy::prelude::*;
use bevy::render::mesh::PlaneMeshBuilder;
use avian3d::prelude::*;
use bevy_ichun::{IchunPlugin, input::KccInputConfig, kcc::Kcc, movement::KccMovementConfig};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct CameraOffsets{
    fp: Vec3,
    tp: Vec3,
    os: Vec3,
}
#[derive(Resource)]
struct PlayerDefaults{
    pub player_height: f32,
    pub player_radius: f32,
}
#[derive(Resource)]
enum CameraPOV {
    FirstPerson,
    ThirdPerson,
    OverShoulder,
}
#[derive(Resource)]
struct Config {
    keybind_player_move_up: KeyCode,
    keybind_player_move_left: KeyCode,
    keybind_player_move_down: KeyCode,
    keybind_player_move_right: KeyCode,
    //keybind_player_interact
    //keybind_ui_menu
    //etc
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keybind_player_move_up: KeyCode::KeyW,
            keybind_player_move_left: KeyCode::KeyA,
            keybind_player_move_down: KeyCode::KeyS,
            keybind_player_move_right: KeyCode::KeyD,
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), IchunPlugin))
        .insert_resource(Config::default())
        .insert_resource(CameraOffsets{
            fp: Vec3::new(0.0, 0.0, 0.0),
            tp: Vec3::new(0.0, 8.0, 10.0),
            os: Vec3::new(1.0, 4.0, 5.0),
        })
        .insert_resource(PlayerDefaults{
            player_height: 1.0,
            player_radius: 0.5,
        })
        .insert_resource(CameraPOV::FirstPerson)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, camera_movement)
        .add_systems(Update, adjust_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_defaults: Res<PlayerDefaults>,
) {
    let grass_tex = asset_server.load("grass.png");
    let textured_material = materials.add(StandardMaterial {
        base_color_texture: Some(grass_tex.into()),
        ..Default::default()
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
        ));

    commands.spawn((
        Player,
        Kcc::default(),
        KccMovementConfig::default(),
        KccInputConfig::default(),
        Mesh3d(meshes.add(Mesh::from(Capsule3d::new(player_defaults.player_radius, player_defaults.player_height)))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 1.0))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        GlobalTransform::default(),
        )).with_children(|parent| {
        parent.spawn((
            MainCamera,
            Camera3d {
                ..default()
            },
            Transform::from_xyz(0.0, 1.5, 0.0),
        ));
    });

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
        Mesh3d(meshes.add(PlaneMeshBuilder::from_size(Vec2::splat(50.0))
            .subdivisions(0)
            .build()
        )),
        MeshMaterial3d(textured_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        ));
}

fn camera_movement(
    plyr_query: Query<&Transform, With<Player>>,
    mut cam_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    offsets: Res<CameraOffsets>,
    view_mode: Res<CameraPOV>,
) {
    let Ok(player_transform) = plyr_query.single() else { return };
    let Ok(mut camera_transform) = cam_query.single_mut() else { return };

    match *view_mode {
        CameraPOV::FirstPerson => {
            camera_transform.translation = offsets.fp;
            camera_transform.rotation = player_transform.rotation;
        }
        CameraPOV::ThirdPerson => {
            camera_transform.translation = offsets.tp;
            camera_transform.look_at(Vec3::ZERO, Vec3::Y);
        }
        CameraPOV::OverShoulder => {
            camera_transform.translation = offsets.os;
            camera_transform.look_at(Vec3::ZERO, Vec3::Y);
        }
    }
}

fn adjust_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut view_mode: ResMut<CameraPOV>,
) {
    if keys.just_pressed(KeyCode::F5) {
        *view_mode = match *view_mode {
            CameraPOV::FirstPerson => CameraPOV::ThirdPerson,
            CameraPOV::ThirdPerson => CameraPOV::OverShoulder,
            CameraPOV::OverShoulder => CameraPOV::FirstPerson,
        };
    }
}
