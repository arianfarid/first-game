use bevy::{app::{App, Plugin}, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

use crate::{basic_enemy_move_patterns::EnemyMovePattern, beam::Beam, collision_core::CollisionEvent, explosion_core::ExplosionEvent, player::Player, GameState};

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
enum EnemyState {
    Active,
    Dead,
}


#[derive(Resource)]
struct ShootTimer(Timer);

fn move_enemy(
    mut query: Query<(&mut BasicEnemy, &mut Transform, Entity)>,
    player: Query<&Transform, (With<Player>, Without<BasicEnemy>)>,
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
                BasicBeam { direction : -1.},
                EnemyFire { power: 20. },
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
    explosion_events: EventWriter<ExplosionEvent>,
    mut commands : Commands,
) {

    for (e_entity, e_transform, mut e_enemy) in enemy_query.iter_mut() {
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
                        // commands.entity(entity).insert(Blinking(Timer::from_seconds(BLINK_DURATION, TimerMode::Once)));
                        commands.entity(b_entity).despawn();
                    }
                }
                if e_enemy.health < 1. {
                    e_enemy.state = EnemyState::Dead;
                    let mut explosion_transform = Transform::from_xyz(e_transform.translation.x, e_transform.translation.y, 2.);
                    explosion_transform.scale = Vec3::new(2., 2., 2.);
                    // explosion_events.send(ExplosionEvent(explosion_transform)); //sound?
                    commands.entity(e_entity).despawn();
                }
            }
            EnemyState::Dead => {
                commands.entity(e_entity).despawn();
            }
        }
        
    }
}