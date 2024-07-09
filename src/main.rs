use bevy::prelude::*;
mod basic_enemy;
mod basic_enemy_move_patterns;
mod camera;
mod canon;
mod collision_core;
mod player;
mod level;
pub mod beam;
mod level_background;
pub mod Player;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    // GameOver,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameLevel {
    #[default]
    SpaceOne,
    SpaceTwo,
}

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins, 
                camera::GameCameraPlugin,
                collision_core::CollisionCorePlugin,
                level::LevelPlugin,
                level_background::LevelBackgroundPlugin,
                basic_enemy::BasicEnemyPlugin, 
                player::PlayerPlugin, 
                beam::BeamPlugin,
         ))
        .init_resource::<Game>()
        .init_state::<GameState>()
        .init_state::<GameLevel>()
        .run();
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
    score: i32,
}
