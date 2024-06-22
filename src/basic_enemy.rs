use bevy::{app::{App, Plugin}, prelude::*};

use crate::GameState;

const ENEMY_SPEED: f32 = 400.;

pub struct BasicEnemyPlugin;

impl Plugin for BasicEnemyPlugin {
    fn build(&self, app: &mut App) {
       app
       .insert_resource(ShootTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
       .add_systems(OnEnter(GameState::Playing), setup)
       .add_systems(Update, (move_enemy, enemy_fire, animate_beams).chain());

    }
}

#[derive(Component)]
struct BasicEnemy {
    direction: f32
}
#[derive(Resource)]
struct ShootTimer(Timer);


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(0., 300., 0.),
            ..default()
        },
        BasicEnemy { direction : 1.}
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(60., 300., 0.),
            ..default()
        },
        BasicEnemy { direction : 1.}
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(-60., 300., 0.),
            ..default()
        },
        BasicEnemy { direction : 1.}
    ));
}

fn move_enemy(
    mut query: Query<(&mut BasicEnemy, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut enemy, mut transform) in query.iter_mut() {
        //simply flip direction depending on bounds
        if transform.translation.x >= 600. {
            enemy.direction = -1.
        } else if transform.translation.x <= -600. {
            enemy.direction = 1.
        }

        //now move enemy
        let new_pos = 
            transform.translation.x + (ENEMY_SPEED * enemy.direction) * time.delta_seconds();
        transform.translation.x = new_pos;
            
    }
}

fn enemy_fire(
    mut query: Query<(&mut BasicEnemy, &mut Transform)>,
    time: Res<Time>,
    mut timer: ResMut<ShootTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for (enemy, mut transform) in query.iter_mut() {
        // Shoot every N seconds
        if timer.0.tick(time.delta()).just_finished() {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("beam_basic.png"),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        0.),
                    ..default()
                },
                BasicBeam { direction : -1.}
            ));
        }
    }
}

#[derive(Component)]
pub struct BasicBeam {
    direction: f32,
}
const BEAM_SPEED: f32 = 250.;
fn animate_beams(
    mut query: Query<(&mut BasicBeam, &mut Transform)>,
    time: Res<Time>,
) {
    for (beam, mut transform) in query.iter_mut() {
        let new_y = transform.translation.y + (beam.direction * BEAM_SPEED * time.delta_seconds());
        transform.translation.y = new_y;
    }
}