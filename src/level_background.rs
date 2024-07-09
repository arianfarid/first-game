use bevy::{asset, prelude::*};

use crate::GameState;

pub struct LevelBackgroundPlugin;
impl Plugin for LevelBackgroundPlugin {
    fn build(&self, app: &mut App) {
       app
       .add_systems(OnEnter(GameState::Playing), load_bg)
       .add_systems(FixedUpdate, (animate_bg).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct BackgroundTile;
//speed per second
const TILE_SPEED: Vec2 = Vec2::new(-50., -100.);
const TILE_X_BOUND: f32 = 1050.;
const TILE_Y_BOUND: f32 = 650.;

fn load_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    //dumb first take
    let mut pointer = Vec2::new(-TILE_X_BOUND, -TILE_Y_BOUND);
    while pointer.y < TILE_Y_BOUND {
        while pointer.x < TILE_X_BOUND {
            render_tile(&mut commands, &asset_server, Transform::from_xyz(pointer.x, pointer.y, -1.));
            pointer.x += 200.;
        }
        //advance y pos
        pointer.y += 200.;
        //reset x
        pointer.x = -TILE_X_BOUND;
    }
}

fn animate_bg(
    mut commands: Commands, asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query: Query<(&mut BackgroundTile, &mut Transform, Entity)>
) {
    for (_tile, mut transform, entity) in query.iter_mut() {
        // //despawn tile if out out of view
        // //using hard-coded dims now, but should query window size or set hard window size
        if transform.translation.y <= -TILE_Y_BOUND {
            commands.entity(entity).despawn();
            render_tile(&mut commands, &asset_server, Transform::from_xyz(transform.translation.x, TILE_Y_BOUND, -1.));
        }
        if transform.translation.x <= -TILE_X_BOUND {
            commands.entity(entity).despawn();
            render_tile(&mut commands, &asset_server, Transform::from_xyz(TILE_X_BOUND, transform.translation.y, -1.));
        }
        transform.translation +=  (TILE_SPEED * time.delta_seconds()).extend(-1.);
        transform.translation.x = transform.translation.x.floor();
        transform.translation.y = transform.translation.y.floor();
    }
}

fn render_tile(commands: &mut Commands, asset_server: &Res<AssetServer>, transform: Transform) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("spacebg.png"),
        transform: transform,
        ..default()
    },BackgroundTile));
}