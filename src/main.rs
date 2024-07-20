use bevy::{prelude::*, window::WindowResolution};
mod basic_enemy;
mod basic_enemy_move_patterns;
mod enemy_core;
pub mod camera;
mod canon;
mod collision_core;
pub mod explosion_core;
pub mod player;
mod level;
pub mod beam;
pub mod shoot_patterns;
mod level_background;

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
                DefaultPlugins.set(
                    // here we configure the main window
                    WindowPlugin {
                        primary_window: Some(Window {
                            resolution: WindowResolution::new(1200.0, 800.0),
                            // ...
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                ), 
                camera::GameCameraPlugin,
                collision_core::CollisionCorePlugin,
                enemy_core::EnemyCorePlugin,
                explosion_core::ExplosionCorePlugin,
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
