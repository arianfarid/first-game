use bevy::prelude::*;
use bevy::window::PrimaryWindow;


const USER_SPEED: f32 = 200.0;


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    // GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
     
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, move_user)
        .add_systems(Update, rotate_user)
        .run();
}
struct Cell {
    width: u32,
    height: u32,
}
/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct Player;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Cell>>,
    score: i32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn((SpriteBundle {
            texture: asset_server.load("spacebg.png"),
            ..default()
        }
    );
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ship.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Player,
    ));
}

fn move_user(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
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

    // Calculate the new horizontal paddle position based on player input
    let new_player_position_x =
        player_transform.translation.x + x_direction * USER_SPEED * time.delta_seconds();
    let new_player_position_y =
        player_transform.translation.y + y_direction * USER_SPEED * time.delta_seconds();

    player_transform.translation.x = new_player_position_x
    //.clamp(left_bound, right_bound)
    ;
    player_transform.translation.y = new_player_position_y
    //.clamp(left_bound, right_bound)
    ;
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
        let to_mouse = (player_translation - v).normalize();
        let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));
        player_transform.rotation = rotate_to_mouse;
       },
       None => {},
    }

}
