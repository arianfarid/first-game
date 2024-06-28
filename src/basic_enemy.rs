use bevy::{app::{App, Plugin}, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

use crate::{beam::Beam, GameLevel, GameState};

const ENEMY_SPEED: f32 = 400.;

pub struct BasicEnemyPlugin;

impl Plugin for BasicEnemyPlugin {
    fn build(&self, app: &mut App) {
       app
       .insert_resource(ShootTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
       .add_event::<CollisionEvent>()
       .add_event::<ExplosionEvent>()
       .add_systems(OnEnter(GameLevel::SpaceOne), setup)
       .add_systems(Update, (move_enemy, enemy_fire, animate_beams, check_collision).chain().run_if(in_state(GameState::Playing)))
       .add_systems(FixedUpdate, (animate_explosion))
       ;

    }
}

#[derive(Component)]
struct BasicEnemy {
    direction: f32,
    health: f32,
    state: EnemyState

}
enum EnemyState {
    Active,
    Dead,
}
#[derive(Event, Default)]
struct CollisionEvent;
#[derive(Component)]
struct Collider;
#[derive(Resource)]
struct ShootTimer(Timer);


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(0., 300., 0.),
            ..default()
        },
        BasicEnemy { state: EnemyState::Active, direction : 1., health: 100.}
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(60., 300., 0.),
            ..default()
        },
        BasicEnemy { state: EnemyState::Active, direction : 1., health: 100.}
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(-60., 300., 0.),
            ..default()
        },
        BasicEnemy { state: EnemyState::Active, direction : 1., health: 100.}
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
    for (enemy, transform) in query.iter_mut() {
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

fn check_collision(
    mut enemy_query: Query<(Entity, &Transform, &mut BasicEnemy), With<BasicEnemy>>,
    mut beam_query: Query<(Entity, &Transform, &Beam), With<Beam>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands : Commands,
) {
    for (mut e_entity, e_transform, mut e_enemy) in enemy_query.iter_mut() {
        match e_enemy.state {
            EnemyState::Active => {
                let ecircle = 
                    BoundingCircle::new(
                        e_transform.translation.truncate(),
                        10.);
                for (b_entity, b_transform, beam) in beam_query.iter_mut()  {
                    let b_box = 
                        Aabb2d::new(b_transform.translation.truncate(), b_transform.scale.truncate() / 2.);
                    if ecircle.intersects(&b_box) {
                        collision_events.send_default();
                        e_enemy.health -= beam.power;
                        commands.entity(b_entity).despawn();
                    }
                }
                if (e_enemy.health < 1.) {
                    e_enemy.state = EnemyState::Dead;
                    explosion_events.send_default(); //sound?
                    let texture = asset_server.load("test_explosion.png");
                    let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 5, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 4 };
                    commands.entity(e_entity).despawn();
                    commands.spawn((
                        SpriteSheetBundle {
                            texture,
                            atlas: TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                            transform: Transform::from_xyz(e_transform.translation.x, e_transform.translation.y, 2.),
                            ..default()
                        },
                        Explosion,
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
                    ));
                }
            }
            EnemyState::Dead => {
                commands.entity(e_entity).despawn();
            }
        }
        
    }
}

#[derive(Event, Default, Debug)]
struct ExplosionEvent;
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}
#[derive(Component)]
struct Explosion;

fn animate_explosion(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas, Entity), With<Explosion>>,
) {
    for (indices, 
        mut timer, 
        mut atlas, 
        entity) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if atlas.index == indices.last {
                commands.entity(entity).despawn();
            }
            else {
                atlas.index += 1;
            };
        }
    }
}