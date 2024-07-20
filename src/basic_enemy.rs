use bevy::{app::{App, Plugin}, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

use crate::{basic_enemy_move_patterns::EnemyMovePattern, beam::{Beam, BeamType, ShootType}, collision_core::CollisionEvent, enemy_core::{EnemyCore, EnemyState, ShootTimer}, explosion_core::ExplosionEvent, level::Wave, player::Player, GameState};

const ENEMY_SPEED: f32 = 400.;

pub struct BasicEnemyPlugin;
impl Plugin for BasicEnemyPlugin {
    fn build(&self, app: &mut App) {
       app
       .insert_resource(ShootTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
       .add_systems(Update, 
        (move_enemy, enemy_fire, animate_beams, check_collision)
                .chain()
                .run_if(in_state(GameState::Playing))
        )
       ;

    }
}

#[derive(Component)]
pub struct BasicEnemy {
    pub x_direction: f32,
    pub y_direction: f32,
    health: f32,
    move_pattern: EnemyMovePattern,
    state: EnemyState,
    shoot: bool,
}
#[derive(Component)]
pub struct EnemyFire {
    pub power: f32,
}
impl BasicEnemy {
    pub fn new(move_pattern: EnemyMovePattern) -> Self {
        BasicEnemy { 
            state: EnemyState::Active, 
            x_direction : 1.,
            y_direction : 0., 
            health: 100.,
            move_pattern: move_pattern,
            shoot: true,
        }
    }
    pub fn shoot(mut self, shoot:bool) -> Self {
        self.shoot = shoot;
        self
    }
    pub fn health(mut self, health: f32) -> Self {
        self.health = health;
        self
    }
}

fn move_enemy(
    mut query: Query<(&mut EnemyCore, &mut Transform, Entity)>,
    player: Query<&Transform, (With<Player>, Without<EnemyCore>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let player_translation_x =  match player.get_single() {
        Ok(p) => p.translation.x,
        _ => 0.,
    };
    for (mut enemy, mut transform, entity) in query.iter_mut() {
        match enemy.move_pattern {
            EnemyMovePattern::Basic => {
                //simply flip direction depending on bounds
                if transform.translation.x >= 600. {
                    enemy.x_direction = -1.
                } else if transform.translation.x <= -600. {
                    enemy.x_direction = 1.
                }
                //now move enemy
                let new_x_pos = 
                    transform.translation.x + (ENEMY_SPEED * enemy.x_direction) * time.delta_seconds();
                transform.translation.x = new_x_pos;
            }
            EnemyMovePattern::Down => {
                enemy.x_direction = 0.;
                enemy.y_direction = -1.;
                let new_y_pos = 
                transform.translation.y + (ENEMY_SPEED * enemy.y_direction) * time.delta_seconds();
                transform.translation.y = new_y_pos;

                // Track player
                let dir = player_translation_x - transform.translation.x;
                let dir = dir / dir.abs();
                enemy.x_direction = dir;
                let new_x_pos = 
                    transform.translation.x + (ENEMY_SPEED / 2. * enemy.x_direction) * time.delta_seconds();
                transform.translation.x = new_x_pos;

                if new_y_pos < -100. {
                    commands.entity(entity).despawn();
                }
            }
            EnemyMovePattern::StartShootGo => {

            }
        }

        
            
    }
}

fn enemy_fire(
    mut query: Query<(&mut EnemyCore, &mut Transform)>,
    time: Res<Time>,
    mut timer: ResMut<ShootTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for (mut enemy, transform) in query.iter_mut() {
        match enemy.shoot_type.to_owned() {
            crate::beam::ShootType::Basic => {
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
                        BasicBeam {speed: BEAM_SPEED,  y : -1., x: 0. },
                        EnemyFire { power: 20. },
                    ));
                }
            },
            ShootType::TestHell(mut shoot_pattern) => {
                if enemy.shoot_timer.0.tick(time.delta()).just_finished() {
                    for beam in shoot_pattern.beam.iter_mut() {
                        let texture_path = match beam.beam_type {
                            BeamType::FireBall => {
                                "beam_fireball.png"
                            }
                            _ => {
                                "beam_basic.png"
                            }
                        };
                        commands.spawn((
                            SpriteBundle {
                                texture: asset_server.load(texture_path),
                                transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 10.),
                                ..Default::default()
                            },
                            BasicBeam {speed: beam.speed, y: beam.direction.y, x: beam.direction.x },
                            EnemyFire { power: beam.power }
                        ));
                    }
                }
            },
        }
    }
}

#[derive(Component)]
pub struct BasicBeam {
    x: f32,
    y: f32,
    speed: f32,
}
pub const BEAM_SPEED: f32 = 250.;
fn animate_beams(
    mut query: Query<(&mut BasicBeam, &mut Transform)>,
    time: Res<Time>,
) {
    for (beam, mut transform) in query.iter_mut() {
        let new_y = transform.translation.y + (beam.y * beam.speed * time.delta_seconds());
        transform.translation.y = new_y;
        let new_x = transform.translation.x + (beam.x * beam.speed * time.delta_seconds());
        transform.translation.x = new_x;
    }
}

fn check_collision(
    mut enemy_query: Query<(&Wave, Entity, &Transform, &mut EnemyCore), With<EnemyCore>>,
    mut beam_query: Query<(Entity, &Transform, &Beam), With<Beam>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut commands : Commands,
) {

    for (wave, e_entity, e_transform, mut e_enemy) in enemy_query.iter_mut() {
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
                        collision_events.send(CollisionEvent(e_entity));
                        e_enemy.health -= beam.power;
                        commands.entity(b_entity).despawn();
                    }
                }
                if e_enemy.health < 1. {
                    e_enemy.state = EnemyState::Dead;
                    let explosion_transform = Transform::from_xyz(e_transform.translation.x, e_transform.translation.y, 2.);
                    explosion_events.send(ExplosionEvent(explosion_transform));
                    commands.entity(e_entity).despawn();
                }
            }
            EnemyState::Dead => {
                commands.entity(e_entity).despawn();
            }
        }
        
    }
}