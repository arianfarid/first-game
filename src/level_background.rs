use bevy::prelude::*;

use crate::GameState;

pub struct LevelBackgroundPlugin;
impl Plugin for LevelBackgroundPlugin {
    fn build(&self, app: &mut App) {
       app
       .add_systems(OnEnter(GameState::Playing), load_bg);
    }
}

fn load_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    //dumb first take
    let mut pointer = Vec2::new(-650., -450.);
    while pointer.y < 450. {
        while pointer.x < 650. {
            commands.spawn((SpriteBundle {
                texture: asset_server.load("spacebg.png"),
                transform: Transform::from_xyz(pointer.x, pointer.y, -1.),
                ..default()
            }));
            pointer.x += 200.;
        }
        //advance y pos
        pointer.y += 200.;
        //reset x
        pointer.x = -650.;
    }
}