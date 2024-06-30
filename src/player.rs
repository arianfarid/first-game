
use bevy::{app::{App, Plugin}, prelude::*};
use bevy::window::PrimaryWindow;
use crate::{GameState, GameLevel, beam::{Beam, BeamType}, canon::{CanonPlugin}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
       app
       .add_plugins(CanonPlugin)
       .add_systems(OnEnter((GameLevel::SpaceOne)), setup)
       .add_systems(OnEnter(GameLevel::SpaceOne), setup_camera)
       .add_systems(Update, (toggle_pause))
       .add_systems(
            Update, 
            (toggle_pause, move_user, rotate_user, user_fire_beam)
                .chain().run_if(in_state(GameState::Playing))
        );
    }
}

#[derive(Component, Debug)]
pub struct Player {
    left_weapon: WeaponType,
}

#[derive(Component, Debug, Default)]
pub enum WeaponType {
    #[default]
    None,
    PlasmaCanon
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32
}

#[derive(Component, Debug)]
struct Acceleration {
    x: f32,
    y: f32
}
pub const USER_SPEED: f32 = 200.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ship.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Player { left_weapon: WeaponType::PlasmaCanon},
        Velocity {x: 0., y: 0.},
        Acceleration {x: 0., y: 0.},
    ));
}

fn move_user(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    // Todo: map to an optional user input
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        x_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        x_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        y_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        y_direction -= 1.0;
    }

    let new_player_position_x =
        player_transform.translation.x + x_direction * USER_SPEED * time.delta_seconds();
    let new_player_position_y =
        player_transform.translation.y + y_direction * USER_SPEED * time.delta_seconds();

    player_transform.translation.x = new_player_position_x;
    player_transform.translation.y = new_player_position_y;
}

fn rotate_user(
    mut player_query: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) { 
    let (camera, camera_transform) = q_camera.single();
    let mut player_transform = player_query.single_mut();
    let player_translation = player_transform.translation.xy();

    let cursor_pos = q_windows.single().cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate());
    match cursor_pos {
       Some(v) => {
        let to_mouse = (v - player_translation).normalize();
        let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));
        player_transform.rotation = rotate_to_mouse;
       },
       None => {},
    }
}

fn user_fire_beam(
    mut player_query: Query<&mut Transform, With<Player>>,
    mouse_buttons:Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let player_transform = player_query.single_mut();
    let player_location = player_transform.translation;
    let player_angle = player_transform.rotation;

    // Convert to axis of rotation
    let axis = (player_angle * Vec3::Y).xy();
    
    if keyboard_input.pressed(KeyCode::Space) || mouse_buttons.pressed(MouseButton::Left) {
        let mut spawn_transform = Transform::from_scale(Vec3::splat(1.0));
        spawn_transform.translation = player_location;
        spawn_transform.rotation = player_angle;
        commands.spawn((
            SpriteBundle {
                transform: spawn_transform,
                texture: asset_server.load("wave.png"),
                ..default()
            },
            //todo: 2 weapons, should be enum w/ params
            Beam::new(BeamType::Proton, Vec2::new(axis.x, axis.y))
        ));
    }
}

fn toggle_pause(
    curr_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if curr_state.get() == &GameState::Playing {
            next_state.set(GameState::Paused)
        } else {
            next_state.set(GameState::Playing)
        }
    }
}


/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;


fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}