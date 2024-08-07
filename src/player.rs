
use bevy::{app::{App, Plugin}, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};
use bevy::window::PrimaryWindow;
use crate::{
    basic_enemy::EnemyFire, 
    beam::{Beam, BeamType}, 
    camera::{MainCamera, CameraShakeEvent}, 
    canon::CanonPlugin, 
    collision_core::CollisionEvent, 
    explosion_core::ExplosionEvent,
    GameLevel, 
    GameState
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
       app
       .add_systems(OnEnter(GameLevel::SpaceOne), setup)
       .add_systems(OnEnter(PlayerState::Dead), destroy_player)
       .init_state::<PlayerState>()
       .add_plugins(CanonPlugin)
       .add_systems(Update, (toggle_pause, despawn))
       .add_systems(
            Update, 
            (toggle_pause, move_user, check_collision, rotate_user, user_fire_beam)
                .chain()
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(PlayerState::Spawned))
        );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum PlayerState {
    #[default]
    Setup,
    Spawned,
    Dead,
}
#[derive(Component, Resource)]
pub struct DespawnTimer(Timer);

#[derive(Component, Debug)]
pub struct Player {
    health: f32,
    shield: f32,
    pub front_weapon: WeaponType,
    pub front_weapon_beam_type: BeamType,
    pub front_weapon_needs_cooldown: bool,
    pub left_weapon: WeaponType,
    pub right_weapon: WeaponType,
}
impl Default for Player {
    fn default() -> Player {
        Player {
            health: 100.,
            shield: 100.,
            front_weapon: WeaponType::WaveGun,
            front_weapon_beam_type: BeamType::Wave,
            front_weapon_needs_cooldown: false,
            left_weapon: WeaponType::PlasmaCanon,
            right_weapon: WeaponType::PlasmaCanon,
        }
    }
}

#[derive(Component, Debug, Default, PartialEq)]
pub enum WeaponType {
    #[default]
    None,
    WaveGun,
    PlasmaCanon
}

#[derive(Component, Resource)]
pub struct FrontWeaponTimer(Timer);

#[derive(Component)]
pub struct WaveGun {
    lockout_time: f32,
}
impl WaveGun {
    fn new() -> Self {
        WaveGun {
            lockout_time: 0.01,
        }
    }
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32
}

#[derive(Component, Debug)]
struct Acceleration {
    x: f32,
    y: f32
}
pub const USER_SPEED: f32 = 300.0;

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut player_state: ResMut<NextState<PlayerState>>
) {
    let player = Player { ..Default::default() };
    let weapon = match player.front_weapon {
        WeaponType::WaveGun => WaveGun::new(),
        WeaponType::PlasmaCanon => WaveGun::new(),
        WeaponType::None => WaveGun::new()
    };
    let weapon_lockout = weapon.lockout_time;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ship.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        player,
        weapon,
        Velocity {x: 0., y: 0.},
        Acceleration {x: 0., y: 0.},
        FrontWeaponTimer(Timer::from_seconds(weapon_lockout, TimerMode::Once)),
    ));
    player_state.set(PlayerState::Spawned);
}

fn move_user(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    // Todo: map to an optional user input
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        x_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        x_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        y_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        y_direction -= 1.0;
    }

    let new_player_position_x =
        player_transform.translation.x + x_direction * USER_SPEED * time.delta_seconds();
    let new_player_position_y =
        player_transform.translation.y + y_direction * USER_SPEED * time.delta_seconds();

    player_transform.translation.x = new_player_position_x;
    player_transform.translation.y = new_player_position_y;
}

fn rotate_user(
    mut player_query: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) { 
    let (camera, camera_transform) = q_camera.single();
    let mut player_transform = player_query.single_mut();
    let player_translation = player_transform.translation.xy();

    let cursor_pos = q_windows.single().cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate());
    match cursor_pos {
       Some(v) => {
        let to_mouse = (v - player_translation).normalize();
        let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));
        player_transform.rotation = rotate_to_mouse;
       },
       None => {},
    }
}

fn user_fire_beam(
    mut player_query: Query< (&mut Transform, &mut Player, &mut FrontWeaponTimer), With<Player>>,
    mouse_buttons:Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (player_transform, mut player, mut front_weapon_timer) = player_query.single_mut();
    if front_weapon_timer.0.tick(time.delta()).finished() {
        player.front_weapon_needs_cooldown = false;
    }
    match player.front_weapon_needs_cooldown {
        true => {}
        false => {
            let player_location = player_transform.translation;
            let player_angle = player_transform.rotation;
            let axis = (player_angle * Vec3::Y).xy();
            
            if keyboard_input.pressed(KeyCode::Space) || mouse_buttons.pressed(MouseButton::Left) {
                let mut spawn_transform = Transform::from_scale(Vec3::splat(1.0));
                spawn_transform.translation = player_location;
                spawn_transform.rotation = player_angle;
                front_weapon_timer.0.reset();
                player.front_weapon_needs_cooldown = true;
                let beam_type = &player.front_weapon_beam_type;
                commands.spawn((
                    SpriteBundle {
                        transform: spawn_transform,
                        texture: asset_server.load("wave.png"),
                        ..default()
                    },
                    //todo: 2 weapons, should be enum w/ params
                    Beam::new(beam_type, Vec2::new(axis.x, axis.y))
                ));
            }
        }
    }
}

fn destroy_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, Entity, &mut Sprite), With<Player>>,
    mut explosion_events: EventWriter<ExplosionEvent>,
) {
    let (transform, mut entity, mut sprite) =  player_query.single_mut();
    sprite.color = Color::rgba(1., 1., 1., 0.3);
    explosion_events.send(ExplosionEvent(Transform::from_xyz(transform.translation.x, transform.translation.y, 2.)));
    commands.entity(entity).insert(DespawnTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn despawn(
    mut commands: Commands,
    mut despawn_query: Query<(Entity), With<DespawnTimer>>
) {
    for entity in despawn_query.iter_mut() {
        // commands.entity(entity).despawn_recursive();
    }
}

fn toggle_pause(
    curr_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,

) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if curr_state.get() == &GameState::Playing {
            next_state.set(GameState::Paused)
        } else {
            next_state.set(GameState::Playing)
        }
    }
}

fn check_collision(
    commands: Commands,
    mut player_query: Query<(&Transform, &mut Player, Entity), With<Player>>,
    mut enemy_fire_query: Query<(&Transform, &mut EnemyFire), With<EnemyFire>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut camera_shake_events: EventWriter<CameraShakeEvent>,
    mut player_state: ResMut<NextState<PlayerState>>
) {
    let (player_transform, mut player, entity) = player_query.single_mut();
    for (enemy_fire_transform, fire) in enemy_fire_query.iter_mut() {
        let pcircle = BoundingCircle::new(
            player_transform.translation.truncate(),
            20.
        );
        let b_box = Aabb2d::new(
            enemy_fire_transform.translation.truncate(),
            enemy_fire_transform.scale.truncate() / 2.
        );
        if pcircle.intersects(&b_box) {
            //shield should always be taken first;
            if player.shield >= fire.power {
                player.shield -= fire.power;
            } else if player.shield < fire.power {
                let from_health = fire.power - player.shield;
                //need val to roll over into health
                player.shield = 0.;
                player.health -= from_health;
                collision_events.send(CollisionEvent(entity));
                camera_shake_events.send(CameraShakeEvent);
            } else {
                player.health -= fire.power;
            }
        }
    }
    if player.health <= 0. {
        player_state.set(PlayerState::Dead)
    }
}