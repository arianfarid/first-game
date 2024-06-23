use bevy::prelude::*;
mod basic_enemy;
mod player;
pub mod beam;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Playing,
    // GameOver,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, basic_enemy::BasicEnemyPlugin, player::PlayerPlugin, beam::BeamPlugin))
        .init_resource::<Game>()
        .add_systems(OnEnter(GameState::Playing), setup_camera)
        .init_state::<GameState>()
        .run();
}
struct Cell {
    width: u32,
    height: u32,
}
/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;


fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}


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
