use bevy::{app::{App, Plugin}, prelude::*};

use crate::GameLevel;
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameLevel::SpaceOne), setup_camera)
        ;
    }
}
/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}